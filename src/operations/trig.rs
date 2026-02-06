// SPDX-License-Identifier: GPL-3.0-only
// Trigonometric operations

use crate::context::Context;
use crate::error::Result;
use crate::object::Object;
use rug::float::Constant;
use rug::Float;

/// SIN: sin (sine function, input in radians)
pub fn sin(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    match ctx.stack.get(0) {
        Some(Object::Number { .. }) => {
            // Extract base before popping
            let (value, base) = match ctx.stack.pop().unwrap() {
                Object::Number { value, base } => (value, base),
                _ => unreachable!(),
            };
            let result = value.sin();
            push_number!(ctx, result, base);
            Ok(())
        }
        Some(Object::Complex { .. }) => {
            let z = match ctx.stack.pop().unwrap() {
                Object::Complex { value, re_base, .. } => (value, re_base),
                _ => unreachable!(),
            };
            let result = z.0.sin();
            ctx.stack.push(Object::Complex {
                value: result,
                re_base: z.1,
                im_base: z.1,
            });
            Ok(())
        }
        _ => Err(crate::error::Error::BadOperandType),
    }
}

/// COS: cos (cosine function, input in radians)
pub fn cos(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    match ctx.stack.get(0) {
        Some(Object::Number { .. }) => {
            // Extract base before popping
            let (value, base) = match ctx.stack.pop().unwrap() {
                Object::Number { value, base } => (value, base),
                _ => unreachable!(),
            };
            let result = value.cos();
            push_number!(ctx, result, base);
            Ok(())
        }
        Some(Object::Complex { .. }) => {
            let z = match ctx.stack.pop().unwrap() {
                Object::Complex { value, re_base, .. } => (value, re_base),
                _ => unreachable!(),
            };
            let result = z.0.cos();
            ctx.stack.push(Object::Complex {
                value: result,
                re_base: z.1,
                im_base: z.1,
            });
            Ok(())
        }
        _ => Err(crate::error::Error::BadOperandType),
    }
}

/// TAN: tan (tangent function, input in radians)
pub fn tan(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    match ctx.stack.get(0) {
        Some(Object::Number { .. }) => {
            // Extract base before popping
            let (value, base) = match ctx.stack.pop().unwrap() {
                Object::Number { value, base } => (value, base),
                _ => unreachable!(),
            };
            let result = value.tan();
            push_number!(ctx, result, base);
            Ok(())
        }
        Some(Object::Complex { .. }) => {
            let z = match ctx.stack.pop().unwrap() {
                Object::Complex { value, re_base, .. } => (value, re_base),
                _ => unreachable!(),
            };
            let result = z.0.tan();
            ctx.stack.push(Object::Complex {
                value: result,
                re_base: z.1,
                im_base: z.1,
            });
            Ok(())
        }
        _ => Err(crate::error::Error::BadOperandType),
    }
}

/// ASIN: asin (arcsine function, returns radians)
pub fn asin(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    match ctx.stack.get(0) {
        Some(Object::Number { .. }) => {
            // Extract base before popping
            let (value, base) = match ctx.stack.pop().unwrap() {
                Object::Number { value, base } => (value, base),
                _ => unreachable!(),
            };
            let result = value.asin();
            push_number!(ctx, result, base);
            Ok(())
        }
        Some(Object::Complex { .. }) => {
            let z = match ctx.stack.pop().unwrap() {
                Object::Complex { value, re_base, .. } => (value, re_base),
                _ => unreachable!(),
            };
            let result = z.0.asin();
            ctx.stack.push(Object::Complex {
                value: result,
                re_base: z.1,
                im_base: z.1,
            });
            Ok(())
        }
        _ => Err(crate::error::Error::BadOperandType),
    }
}

/// ACOS: acos (arccosine function, returns radians)
pub fn acos(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    match ctx.stack.get(0) {
        Some(Object::Number { .. }) => {
            // Extract base before popping
            let (value, base) = match ctx.stack.pop().unwrap() {
                Object::Number { value, base } => (value, base),
                _ => unreachable!(),
            };
            let result = value.acos();
            push_number!(ctx, result, base);
            Ok(())
        }
        Some(Object::Complex { .. }) => {
            let z = match ctx.stack.pop().unwrap() {
                Object::Complex { value, re_base, .. } => (value, re_base),
                _ => unreachable!(),
            };
            let result = z.0.acos();
            ctx.stack.push(Object::Complex {
                value: result,
                re_base: z.1,
                im_base: z.1,
            });
            Ok(())
        }
        _ => Err(crate::error::Error::BadOperandType),
    }
}

/// ATAN: atan (arctangent function, returns radians)
pub fn atan(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    match ctx.stack.get(0) {
        Some(Object::Number { .. }) => {
            // Extract base before popping
            let (value, base) = match ctx.stack.pop().unwrap() {
                Object::Number { value, base } => (value, base),
                _ => unreachable!(),
            };
            let result = value.atan();
            push_number!(ctx, result, base);
            Ok(())
        }
        Some(Object::Complex { .. }) => {
            let z = match ctx.stack.pop().unwrap() {
                Object::Complex { value, re_base, .. } => (value, re_base),
                _ => unreachable!(),
            };
            let result = z.0.atan();
            ctx.stack.push(Object::Complex {
                value: result,
                re_base: z.1,
                im_base: z.1,
            });
            Ok(())
        }
        _ => Err(crate::error::Error::BadOperandType),
    }
}

/// D->R: d->r (degrees to radians)
pub fn d_to_r(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);
    arg_must_be!(ctx, 0, Number);

    // Extract base before popping
    let (degrees, base) = match ctx.stack.pop().unwrap() {
        Object::Number { value, base } => (value, base),
        _ => unreachable!(),
    };
    // radians = degrees * pi / 180
    let pi = Float::with_val(ctx.config.precision_bits, Constant::Pi);
    let factor = pi / 180.0;
    let result = degrees * factor;
    push_number!(ctx, result, base);
    Ok(())
}

/// R->D: r->d (radians to degrees)
pub fn r_to_d(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);
    arg_must_be!(ctx, 0, Number);

    // Extract base before popping
    let (radians, base) = match ctx.stack.pop().unwrap() {
        Object::Number { value, base } => (value, base),
        _ => unreachable!(),
    };
    // degrees = radians * 180 / pi
    let pi = Float::with_val(ctx.config.precision_bits, Constant::Pi);
    let factor = 180.0 / pi;
    let result = radians * factor;
    push_number!(ctx, result, base);
    Ok(())
}

/// PI: pi (push pi constant onto stack)
pub fn pi(ctx: &mut Context) -> Result<()> {
    let pi = Float::with_val(ctx.config.precision_bits, Constant::Pi);
    push_number!(ctx, pi);
    Ok(())
}

/// ATAN2: atan2 (two-argument arctangent)
/// Stack: x y -> atan2(y, x)
pub fn atan2(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 2);
    arg_must_be!(ctx, 0, Number);
    arg_must_be!(ctx, 1, Number);

    // Extract base from left operand (stack position 1) before popping
    let base = match ctx.stack.get(1) {
        Some(Object::Number { base, .. }) => *base,
        _ => 10,
    };

    let (x, y) = pop_two_numbers!(ctx);
    let result = y.atan2(&x);
    push_number!(ctx, result, base);
    Ok(())
}

/// SINH: sinh (hyperbolic sine)
pub fn sinh(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    match ctx.stack.get(0) {
        Some(Object::Number { .. }) => {
            // Extract base before popping
            let (value, base) = match ctx.stack.pop().unwrap() {
                Object::Number { value, base } => (value, base),
                _ => unreachable!(),
            };
            let result = value.sinh();
            push_number!(ctx, result, base);
            Ok(())
        }
        Some(Object::Complex { .. }) => {
            let z = match ctx.stack.pop().unwrap() {
                Object::Complex { value, re_base, .. } => (value, re_base),
                _ => unreachable!(),
            };
            let result = z.0.sinh();
            ctx.stack.push(Object::Complex {
                value: result,
                re_base: z.1,
                im_base: z.1,
            });
            Ok(())
        }
        _ => Err(crate::error::Error::BadOperandType),
    }
}

/// COSH: cosh (hyperbolic cosine)
pub fn cosh(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    match ctx.stack.get(0) {
        Some(Object::Number { .. }) => {
            // Extract base before popping
            let (value, base) = match ctx.stack.pop().unwrap() {
                Object::Number { value, base } => (value, base),
                _ => unreachable!(),
            };
            let result = value.cosh();
            push_number!(ctx, result, base);
            Ok(())
        }
        Some(Object::Complex { .. }) => {
            let z = match ctx.stack.pop().unwrap() {
                Object::Complex { value, re_base, .. } => (value, re_base),
                _ => unreachable!(),
            };
            let result = z.0.cosh();
            ctx.stack.push(Object::Complex {
                value: result,
                re_base: z.1,
                im_base: z.1,
            });
            Ok(())
        }
        _ => Err(crate::error::Error::BadOperandType),
    }
}

/// TANH: tanh (hyperbolic tangent)
pub fn tanh(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    match ctx.stack.get(0) {
        Some(Object::Number { .. }) => {
            // Extract base before popping
            let (value, base) = match ctx.stack.pop().unwrap() {
                Object::Number { value, base } => (value, base),
                _ => unreachable!(),
            };
            let result = value.tanh();
            push_number!(ctx, result, base);
            Ok(())
        }
        Some(Object::Complex { .. }) => {
            let z = match ctx.stack.pop().unwrap() {
                Object::Complex { value, re_base, .. } => (value, re_base),
                _ => unreachable!(),
            };
            let result = z.0.tanh();
            ctx.stack.push(Object::Complex {
                value: result,
                re_base: z.1,
                im_base: z.1,
            });
            Ok(())
        }
        _ => Err(crate::error::Error::BadOperandType),
    }
}

/// ASINH: asinh (inverse hyperbolic sine)
pub fn asinh(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    match ctx.stack.get(0) {
        Some(Object::Number { .. }) => {
            // Extract base before popping
            let (value, base) = match ctx.stack.pop().unwrap() {
                Object::Number { value, base } => (value, base),
                _ => unreachable!(),
            };
            let result = value.asinh();
            push_number!(ctx, result, base);
            Ok(())
        }
        Some(Object::Complex { .. }) => {
            let z = match ctx.stack.pop().unwrap() {
                Object::Complex { value, re_base, .. } => (value, re_base),
                _ => unreachable!(),
            };
            let result = z.0.asinh();
            ctx.stack.push(Object::Complex {
                value: result,
                re_base: z.1,
                im_base: z.1,
            });
            Ok(())
        }
        _ => Err(crate::error::Error::BadOperandType),
    }
}

/// ACOSH: acosh (inverse hyperbolic cosine)
pub fn acosh(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    match ctx.stack.get(0) {
        Some(Object::Number { .. }) => {
            // Extract base before popping
            let (value, base) = match ctx.stack.pop().unwrap() {
                Object::Number { value, base } => (value, base),
                _ => unreachable!(),
            };
            let result = value.acosh();
            push_number!(ctx, result, base);
            Ok(())
        }
        Some(Object::Complex { .. }) => {
            let z = match ctx.stack.pop().unwrap() {
                Object::Complex { value, re_base, .. } => (value, re_base),
                _ => unreachable!(),
            };
            let result = z.0.acosh();
            ctx.stack.push(Object::Complex {
                value: result,
                re_base: z.1,
                im_base: z.1,
            });
            Ok(())
        }
        _ => Err(crate::error::Error::BadOperandType),
    }
}

/// ATANH: atanh (inverse hyperbolic tangent)
pub fn atanh(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    match ctx.stack.get(0) {
        Some(Object::Number { .. }) => {
            // Extract base before popping
            let (value, base) = match ctx.stack.pop().unwrap() {
                Object::Number { value, base } => (value, base),
                _ => unreachable!(),
            };
            let result = value.atanh();
            push_number!(ctx, result, base);
            Ok(())
        }
        Some(Object::Complex { .. }) => {
            let z = match ctx.stack.pop().unwrap() {
                Object::Complex { value, re_base, .. } => (value, re_base),
                _ => unreachable!(),
            };
            let result = z.0.atanh();
            ctx.stack.push(Object::Complex {
                value: result,
                re_base: z.1,
                im_base: z.1,
            });
            Ok(())
        }
        _ => Err(crate::error::Error::BadOperandType),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object::Object;
    use rug::Float;

    #[test]
    fn test_sin_cos() {
        let mut ctx = Context::new();

        // sin(0) = 0
        push_number!(ctx, Float::with_val(128, 0));
        sin(&mut ctx).unwrap();
        match ctx.stack.get(0).unwrap() {
            Object::Number { value, .. } => {
                assert!((value.to_f64() - 0.0).abs() < 1e-10);
            }
            _ => panic!("Expected number"),
        }
        ctx.stack.pop();

        // cos(0) = 1
        push_number!(ctx, Float::with_val(128, 0));
        cos(&mut ctx).unwrap();
        match ctx.stack.get(0).unwrap() {
            Object::Number { value, .. } => {
                assert!((value.to_f64() - 1.0).abs() < 1e-10);
            }
            _ => panic!("Expected number"),
        }
    }

    #[test]
    fn test_pi() {
        let mut ctx = Context::new();

        pi(&mut ctx).unwrap();

        match ctx.stack.get(0).unwrap() {
            Object::Number { value, .. } => {
                assert!((value.to_f64() - std::f64::consts::PI).abs() < 1e-10);
            }
            _ => panic!("Expected number"),
        }
    }

    #[test]
    fn test_d_to_r() {
        let mut ctx = Context::new();

        // 180 degrees = pi radians
        push_number!(ctx, Float::with_val(128, 180));
        d_to_r(&mut ctx).unwrap();

        match ctx.stack.get(0).unwrap() {
            Object::Number { value, .. } => {
                assert!((value.to_f64() - std::f64::consts::PI).abs() < 1e-10);
            }
            _ => panic!("Expected number"),
        }
    }

    #[test]
    fn test_sinh() {
        let mut ctx = Context::new();

        // sinh(0) = 0
        push_number!(ctx, Float::with_val(128, 0));
        sinh(&mut ctx).unwrap();

        match ctx.stack.get(0).unwrap() {
            Object::Number { value, .. } => {
                assert!((value.to_f64() - 0.0).abs() < 1e-10);
            }
            _ => panic!("Expected number"),
        }
    }
}
