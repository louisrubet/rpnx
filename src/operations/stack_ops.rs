// SPDX-License-Identifier: GPL-3.0-only
// Stack manipulation operations

use crate::context::Context;
use crate::error::{Error, Result};
use crate::object::Object;

/// Swap top two items: swap
pub fn swap(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 2);

    let a = ctx.stack.pop().unwrap();
    let b = ctx.stack.pop().unwrap();
    ctx.stack.push(a);
    ctx.stack.push(b);
    Ok(())
}

/// Duplicate top item: dup
pub fn dup(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    let obj = ctx.stack.get(0).unwrap().clone();
    ctx.stack.push(obj);
    Ok(())
}

/// Duplicate top two items: dup2
pub fn dup2(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 2);

    let obj1 = ctx.stack.get(1).unwrap().clone();
    let obj0 = ctx.stack.get(0).unwrap().clone();
    ctx.stack.push(obj1);
    ctx.stack.push(obj0);
    Ok(())
}

/// Duplicate top n items: dupn
pub fn dupn(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);
    arg_must_be!(ctx, 0, Number);

    // Get count from stack
    let n = match ctx.stack.pop().unwrap() {
        Object::Number { value, .. } => {
            let f = value.to_f64();
            if !f.is_finite() || f < 0.0 || f.fract() != 0.0 {
                return Err(Error::BadOperandType);
            }
            f as usize
        }
        _ => unreachable!(),
    };

    if n == 0 {
        return Err(Error::OutOfRange);
    }

    min_arguments!(ctx, n);

    // Duplicate top n items - collect them first before pushing
    let items: Vec<_> = (0..n)
        .rev()
        .map(|i| ctx.stack.get(i).unwrap().clone())
        .collect();

    for obj in items {
        ctx.stack.push(obj);
    }

    Ok(())
}

/// Drop (remove) top item: drop, pop
pub fn drop(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);
    ctx.stack.pop();
    Ok(())
}

/// Drop top two items: drop2
pub fn drop2(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 2);
    ctx.stack.erase(0, 2);
    Ok(())
}

/// Drop top n items: dropn
pub fn dropn(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);
    arg_must_be!(ctx, 0, Number);

    // Get count from stack
    let n = match ctx.stack.get(0).unwrap() {
        Object::Number { value, .. } => {
            let f = value.to_f64();
            if !f.is_finite() || f < 0.0 || f.fract() != 0.0 {
                return Err(Error::BadOperandType);
            }
            f as usize
        }
        _ => unreachable!(),
    };

    min_arguments!(ctx, n + 1);
    ctx.stack.erase(0, n + 1);
    Ok(())
}

/// Erase entire stack: erase, del, clear
pub fn erase(ctx: &mut Context) -> Result<()> {
    ctx.stack.clear();
    Ok(())
}

/// Pick: copy item at level n to top
pub fn pick(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);
    arg_must_be!(ctx, 0, Number);

    // Get level from stack
    let level = match ctx.stack.pop().unwrap() {
        Object::Number { value, .. } => {
            let f = value.to_f64();
            if !f.is_finite() || f.fract() != 0.0 {
                return Err(Error::BadOperandType);
            }
            if f < 1.0 {
                return Err(Error::OutOfRange);
            }
            f as usize
        }
        _ => unreachable!(),
    };

    // Check bounds
    if level > ctx.stack.len() {
        return Err(Error::OutOfRange);
    }

    // Pick item (level 1 = top, level 2 = second, etc.)
    let obj = ctx.stack.get(level - 1).unwrap().clone();
    ctx.stack.push(obj);
    Ok(())
}

/// Rotate top 3 items: rot
/// Before: 3 2 1 -> After: 2 1 3
pub fn rot(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 3);

    let obj2 = ctx.stack.get(2).unwrap().clone();
    ctx.stack.erase(2, 1);
    ctx.stack.push(obj2);
    Ok(())
}

/// Roll: move nth item to top
pub fn roll(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);
    arg_must_be!(ctx, 0, Number);

    // Get count from stack
    let n = match ctx.stack.pop().unwrap() {
        Object::Number { value, .. } => {
            let f = value.to_f64();
            if !f.is_finite() || f < 0.0 || f.fract() != 0.0 {
                return Err(Error::BadOperandType);
            }
            f as usize
        }
        _ => unreachable!(),
    };

    if n == 0 {
        return Err(Error::OutOfRange);
    }

    min_arguments!(ctx, n);

    // Roll: move nth item to top
    let obj = ctx.stack.get(n - 1).unwrap().clone();
    ctx.stack.erase(n - 1, 1);
    ctx.stack.push(obj);
    Ok(())
}

/// Roll down: move top item to nth position
pub fn rolld(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 2);
    arg_must_be!(ctx, 0, Number);

    // Get count from stack
    let n = match ctx.stack.pop().unwrap() {
        Object::Number { value, .. } => {
            let f = value.to_f64();
            if !f.is_finite() || f < 0.0 || f.fract() != 0.0 {
                return Err(Error::BadOperandType);
            }
            f as usize
        }
        _ => unreachable!(),
    };

    if n == 0 {
        return Err(Error::OutOfRange);
    }

    min_arguments!(ctx, n);

    // Roll down: move top to nth position
    let obj = ctx.stack.pop().unwrap();

    // Insert at position n-1 (0-indexed)
    // We need to temporarily convert VecDeque to Vec for insertion
    let mut temp: Vec<_> = ctx.stack.iter().cloned().collect();
    if n - 1 < temp.len() {
        temp.insert(n - 1, obj);
        ctx.stack.clear();
        for item in temp.into_iter().rev() {
            ctx.stack.push(item);
        }
    } else {
        return Err(Error::OutOfRange);
    }

    Ok(())
}

/// Over: copy second item to top
pub fn over(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 2);

    let obj = ctx.stack.get(1).unwrap().clone();
    ctx.stack.push(obj);
    Ok(())
}

/// Depth: push stack size onto stack
pub fn depth(ctx: &mut Context) -> Result<()> {
    use rug::Float;
    let size = ctx.stack.len();
    push_number!(ctx, Float::with_val(ctx.config.precision_bits, size));
    Ok(())
}

/// Edit: pop stack level 1 and present it for editing
/// This is a special operation that signals the REPL to enter edit mode
pub fn edit(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    // Pop the top of stack
    let obj = ctx.stack.pop().unwrap();

    // Convert to editable string representation
    let content = obj.display(&ctx.config);

    // Return special Edit error that carries the content
    Err(crate::error::Error::Edit(content))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object::Object;
    use rug::Float;

    #[test]
    fn test_swap() {
        let mut ctx = Context::new();
        ctx.stack.push(Object::Number {
            value: Float::with_val(128, 1),
            base: 10,
        });
        ctx.stack.push(Object::Number {
            value: Float::with_val(128, 2),
            base: 10,
        });

        swap(&mut ctx).unwrap();

        assert_eq!(ctx.stack.len(), 2);
        match ctx.stack.get(0).unwrap() {
            Object::Number { value, .. } => assert_eq!(value.to_f64(), 1.0),
            _ => panic!("Expected number"),
        }
    }

    #[test]
    fn test_dup() {
        let mut ctx = Context::new();
        ctx.stack.push(Object::Number {
            value: Float::with_val(128, 42),
            base: 10,
        });

        dup(&mut ctx).unwrap();

        assert_eq!(ctx.stack.len(), 2);
    }

    #[test]
    fn test_rot() {
        let mut ctx = Context::new();
        ctx.stack.push(Object::Number {
            value: Float::with_val(128, 3),
            base: 10,
        });
        ctx.stack.push(Object::Number {
            value: Float::with_val(128, 2),
            base: 10,
        });
        ctx.stack.push(Object::Number {
            value: Float::with_val(128, 1),
            base: 10,
        });

        rot(&mut ctx).unwrap();

        assert_eq!(ctx.stack.len(), 3);
        // After rot: 2 1 3
        match ctx.stack.get(0).unwrap() {
            Object::Number { value, .. } => assert_eq!(value.to_f64(), 3.0),
            _ => panic!("Expected number"),
        }
    }
}
