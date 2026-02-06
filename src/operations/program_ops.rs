// SPDX-License-Identifier: GPL-3.0-only
// Program evaluation operations

use crate::context::Context;
use crate::error::Result;
use crate::object::Object;

/// EVAL: eval (execute/evaluate any object)
/// Stack: object -> (executes or evaluates object)
/// - Program: executes the program (handles local variables if declared)
/// - Symbol: recalls the variable and pushes its value
/// - Keyword: executes the keyword
/// - Branch: executes the branch
/// - Other (Number/String/Complex): pushes to stack (evaluates to itself)
pub fn eval(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    let obj = ctx.stack.pop().unwrap();

    match obj {
        // Program: execute it
        Object::Program(program) => {
            // Check if program has local variables
            if !program.local_vars.is_empty() {
                // First, execute prefix code (e.g., dup2 *)
                if !program.prefix_objects.is_empty() {
                    crate::program::run_objects(&program.prefix_objects, ctx)?;
                }

                // Now pop values from stack for local variables
                let num_vars = program.local_vars.len();

                // Check we have enough values on stack
                if ctx.stack.len() < num_vars {
                    return Err(crate::error::Error::MissingOperand);
                }

                // Pop values and bind to local variables
                // Variables are bound in order: first variable gets deepest stack value among those being popped
                // With stack [a, b, c] (c on top) and vars [x, y], x gets b and y gets c
                let mut values = Vec::new();
                for _ in 0..num_vars {
                    values.push(ctx.stack.pop().unwrap());
                }

                // Reverse to match order: first variable = first popped value from bottom
                values.reverse();

                // Store in local heap
                for (var_name, value) in program.local_vars.iter().zip(values.iter()) {
                    ctx.local_heap.store(var_name.clone(), value.clone());
                }

                // Execute the program's body objects
                let result = crate::program::run_objects(&program.objects, ctx);

                // Clean up local variables
                for var_name in &program.local_vars {
                    ctx.local_heap.purge(var_name);
                }

                result
            } else {
                // No local variables, execute normally (prefix and body combined)
                if !program.prefix_objects.is_empty() {
                    crate::program::run_objects(&program.prefix_objects, ctx)?;
                }
                crate::program::run_objects(&program.objects, ctx)?;
                Ok(())
            }
        }

        // Symbol: recall variable and push value
        Object::Symbol { name, .. } => {
            if let Some(value) = ctx.find_variable(&name) {
                ctx.stack.push(value.clone());
                Ok(())
            } else {
                // Variable doesn't exist, push the symbol itself
                ctx.stack.push(Object::Symbol {
                    name,
                    auto_eval: false,
                });
                Ok(())
            }
        }

        // Keyword: execute it
        Object::Keyword { handler, .. } => {
            handler(ctx)?;
            Ok(())
        }

        // Branch: execute it (though this is unusual)
        Object::Branch {
            handler, mut args, ..
        } => {
            handler(ctx, &mut args)?;
            Ok(())
        }

        // Everything else (Number, String, Complex): push to stack
        _ => {
            ctx.stack.push(obj);
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::object::Object;
    use crate::parser::Parser;
    use rug::Float;

    #[test]
    fn test_eval() {
        let mut ctx = Context::new();

        // Parse a program: << 2 3 + >>
        let tokens = Lexer::analyze("<< 2 3 + >>").unwrap();
        let objects = Parser::build_objects(tokens).unwrap();

        // Should be a single Program object
        assert_eq!(objects.len(), 1);
        ctx.stack.push(objects[0].clone());

        // Execute eval
        eval(&mut ctx).unwrap();

        // Stack should now have 5 (result of 2+3)
        match ctx.stack.get(0).unwrap() {
            Object::Number { value, .. } => {
                assert_eq!(value.to_f64(), 5.0);
            }
            _ => panic!("Expected number"),
        }
    }

    #[test]
    fn test_program_in_variable() {
        let mut ctx = Context::new();

        // Store a program in a variable: << dup * >> 'sq' sto
        let tokens = Lexer::analyze("<< dup * >>").unwrap();
        let objects = Parser::build_objects(tokens).unwrap();
        ctx.stack.push(objects[0].clone());

        // Store it in 'sq'
        ctx.stack.push(Object::Symbol {
            name: "sq".to_string(),
            auto_eval: false,
        });

        // Call sto operation
        crate::operations::store::sto(&mut ctx).unwrap();

        // Now test: 5 sq (recall and execute)
        push_number!(ctx, Float::with_val(ctx.config.precision_bits, 5));

        // Recall 'sq' - should be a program
        let program = match ctx.global_heap.recall("sq") {
            Some(Object::Program(p)) => p.clone(),
            _ => panic!("Expected program in variable"),
        };
        crate::program::run_objects(&program.objects, &mut ctx).unwrap();

        // Should have 25 on stack
        match ctx.stack.get(0).unwrap() {
            Object::Number { value, .. } => {
                assert_eq!(value.to_f64(), 25.0);
            }
            _ => panic!("Expected number"),
        }
    }
}
