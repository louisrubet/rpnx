// SPDX-License-Identifier: GPL-3.0-only
// Variable storage operations

use crate::context::Context;
use crate::error::{Error, Result};
use crate::object::Object;

/// Store: sto (store value with symbol name)
/// Stack: value 'name' -> (empty)
pub fn sto(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 2);

    // Top of stack must be a symbol (variable name)
    let name = match ctx.stack.get(0) {
        Some(Object::Symbol { name, .. }) => name.clone(),
        _ => return Err(Error::BadOperandType),
    };

    // Pop the symbol
    ctx.stack.pop();

    // Pop the value and store it
    let value = ctx.stack.pop().ok_or(Error::MissingOperand)?;

    // Store in local_heap if variable already exists there, otherwise global_heap
    if ctx.local_heap.contains(&name) {
        ctx.local_heap.store(name, value);
    } else {
        ctx.global_heap.store(name, value);
    }

    Ok(())
}

/// Recall: rcl (recall variable value)
/// Stack: 'name' -> value
pub fn rcl(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    // Top of stack must be a symbol
    let name = match ctx.stack.pop() {
        Some(Object::Symbol { name, .. }) => name,
        _ => return Err(Error::BadOperandType),
    };

    // Look up variable in heaps (local -> parent -> global)
    if let Some(obj) = ctx.find_variable(&name) {
        ctx.stack.push(obj.clone());
        Ok(())
    } else {
        // Push the symbol back onto the stack before returning error
        ctx.stack.push(Object::Symbol {
            name: name.clone(),
            auto_eval: false,
        });
        Err(Error::UnknownVariable(name))
    }
}

/// Purge: purge (delete variable)
/// Stack: 'name' -> (empty)
pub fn purge(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    // Top of stack must be a symbol
    let name = match ctx.stack.pop() {
        Some(Object::Symbol { name, .. }) => name,
        _ => return Err(Error::BadOperandType),
    };

    // Remove from global heap
    if ctx.global_heap.purge(&name).is_some() {
        Ok(())
    } else {
        Err(Error::UnknownVariable(name))
    }
}

/// Store add: sto+ (add to stored variable)
/// Stack: value 'name' -> (empty)
pub fn sto_add(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 2);

    // Check that top is symbol
    if !matches!(ctx.stack.get(0), Some(Object::Symbol { .. })) {
        return Err(Error::BadOperandType);
    }

    // Check variable exists
    let name = match ctx.stack.get(0) {
        Some(Object::Symbol { name, .. }) => name.clone(),
        _ => unreachable!(),
    };

    if !ctx.global_heap.contains(&name) && !ctx.local_heap.contains(&name) {
        return Err(Error::UnknownVariable(name));
    }

    // Implement: dup rcl rot + swap sto
    // Stack: value 'name'
    crate::operations::stack_ops::dup(ctx)?; // value 'name' 'name'
    rcl(ctx)?; // value 'name' old_value
    crate::operations::stack_ops::rot(ctx)?; // 'name' old_value value
    crate::operations::real::plus(ctx)?; // 'name' new_value
    crate::operations::stack_ops::swap(ctx)?; // new_value 'name'
    sto(ctx)?; // (empty)

    Ok(())
}

/// Store subtract: sto- (subtract from stored variable)
pub fn sto_sub(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 2);

    if !matches!(ctx.stack.get(0), Some(Object::Symbol { .. })) {
        return Err(Error::BadOperandType);
    }

    let name = match ctx.stack.get(0) {
        Some(Object::Symbol { name, .. }) => name.clone(),
        _ => unreachable!(),
    };

    if !ctx.global_heap.contains(&name) && !ctx.local_heap.contains(&name) {
        return Err(Error::UnknownVariable(name));
    }

    crate::operations::stack_ops::dup(ctx)?;
    rcl(ctx)?;
    crate::operations::stack_ops::rot(ctx)?;
    crate::operations::real::minus(ctx)?;
    crate::operations::stack_ops::swap(ctx)?;
    sto(ctx)?;

    Ok(())
}

/// Store multiply: sto* (multiply stored variable)
pub fn sto_mul(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 2);

    if !matches!(ctx.stack.get(0), Some(Object::Symbol { .. })) {
        return Err(Error::BadOperandType);
    }

    let name = match ctx.stack.get(0) {
        Some(Object::Symbol { name, .. }) => name.clone(),
        _ => unreachable!(),
    };

    if !ctx.global_heap.contains(&name) && !ctx.local_heap.contains(&name) {
        return Err(Error::UnknownVariable(name));
    }

    crate::operations::stack_ops::dup(ctx)?;
    rcl(ctx)?;
    crate::operations::stack_ops::rot(ctx)?;
    crate::operations::real::multiply(ctx)?;
    crate::operations::stack_ops::swap(ctx)?;
    sto(ctx)?;

    Ok(())
}

/// Store divide: sto/ (divide stored variable)
pub fn sto_div(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 2);

    if !matches!(ctx.stack.get(0), Some(Object::Symbol { .. })) {
        return Err(Error::BadOperandType);
    }

    let name = match ctx.stack.get(0) {
        Some(Object::Symbol { name, .. }) => name.clone(),
        _ => unreachable!(),
    };

    if !ctx.global_heap.contains(&name) && !ctx.local_heap.contains(&name) {
        return Err(Error::UnknownVariable(name));
    }

    crate::operations::stack_ops::dup(ctx)?;
    rcl(ctx)?;
    crate::operations::stack_ops::rot(ctx)?;
    crate::operations::real::divide(ctx)?;
    crate::operations::stack_ops::swap(ctx)?;
    sto(ctx)?;

    Ok(())
}

/// Store negate: stoneg (negate stored variable)
/// Stack: 'name' -> (empty)
pub fn sto_neg(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    if !matches!(ctx.stack.get(0), Some(Object::Symbol { .. })) {
        return Err(Error::BadOperandType);
    }

    let name = match ctx.stack.get(0) {
        Some(Object::Symbol { name, .. }) => name.clone(),
        _ => unreachable!(),
    };

    if !ctx.global_heap.contains(&name) && !ctx.local_heap.contains(&name) {
        return Err(Error::UnknownVariable(name));
    }

    crate::operations::stack_ops::dup(ctx)?;
    rcl(ctx)?;
    crate::operations::real::neg(ctx)?;
    crate::operations::stack_ops::swap(ctx)?;
    sto(ctx)?;

    Ok(())
}

/// Store inverse: stoinv (inverse stored variable)
pub fn sto_inv(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    if !matches!(ctx.stack.get(0), Some(Object::Symbol { .. })) {
        return Err(Error::BadOperandType);
    }

    let name = match ctx.stack.get(0) {
        Some(Object::Symbol { name, .. }) => name.clone(),
        _ => unreachable!(),
    };

    if !ctx.global_heap.contains(&name) && !ctx.local_heap.contains(&name) {
        return Err(Error::UnknownVariable(name));
    }

    crate::operations::stack_ops::dup(ctx)?;
    rcl(ctx)?;
    crate::operations::real::inv(ctx)?;
    crate::operations::stack_ops::swap(ctx)?;
    sto(ctx)?;

    Ok(())
}

/// Variables: vars (list all variables)
pub fn vars(ctx: &mut Context) -> Result<()> {
    println!("\nGlobal variables:");
    let mut index = 1;
    for name in ctx.global_heap.var_names() {
        if let Some(obj) = ctx.global_heap.recall(name) {
            println!("  var {}: name '{}', type {}", index, name, obj.type_name());
            index += 1;
        }
    }

    if !ctx.local_heap.is_empty() {
        println!("\nLocal variables:");
        for name in ctx.local_heap.var_names() {
            if let Some(obj) = ctx.local_heap.recall(name) {
                println!("  var {}: name '{}', type {}", index, name, obj.type_name());
                index += 1;
            }
        }
    }

    if index == 1 {
        println!("  (no variables defined)");
    }

    Ok(())
}

/// Clear user variables: clusr (clear all global heap variables)
pub fn clusr(ctx: &mut Context) -> Result<()> {
    ctx.global_heap.clear();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object::Object;
    use rug::Float;

    #[test]
    fn test_sto_rcl() {
        let mut ctx = Context::new();

        // Store: 42 'x' sto
        ctx.stack.push(Object::Number {
            value: Float::with_val(128, 42),
            base: 10,
        });
        ctx.stack.push(Object::Symbol {
            name: "x".to_string(),
            auto_eval: false,
        });
        sto(&mut ctx).unwrap();

        assert_eq!(ctx.stack.len(), 0);
        assert!(ctx.global_heap.contains("x"));

        // Recall: 'x' rcl
        ctx.stack.push(Object::Symbol {
            name: "x".to_string(),
            auto_eval: false,
        });
        rcl(&mut ctx).unwrap();

        assert_eq!(ctx.stack.len(), 1);
        match ctx.stack.get(0).unwrap() {
            Object::Number { value, .. } => assert_eq!(value.to_f64(), 42.0),
            _ => panic!("Expected number"),
        }
    }

    #[test]
    fn test_sto_add() {
        let mut ctx = Context::new();

        // Store initial value: 10 'x' sto
        ctx.stack.push(Object::Number {
            value: Float::with_val(128, 10),
            base: 10,
        });
        ctx.stack.push(Object::Symbol {
            name: "x".to_string(),
            auto_eval: false,
        });
        sto(&mut ctx).unwrap();

        // Add to variable: 5 'x' sto+
        ctx.stack.push(Object::Number {
            value: Float::with_val(128, 5),
            base: 10,
        });
        ctx.stack.push(Object::Symbol {
            name: "x".to_string(),
            auto_eval: false,
        });
        sto_add(&mut ctx).unwrap();

        // Recall and check: 'x' rcl
        ctx.stack.push(Object::Symbol {
            name: "x".to_string(),
            auto_eval: false,
        });
        rcl(&mut ctx).unwrap();

        match ctx.stack.get(0).unwrap() {
            Object::Number { value, .. } => assert_eq!(value.to_f64(), 15.0),
            _ => panic!("Expected number"),
        }
    }

    #[test]
    fn test_purge() {
        let mut ctx = Context::new();

        // Store: 99 'temp' sto
        ctx.stack.push(Object::Number {
            value: Float::with_val(128, 99),
            base: 10,
        });
        ctx.stack.push(Object::Symbol {
            name: "temp".to_string(),
            auto_eval: false,
        });
        sto(&mut ctx).unwrap();

        assert!(ctx.global_heap.contains("temp"));

        // Purge: 'temp' purge
        ctx.stack.push(Object::Symbol {
            name: "temp".to_string(),
            auto_eval: false,
        });
        purge(&mut ctx).unwrap();

        assert!(!ctx.global_heap.contains("temp"));
    }
}
