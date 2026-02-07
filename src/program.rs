// SPDX-License-Identifier: GPL-3.0-only
// Program execution engine

use crate::context::Context;
use crate::error::{Error, Result};
use crate::lexer::Lexer;
use crate::object::Object;
use crate::parser::Parser;

/// Execute a program string
pub fn execute(source: &str, ctx: &mut Context) -> Result<()> {
    // Lexical analysis
    let tokens = Lexer::analyze(source)?;

    // Parse to objects
    let mut objects = Parser::build_objects(tokens)?;

    // Preprocess: link control flow
    link_control_flow(&mut objects)?;

    // Execute
    run_objects(&objects, ctx)
}

/// Execute a vector of objects
pub fn run_objects(objects: &[Object], ctx: &mut Context) -> Result<()> {
    use crate::operations::branch::{RT_ERROR, STEP_OUT};

    // Make a mutable copy of objects for updating branch args during execution
    let mut objects = objects.to_vec();
    let mut ip = 0; // Instruction pointer

    while ip < objects.len() {
        // Clone necessary data to avoid borrow conflicts
        let obj = objects[ip].clone();

        match obj {
            // Keyword: call handler function
            Object::Keyword { handler, .. } => {
                handler(ctx)?;
                ip += 1;
            }

            // Branch: call branch handler (control flow)
            Object::Branch {
                handler,
                args,
                name,
            } => {
                let mut args_mut = args;

                // Special handling for ELSE: copy condition from IF (which was set by THEN)
                if name == "else" {
                    let if_index = args.arg3;
                    if let Some(Object::Branch { args: if_args, .. }) = objects.get(if_index) {
                        args_mut.condition = if_args.condition;
                    }
                }

                // Special handling for NEXT/STEP: copy loop bounds from FOR/START on first iteration
                if (name == "next" || name == "step") && !args_mut.arg_bool {
                    if let Some(Object::Branch { args: for_args, .. }) = objects.get(args_mut.arg1)
                    {
                        args_mut.first_index = for_args.first_index;
                        args_mut.last_index = for_args.last_index;
                    }
                }

                let next_ip = handler(ctx, &mut args_mut)?;

                // Special handling for THEN: store condition back to IF so ELSE can read it
                if name == "then" {
                    let if_index = args.arg3;
                    if let Some(Object::Branch { args: if_args, .. }) = objects.get_mut(if_index) {
                        if_args.condition = args_mut.condition;
                    }
                }

                // Special handling for FOR: initialize loop variable
                if name == "for" && args_mut.arg1 != STEP_OUT {
                    // Get loop variable symbol
                    if let Some(Object::Symbol { name: var_name, .. }) = objects.get(args_mut.arg1)
                    {
                        // Store initial value in local heap
                        use rug::Float;
                        let initial_value = Object::Number {
                            value: Float::with_val(ctx.config.precision_bits, args_mut.first_index),
                            base: 10,
                        };
                        ctx.local_heap.store(var_name.clone(), initial_value);
                    }
                }

                // Special handling for NEXT/STEP: update loop variable
                if (name == "next" || name == "step") && args_mut.arg1 != STEP_OUT {
                    // Get FOR/START index
                    if let Some(Object::Branch { args: for_args, .. }) = objects.get(args_mut.arg1)
                    {
                        if for_args.arg1 != STEP_OUT {
                            // This is a FOR loop (not START)
                            if let Some(Object::Symbol { name: var_name, .. }) =
                                objects.get(for_args.arg1)
                            {
                                // Update loop variable in local heap
                                // For STEP: compute exact value to avoid accumulation errors
                                // For NEXT: compute from iteration count
                                use rug::Float;
                                let new_value = if name == "step" {
                                    // STEP: Compute with MPFR precision from stored step value
                                    let step_key = format!("__step_{}", args_mut.arg1);
                                    if let Some(Object::Number {
                                        value: step_mpfr, ..
                                    }) = ctx.local_heap.recall(&step_key)
                                    {
                                        // Compute: start + (iteration * step) using MPFR
                                        let start = Float::with_val(
                                            ctx.config.precision_bits,
                                            args_mut.first_index,
                                        );
                                        let iteration = Float::with_val(
                                            ctx.config.precision_bits,
                                            args_mut.iteration_count,
                                        );
                                        let current = start + (iteration * step_mpfr);
                                        Object::Number {
                                            value: current,
                                            base: 10,
                                        }
                                    } else {
                                        // Fallback (shouldn't happen)
                                        let current_value = args_mut.first_index
                                            + (args_mut.iteration_count as f64
                                                * args_mut.step_value);
                                        Object::Number {
                                            value: Float::with_val(
                                                ctx.config.precision_bits,
                                                current_value,
                                            ),
                                            base: 10,
                                        }
                                    }
                                } else {
                                    // NEXT: simple increment by 1
                                    let current_value =
                                        args_mut.first_index + (args_mut.iteration_count as f64);
                                    Object::Number {
                                        value: Float::with_val(
                                            ctx.config.precision_bits,
                                            current_value,
                                        ),
                                        base: 10,
                                    }
                                };
                                ctx.local_heap.store(var_name.clone(), new_value);
                            }
                        }
                    }
                }

                // Update args in objects array (for stateful branches like FOR/NEXT)
                if let Object::Branch { args: obj_args, .. } = &mut objects[ip] {
                    *obj_args = args_mut;
                }

                match next_ip {
                    STEP_OUT => {
                        // Clean up loop variable when NEXT/STEP exits
                        if name == "next" || name == "step" {
                            // Get FOR/START instruction
                            if let Some(Object::Branch { args: for_args, .. }) =
                                objects.get(args_mut.arg1)
                            {
                                // Only FOR loops have loop variables (arg1 != STEP_OUT)
                                if for_args.arg1 != STEP_OUT {
                                    if let Some(Object::Symbol { name: var_name, .. }) =
                                        objects.get(for_args.arg1)
                                    {
                                        ctx.local_heap.purge(var_name);
                                    }
                                }
                            }
                        }
                        ip += 1; // Continue to next instruction
                    }
                    RT_ERROR => {
                        return Err(Error::RuntimeError("branch execution error".to_string()));
                    }
                    _ => {
                        // Special handling for NEXT/STEP: jump to FOR's arg1 + 1
                        if name == "next" || name == "step" {
                            // next_ip is the FOR/START index
                            if let Some(Object::Branch { args: for_args, .. }) =
                                objects.get(next_ip)
                            {
                                if for_args.arg1 != STEP_OUT {
                                    // FOR loop: jump to symbol + 1 (start of loop body)
                                    ip = for_args.arg1 + 1;
                                } else {
                                    // START loop: jump to START + 1
                                    ip = next_ip + 1;
                                }
                            } else {
                                ip = next_ip; // Fallback
                            }
                        } else {
                            ip = next_ip; // Normal jump
                        }
                    }
                }
            }

            // Symbol with auto_eval: recall variable
            Object::Symbol {
                name,
                auto_eval: true,
            } => {
                auto_recall(ctx, &name)?;
                ip += 1;
            }

            // Program marked for auto-evaluation: evaluate it inline
            Object::Program(ref program) if program.auto_eval => {
                // This is a program with local variable binding
                // Execute it inline rather than pushing it

                // Execute prefix code
                if !program.prefix_objects.is_empty() {
                    run_objects(&program.prefix_objects, ctx)?;
                }

                // Bind local variables if present
                if !program.local_vars.is_empty() {
                    let num_vars = program.local_vars.len();
                    if ctx.stack.len() < num_vars {
                        return Err(Error::MissingOperand);
                    }

                    let mut values = Vec::new();
                    for _ in 0..num_vars {
                        values.push(ctx.stack.pop().unwrap());
                    }
                    values.reverse();

                    for (var_name, value) in program.local_vars.iter().zip(values.iter()) {
                        ctx.local_heap.store(var_name.clone(), value.clone());
                    }

                    // Execute body
                    run_objects(&program.objects, ctx)?;

                    // Clean up local variables
                    for var_name in &program.local_vars {
                        ctx.local_heap.purge(var_name);
                    }
                } else {
                    // No local vars, just execute body
                    run_objects(&program.objects, ctx)?;
                }

                ip += 1;
            }

            // Everything else: push to stack
            _ => {
                ctx.stack.push(obj);
                ip += 1;
            }
        }
    }

    Ok(())
}

/// Auto-recall a variable and push it to the stack
/// If variable doesn't exist, push the symbol itself
/// If the variable contains a Program, execute it (standard RPL behavior)
fn auto_recall(ctx: &mut Context, name: &str) -> Result<()> {
    if let Some(obj) = ctx.find_variable(name) {
        // Clone the object to avoid borrow issues
        let obj = obj.clone();

        // Variable exists
        match obj {
            Object::Program(program) => {
                // Programs are auto-executed when recalled (standard RPL behavior)
                // Execute prefix code if present
                if !program.prefix_objects.is_empty() {
                    run_objects(&program.prefix_objects, ctx)?;
                }

                // Handle local variables if present
                if !program.local_vars.is_empty() {
                    let num_vars = program.local_vars.len();
                    if ctx.stack.len() < num_vars {
                        return Err(Error::MissingOperand);
                    }

                    let mut values = Vec::new();
                    for _ in 0..num_vars {
                        values.push(ctx.stack.pop().unwrap());
                    }
                    values.reverse();

                    for (var_name, value) in program.local_vars.iter().zip(values.iter()) {
                        ctx.local_heap.store(var_name.clone(), value.clone());
                    }

                    // Execute body
                    run_objects(&program.objects, ctx)?;

                    // Clean up local variables
                    for var_name in &program.local_vars {
                        ctx.local_heap.purge(var_name);
                    }
                } else {
                    // No local vars, just execute body
                    run_objects(&program.objects, ctx)?;
                }

                Ok(())
            }
            _ => {
                // Not a program, just push the value
                ctx.stack.push(obj);
                Ok(())
            }
        }
    } else {
        // Variable doesn't exist, push the symbol itself
        ctx.stack.push(Object::Symbol {
            name: name.to_string(),
            auto_eval: false,
        });
        Ok(())
    }
}

/// Control flow preprocessing: link branch instructions
pub fn link_control_flow(objects: &mut [Object]) -> Result<()> {
    use crate::operations::branch::STEP_OUT;

    // Layout structure for tracking IF/DO/WHILE contexts
    #[derive(Debug, Clone)]
    #[allow(dead_code)]
    struct IfLayout {
        index_if_or_do_or_while: usize,
        index_then_or_until_or_repeat: Option<usize>,
        index_else: Option<usize>,
        index_end: Option<usize>,
        is_do_until: bool,
        is_while_repeat: bool,
    }

    impl IfLayout {
        fn new(index: usize) -> Self {
            Self {
                index_if_or_do_or_while: index,
                index_then_or_until_or_repeat: None,
                index_else: None,
                index_end: None,
                is_do_until: false,
                is_while_repeat: false,
            }
        }
    }

    let mut vlayout: Vec<IfLayout> = Vec::new(); // Stack of if/do/while contexts
    let mut vstart_index: Vec<usize> = Vec::new(); // Stack of start/for contexts

    for i in 0..objects.len() {
        match &objects[i] {
            Object::Branch { name, .. } => {
                match *name {
                    // IF: create new layout context
                    "if" => {
                        vlayout.push(IfLayout::new(i));
                    }

                    // THEN: link to IF
                    "then" => {
                        if vlayout.is_empty() {
                            return Err(Error::SyntaxError("'then' without 'if'".to_string()));
                        }

                        let layout_index = vlayout.len() - 1;
                        let layout = &mut vlayout[layout_index];

                        if layout.index_then_or_until_or_repeat.is_some() {
                            return Err(Error::SyntaxError("duplicate 'then'".to_string()));
                        }

                        layout.index_then_or_until_or_repeat = Some(i);

                        let if_index = layout.index_if_or_do_or_while;

                        // Update THEN's args
                        if let Object::Branch { args, .. } = &mut objects[i] {
                            args.arg1 = i + 1; // True branch: THEN+1
                            args.arg2 = STEP_OUT; // Will be filled by ELSE or END
                            args.arg3 = if_index; // Back-reference to IF
                        }

                        // Update IF's args (arg2 = address after THEN)
                        if let Object::Branch { args, .. } = &mut objects[if_index] {
                            args.arg2 = i + 1;
                        }
                    }

                    // ELSE: link to IF and THEN
                    "else" => {
                        if vlayout.is_empty() {
                            return Err(Error::SyntaxError("'else' without 'if'".to_string()));
                        }

                        let layout_index = vlayout.len() - 1;
                        let layout = &mut vlayout[layout_index];

                        if layout.index_then_or_until_or_repeat.is_none() {
                            return Err(Error::SyntaxError("'else' without 'then'".to_string()));
                        }

                        if layout.index_else.is_some() {
                            return Err(Error::SyntaxError("duplicate 'else'".to_string()));
                        }

                        layout.index_else = Some(i);

                        let if_index = layout.index_if_or_do_or_while;
                        let then_index = layout.index_then_or_until_or_repeat.unwrap();

                        // Update ELSE's args
                        if let Object::Branch { args, .. } = &mut objects[i] {
                            args.arg1 = i + 1; // False branch: ELSE+1
                            args.arg2 = STEP_OUT; // Will be filled by END
                            args.arg3 = if_index;
                        }

                        // Update THEN's arg2 to skip to ELSE
                        if let Object::Branch { args, .. } = &mut objects[then_index] {
                            args.arg2 = i + 1; // THEN's false branch: ELSE+1
                        }
                    }

                    // END: complete IF/DO/WHILE context
                    "end" => {
                        if vlayout.is_empty() {
                            return Err(Error::SyntaxError(
                                "'end' without matching control structure".to_string(),
                            ));
                        }

                        let layout = vlayout.pop().unwrap();

                        // Handle DO..UNTIL..END
                        if layout.is_do_until {
                            if layout.index_then_or_until_or_repeat.is_none() {
                                return Err(Error::SyntaxError("'do' without 'until'".to_string()));
                            }
                            // END needs to jump back to DO if condition is false
                            if let Object::Branch { args, .. } = &mut objects[i] {
                                args.arg1 = layout.index_if_or_do_or_while + 1;
                                // Jump to DO+1
                            }
                        }
                        // Handle WHILE..REPEAT..END
                        else if layout.is_while_repeat {
                            if layout.index_then_or_until_or_repeat.is_none() {
                                return Err(Error::SyntaxError(
                                    "'while' without 'repeat'".to_string(),
                                ));
                            }
                            let repeat_index = layout.index_then_or_until_or_repeat.unwrap();
                            let while_index = layout.index_if_or_do_or_while;

                            // END jumps back to WHILE
                            if let Object::Branch { args, .. } = &mut objects[i] {
                                args.arg2 = while_index + 1; // Jump to WHILE+1
                            }

                            // REPEAT's arg1 is exit point (END+1)
                            if let Object::Branch { args, .. } = &mut objects[repeat_index] {
                                args.arg1 = i + 1; // Exit to END+1
                            }
                        }
                        // Handle IF..THEN..[ELSE]..END
                        else {
                            if layout.index_then_or_until_or_repeat.is_none() {
                                return Err(Error::SyntaxError("'if' without 'then'".to_string()));
                            }
                            let then_index = layout.index_then_or_until_or_repeat.unwrap();
                            if let Some(else_index) = layout.index_else {
                                // Update ELSE's arg2 to point to END
                                if let Object::Branch { args, .. } = &mut objects[else_index] {
                                    args.arg2 = i; // ELSE exit: END
                                }
                            } else {
                                // No ELSE: update THEN's arg2 to point to END
                                if let Object::Branch { args, .. } = &mut objects[then_index] {
                                    args.arg2 = i; // THEN's false branch: END
                                }
                            }
                        }
                    }

                    // DO: create new layout context for DO..UNTIL
                    "do" => {
                        let mut layout = IfLayout::new(i);
                        layout.is_do_until = true;
                        vlayout.push(layout);
                    }

                    // UNTIL: mark in layout
                    "until" => {
                        if vlayout.is_empty() || !vlayout.last().unwrap().is_do_until {
                            return Err(Error::SyntaxError("'until' without 'do'".to_string()));
                        }

                        let layout_index = vlayout.len() - 1;

                        if vlayout[layout_index]
                            .index_then_or_until_or_repeat
                            .is_some()
                        {
                            return Err(Error::SyntaxError("duplicate 'until'".to_string()));
                        }

                        vlayout[layout_index].index_then_or_until_or_repeat = Some(i);
                    }

                    // WHILE: create new layout context for WHILE..REPEAT
                    "while" => {
                        let mut layout = IfLayout::new(i);
                        layout.is_while_repeat = true;
                        vlayout.push(layout);
                    }

                    // REPEAT: mark in layout
                    "repeat" => {
                        if vlayout.is_empty() || !vlayout.last().unwrap().is_while_repeat {
                            return Err(Error::SyntaxError("'repeat' without 'while'".to_string()));
                        }

                        let layout_index = vlayout.len() - 1;

                        if vlayout[layout_index]
                            .index_then_or_until_or_repeat
                            .is_some()
                        {
                            return Err(Error::SyntaxError("duplicate 'repeat'".to_string()));
                        }

                        vlayout[layout_index].index_then_or_until_or_repeat = Some(i);
                    }

                    // FOR/START: push to start index stack
                    "for" | "start" => {
                        vstart_index.push(i);

                        // Set arg1 based on whether this is FOR or START
                        if *name == "for" {
                            // FOR: arg1 points to loop variable (next instruction should be Symbol)
                            if let Object::Branch { args, .. } = &mut objects[i] {
                                args.arg1 = i + 1;
                            }
                        } else {
                            // START: arg1 = STEP_OUT (no loop variable)
                            if let Object::Branch { args, .. } = &mut objects[i] {
                                args.arg1 = STEP_OUT;
                            }
                        }
                    }

                    // NEXT/STEP: link to FOR/START
                    "next" | "step" => {
                        if vstart_index.is_empty() {
                            return Err(Error::SyntaxError(format!(
                                "'{}' without 'for' or 'start'",
                                name
                            )));
                        }

                        let start_or_for_index = vstart_index.pop().unwrap();

                        // Update NEXT/STEP's arg1 to point back to FOR/START
                        if let Object::Branch { args, .. } = &mut objects[i] {
                            args.arg1 = start_or_for_index;
                        }

                        // Update FOR/START's arg2 to point to NEXT/STEP
                        if let Object::Branch { args, .. } = &mut objects[start_or_for_index] {
                            args.arg2 = i;
                        }
                    }

                    _ => {}
                }
            }
            _ => {}
        }
    }

    // Verify all contexts are closed
    if !vlayout.is_empty() {
        return Err(Error::SyntaxError("unclosed control structure".to_string()));
    }

    if !vstart_index.is_empty() {
        return Err(Error::SyntaxError(
            "unclosed loop (missing 'next' or 'step')".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object::Object;

    #[test]
    fn test_execute_simple() {
        let mut ctx = Context::new();

        // Push two numbers
        execute("2 3", &mut ctx).unwrap();

        assert_eq!(ctx.stack.len(), 2);
    }

    #[test]
    fn test_execute_swap() {
        let mut ctx = Context::new();

        // Push two numbers and swap
        execute("2 3 swap", &mut ctx).unwrap();

        assert_eq!(ctx.stack.len(), 2);

        // After swap: 3 should be on top (index 0), 2 below (index 1)
        match ctx.stack.get(0) {
            Some(Object::Number { value, .. }) => {
                assert_eq!(value.to_f64(), 2.0);
            }
            _ => panic!("Expected number on stack"),
        }
    }

    #[test]
    fn test_execute_dup() {
        let mut ctx = Context::new();

        execute("5 dup", &mut ctx).unwrap();

        assert_eq!(ctx.stack.len(), 2);
    }

    #[test]
    fn test_execute_drop() {
        let mut ctx = Context::new();

        execute("1 2 3 drop", &mut ctx).unwrap();

        assert_eq!(ctx.stack.len(), 2);
    }
}
