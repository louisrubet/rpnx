// SPDX-License-Identifier: GPL-3.0-only
// Branch operations for control flow

use crate::context::Context;
use crate::error::{Error, Result};
use crate::object::{BranchArgs, Object};

// Special return values for branch operations
pub const STEP_OUT: usize = usize::MAX; // Continue to next instruction
pub const RT_ERROR: usize = usize::MAX - 1; // Runtime error

/// IF: if (evaluate condition and store result)
/// Stack: condition -> (empty)
/// Sets condition field to 1 (true) or 0 (false)
pub fn rpnx_if(ctx: &mut Context, args: &mut BranchArgs) -> Result<usize> {
    min_arguments!(ctx, 1);
    arg_must_be!(ctx, 0, Number);

    let condition = pop_one_number!(ctx);

    // Store condition result in condition field
    args.condition = if condition != 0.0 { 1 } else { 0 };

    Ok(STEP_OUT)
}

/// THEN: then (branch based on IF condition)
/// arg1 = address of THEN+1 (true branch)
/// arg2 = address of ELSE+1 or END (false branch)
/// arg3 = address of IF
/// condition = IF's condition result (copied during execution)
pub fn rpnx_then(_ctx: &mut Context, args: &mut BranchArgs) -> Result<usize> {
    if args.condition == 1 {
        Ok(args.arg1) // True: execute THEN block
    } else {
        Ok(args.arg2) // False: skip to ELSE or END
    }
}

/// ELSE: else (alternative branch for IF/THEN)
/// arg1 = address of ELSE+1 (false branch)
/// arg2 = address of END+1 (true exit)
/// arg3 = address of IF
/// condition = IF's condition result (copied during execution)
pub fn rpnx_else(_ctx: &mut Context, args: &mut BranchArgs) -> Result<usize> {
    if args.condition == 1 {
        Ok(args.arg2) // True: skip ELSE, go to END
    } else {
        Ok(args.arg1) // False: execute ELSE block
    }
}

/// END: end (terminate IF/THEN/ELSE, DO/UNTIL, or WHILE/REPEAT)
/// For IF context: just continue
/// For DO/UNTIL: arg1 = DO+1 (loop restart)
/// For WHILE/REPEAT: arg2 = WHILE+1 (test restart)
pub fn rpnx_end(ctx: &mut Context, args: &mut BranchArgs) -> Result<usize> {
    // Check if this is a DO..UNTIL context
    if args.arg1 != STEP_OUT {
        min_arguments!(ctx, 1);
        arg_must_be!(ctx, 0, Number);

        let condition = pop_one_number!(ctx);
        if condition == 0.0 {
            return Ok(args.arg1); // Loop back to DO
        }
    }
    // Check if this is a WHILE..REPEAT context
    else if args.arg2 != STEP_OUT {
        return Ok(args.arg2); // Loop back to WHILE
    }

    Ok(STEP_OUT)
}

/// FOR: for (start counted loop with variable)
/// Stack: <start> <end> -> (empty)
/// arg1 = address of loop variable symbol
/// arg2 = address of NEXT/STEP
/// Stores loop bounds in first_index/last_index
pub fn rpnx_for(ctx: &mut Context, args: &mut BranchArgs) -> Result<usize> {
    min_arguments!(ctx, 2);
    arg_must_be!(ctx, 0, Number);
    arg_must_be!(ctx, 1, Number);

    let end_value = pop_one_number!(ctx);
    let start_value = pop_one_number!(ctx);

    args.first_index = start_value.to_f64();
    args.last_index = end_value.to_f64();

    if start_value > end_value {
        // Skip loop entirely
        Ok(args.arg2 + 1)
    } else {
        // Initialize loop variable in local heap
        // This will be done by the execution engine
        Ok(args.arg1 + 1) // Jump to instruction after loop variable
    }
}

/// NEXT: next (increment loop counter and test)
/// arg1 = address of FOR/START
/// Note: This function needs access to the FOR/START instruction to get its arg1
/// The actual jump target is calculated in the execution loop
pub fn rpnx_next(_ctx: &mut Context, args: &mut BranchArgs) -> Result<usize> {
    // Initialize counter on first iteration
    if !args.arg_bool {
        args.arg_bool = true;
        args.iteration_count = 0;
        args.step_value = 1.0;
    }

    // Increment iteration counter
    args.iteration_count += 1;

    // Compute current value: start + (iteration * 1)
    let current_value = args.first_index + (args.iteration_count as f64 * 1.0);

    // Test condition
    if current_value > args.last_index {
        args.arg_bool = false; // Reset for next loop
        args.iteration_count = 0;
        Ok(STEP_OUT) // Exit loop
    } else {
        // Return a special marker that execution loop will handle
        // Execution loop will look up FOR/START's arg1 and jump to that + 1
        Ok(args.arg1) // FOR/START index (execution loop adds 1 to FOR's arg1)
    }
}

/// STEP: step (custom step increment for loop)
/// Stack: <step> -> (empty)
/// arg1 = address of FOR/START
pub fn rpnx_step(ctx: &mut Context, args: &mut BranchArgs) -> Result<usize> {
    min_arguments!(ctx, 1);
    arg_must_be!(ctx, 0, Number);

    let (step_value, _base) = match ctx.stack.pop().unwrap() {
        Object::Number { value, base } => (value, base),
        _ => unreachable!(),
    };

    let step_f64 = step_value.to_f64();

    if step_f64 <= 0.0 {
        return Err(Error::OutOfRange);
    }

    // Initialize on first iteration
    if !args.arg_bool {
        args.arg_bool = true;
        args.step_value = step_f64;
        args.iteration_count = 0;

        // Store the exact step value in heap for precise computation
        let step_key = format!("__step_{}", args.arg1);
        ctx.local_heap.store(
            step_key,
            Object::Number {
                value: step_value,
                base: 10,
            },
        );
    }

    // Increment iteration counter
    args.iteration_count += 1;

    // Retrieve exact step value from heap and compute with MPFR precision
    let step_key = format!("__step_{}", args.arg1);
    let current_value_f64 = if let Some(Object::Number {
        value: step_mpfr, ..
    }) = ctx.local_heap.recall(&step_key)
    {
        use rug::Float;
        // Compute: start + (iteration * step) using MPFR
        let start = Float::with_val(ctx.config.precision_bits, args.first_index);
        let iteration = Float::with_val(ctx.config.precision_bits, args.iteration_count);
        let current = start + (iteration * step_mpfr);
        current.to_f64()
    } else {
        // Fallback to f64 if step not found (shouldn't happen)
        args.first_index + (args.iteration_count as f64 * args.step_value)
    };

    // Test condition
    if current_value_f64 > args.last_index {
        args.arg_bool = false;
        args.iteration_count = 0;
        // Clean up step value from heap
        ctx.local_heap.purge(&step_key);
        Ok(STEP_OUT)
    } else {
        // Return FOR/START index (execution loop will compute jump target)
        Ok(args.arg1)
    }
}

/// START: start (start counted loop without variable)
/// Stack: <start> <end> -> (empty)
/// arg1 = STEP_OUT (no loop variable)
/// arg2 = address of NEXT/STEP
pub fn rpnx_start(ctx: &mut Context, args: &mut BranchArgs) -> Result<usize> {
    min_arguments!(ctx, 2);
    arg_must_be!(ctx, 0, Number);
    arg_must_be!(ctx, 1, Number);

    let end_value = pop_one_number!(ctx);
    let start_value = pop_one_number!(ctx);

    args.first_index = start_value.to_f64();
    args.last_index = end_value.to_f64();

    if start_value > end_value {
        // Skip loop entirely
        Ok(args.arg2 + 1)
    } else {
        Ok(STEP_OUT) // Continue to loop body
    }
}

/// WHILE: while (start condition-based loop)
/// No action - condition is evaluated before REPEAT
pub fn rpnx_while(_ctx: &mut Context, _args: &mut BranchArgs) -> Result<usize> {
    Ok(STEP_OUT)
}

/// REPEAT: repeat (test condition and loop)
/// Stack: condition -> (empty)
/// arg1 = address of END+1 (exit point)
pub fn rpnx_repeat(ctx: &mut Context, args: &mut BranchArgs) -> Result<usize> {
    min_arguments!(ctx, 1);
    arg_must_be!(ctx, 0, Number);

    let condition = pop_one_number!(ctx);

    if condition == 0.0 {
        Ok(args.arg1) // Exit to END+1
    } else {
        Ok(STEP_OUT) // Continue to END
    }
}

/// DO: do (start unconditional loop)
/// No action - just marks loop start
pub fn rpnx_do(_ctx: &mut Context, _args: &mut BranchArgs) -> Result<usize> {
    Ok(STEP_OUT)
}

/// UNTIL: until (no-op, condition tested at END)
/// Stack: condition -> (leaves on stack for END)
pub fn rpnx_until(_ctx: &mut Context, _args: &mut BranchArgs) -> Result<usize> {
    Ok(STEP_OUT)
}

/// IFT: ift (inline if-then: condition value -> value if true, nothing if false)
/// Stack: condition value -> value (if condition != 0) or nothing (if condition == 0)
/// IFT: ift (inline if-then: condition value -> conditionally pushes value)
/// Stack: condition value -> value (if true) or nothing (if false)
/// If value is a Program, executes it; otherwise pushes it to stack
pub fn rpnx_ift(ctx: &mut Context, _args: &mut BranchArgs) -> Result<usize> {
    min_arguments!(ctx, 2);
    arg_must_be!(ctx, 1, Number);

    // Pop value first (top of stack)
    let value = ctx.stack.pop().unwrap();

    // Pop condition
    let condition = pop_one_number!(ctx);

    if condition != 0.0 {
        // True: push value or execute program
        match value {
            Object::Program(p) => {
                crate::program::run_objects(&p.objects, ctx)?;
            }
            _ => {
                ctx.stack.push(value);
            }
        }
    }
    // else: drop the value (don't push it back)

    Ok(STEP_OUT)
}

/// IFTE: ifte (inline if-then-else: condition true_value false_value -> pushes one value)
/// Stack: condition true_value false_value -> result
/// If value is a Program, executes it; otherwise pushes it to stack
pub fn rpnx_ifte(ctx: &mut Context, _args: &mut BranchArgs) -> Result<usize> {
    min_arguments!(ctx, 3);
    arg_must_be!(ctx, 2, Number);

    let false_value = match ctx.stack.pop() {
        Some(obj) => obj,
        None => return Err(Error::MissingOperand),
    };

    let true_value = match ctx.stack.pop() {
        Some(obj) => obj,
        None => return Err(Error::MissingOperand),
    };

    let condition = pop_one_number!(ctx);

    if condition != 0.0 {
        // True branch
        match true_value {
            Object::Program(p) => {
                crate::program::run_objects(&p.objects, ctx)?;
            }
            _ => {
                ctx.stack.push(true_value);
            }
        }
    } else {
        // False branch
        match false_value {
            Object::Program(p) => {
                crate::program::run_objects(&p.objects, ctx)?;
            }
            _ => {
                ctx.stack.push(false_value);
            }
        }
    }

    Ok(STEP_OUT)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rug::Float;

    #[test]
    fn test_if_true() {
        let mut ctx = Context::new();
        let mut args = BranchArgs::default();

        // Push condition: 1 (true)
        use rug::Float;
        push_number!(ctx, Float::with_val(128, 1));

        rpnx_if(&mut ctx, &mut args).unwrap();

        assert_eq!(args.condition, 1); // Condition should be true
        assert_eq!(ctx.stack.len(), 0); // Stack should be empty
    }

    #[test]
    fn test_if_false() {
        let mut ctx = Context::new();
        let mut args = BranchArgs::default();

        // Push condition: 0 (false)
        use rug::Float;
        push_number!(ctx, Float::with_val(128, 0));

        rpnx_if(&mut ctx, &mut args).unwrap();

        assert_eq!(args.condition, 0); // Condition should be false
        assert_eq!(ctx.stack.len(), 0);
    }

    #[test]
    fn test_for_loop() {
        let mut ctx = Context::new();
        let mut args = BranchArgs::default();

        // Set up FOR: 1 10 for
        push_number!(ctx, Float::with_val(128, 1));
        push_number!(ctx, Float::with_val(128, 10));

        args.arg1 = 5; // Dummy symbol address
        args.arg2 = 20; // Dummy NEXT address

        let result = rpnx_for(&mut ctx, &mut args).unwrap();

        assert_eq!(args.first_index, 1.0);
        assert_eq!(args.last_index, 10.0);
        assert_eq!(result, 6); // Should jump to arg1 + 1
        assert_eq!(ctx.stack.len(), 0);
    }
}
