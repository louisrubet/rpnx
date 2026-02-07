// SPDX-License-Identifier: GPL-3.0-only
// Comparison and test operations

use crate::context::Context;
use crate::error::{Error, Result};
use crate::object::Object;
use rug::Float;

/// Greater than: >
pub fn greater(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 2);
    arg_must_be!(ctx, 0, Number);
    arg_must_be!(ctx, 1, Number);

    let (a, b) = pop_two_numbers!(ctx);
    let result = if a > b { 1 } else { 0 };
    push_number!(ctx, Float::with_val(ctx.config.precision_bits, result));
    Ok(())
}

/// Greater than or equal: >=
pub fn greater_eq(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 2);
    arg_must_be!(ctx, 0, Number);
    arg_must_be!(ctx, 1, Number);

    let (a, b) = pop_two_numbers!(ctx);
    let result = if a >= b { 1 } else { 0 };
    push_number!(ctx, Float::with_val(ctx.config.precision_bits, result));
    Ok(())
}

/// Less than: <
pub fn less(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 2);
    arg_must_be!(ctx, 0, Number);
    arg_must_be!(ctx, 1, Number);

    let (a, b) = pop_two_numbers!(ctx);
    let result = if a < b { 1 } else { 0 };
    push_number!(ctx, Float::with_val(ctx.config.precision_bits, result));
    Ok(())
}

/// Less than or equal: <=
pub fn less_eq(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 2);
    arg_must_be!(ctx, 0, Number);
    arg_must_be!(ctx, 1, Number);

    let (a, b) = pop_two_numbers!(ctx);
    let result = if a <= b { 1 } else { 0 };
    push_number!(ctx, Float::with_val(ctx.config.precision_bits, result));
    Ok(())
}

/// Equal: ==
pub fn equal(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 2);

    match (ctx.stack.get(1), ctx.stack.get(0)) {
        (Some(Object::Number { .. }), Some(Object::Number { .. })) => {
            let (a, b) = pop_two_numbers!(ctx);
            let result = if a == b { 1 } else { 0 };
            push_number!(ctx, Float::with_val(ctx.config.precision_bits, result));
            Ok(())
        }
        (Some(Object::Complex { .. }), Some(Object::Complex { .. })) => {
            let b = match ctx.stack.pop().unwrap() {
                Object::Complex { value, .. } => value,
                _ => unreachable!(),
            };
            let a = match ctx.stack.pop().unwrap() {
                Object::Complex { value, .. } => value,
                _ => unreachable!(),
            };
            let result = if a == b { 1 } else { 0 };
            push_number!(ctx, Float::with_val(ctx.config.precision_bits, result));
            Ok(())
        }
        _ => Err(Error::BadOperandType),
    }
}

/// Not equal: !=
pub fn not_equal(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 2);

    match (ctx.stack.get(1), ctx.stack.get(0)) {
        (Some(Object::Number { .. }), Some(Object::Number { .. })) => {
            let (a, b) = pop_two_numbers!(ctx);
            let result = if a != b { 1 } else { 0 };
            push_number!(ctx, Float::with_val(ctx.config.precision_bits, result));
            Ok(())
        }
        (Some(Object::Complex { .. }), Some(Object::Complex { .. })) => {
            let b = match ctx.stack.pop().unwrap() {
                Object::Complex { value, .. } => value,
                _ => unreachable!(),
            };
            let a = match ctx.stack.pop().unwrap() {
                Object::Complex { value, .. } => value,
                _ => unreachable!(),
            };
            let result = if a != b { 1 } else { 0 };
            push_number!(ctx, Float::with_val(ctx.config.precision_bits, result));
            Ok(())
        }
        _ => Err(Error::BadOperandType),
    }
}

/// Logical AND: and
pub fn and(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 2);
    arg_must_be!(ctx, 0, Number);
    arg_must_be!(ctx, 1, Number);

    let (a, b) = pop_two_numbers!(ctx);
    let result = if a != 0 && b != 0 { 1 } else { 0 };
    push_number!(ctx, Float::with_val(ctx.config.precision_bits, result));
    Ok(())
}

/// Logical OR: or
pub fn or(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 2);
    arg_must_be!(ctx, 0, Number);
    arg_must_be!(ctx, 1, Number);

    let (a, b) = pop_two_numbers!(ctx);
    let result = if a != 0 || b != 0 { 1 } else { 0 };
    push_number!(ctx, Float::with_val(ctx.config.precision_bits, result));
    Ok(())
}

/// Logical XOR: xor
pub fn xor(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 2);
    arg_must_be!(ctx, 0, Number);
    arg_must_be!(ctx, 1, Number);

    let (a, b) = pop_two_numbers!(ctx);
    let a_bool = a != 0;
    let b_bool = b != 0;
    let result = if a_bool ^ b_bool { 1 } else { 0 };
    push_number!(ctx, Float::with_val(ctx.config.precision_bits, result));
    Ok(())
}

/// Logical NOT: not
pub fn not(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);
    arg_must_be!(ctx, 0, Number);

    let a = pop_one_number!(ctx);
    let result = if a == 0 { 1 } else { 0 };
    push_number!(ctx, Float::with_val(ctx.config.precision_bits, result));
    Ok(())
}

/// Same: same (alias for ==)
pub fn same(ctx: &mut Context) -> Result<()> {
    equal(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object::Object;
    use rug::Float;

    #[test]
    fn test_greater() {
        let mut ctx = Context::new();
        ctx.stack.push(Object::Number {
            value: Float::with_val(128, 5),
            base: 10,
        });
        ctx.stack.push(Object::Number {
            value: Float::with_val(128, 10),
            base: 10,
        });

        greater(&mut ctx).unwrap();

        assert_eq!(ctx.stack.len(), 1);
        match ctx.stack.get(0).unwrap() {
            Object::Number { value, .. } => assert_eq!(value.to_f64(), 0.0), // 5 > 10 is false
            _ => panic!("Expected number"),
        }
    }

    #[test]
    fn test_equal() {
        let mut ctx = Context::new();
        ctx.stack.push(Object::Number {
            value: Float::with_val(128, 42),
            base: 10,
        });
        ctx.stack.push(Object::Number {
            value: Float::with_val(128, 42),
            base: 10,
        });

        equal(&mut ctx).unwrap();

        assert_eq!(ctx.stack.len(), 1);
        match ctx.stack.get(0).unwrap() {
            Object::Number { value, .. } => assert_eq!(value.to_f64(), 1.0), // Equal
            _ => panic!("Expected number"),
        }
    }

    #[test]
    fn test_and() {
        let mut ctx = Context::new();
        ctx.stack.push(Object::Number {
            value: Float::with_val(128, 1),
            base: 10,
        });
        ctx.stack.push(Object::Number {
            value: Float::with_val(128, 1),
            base: 10,
        });

        and(&mut ctx).unwrap();

        assert_eq!(ctx.stack.len(), 1);
        match ctx.stack.get(0).unwrap() {
            Object::Number { value, .. } => assert_eq!(value.to_f64(), 1.0), // true AND true
            _ => panic!("Expected number"),
        }
    }
}
