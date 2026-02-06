// SPDX-License-Identifier: GPL-3.0-only
// Logarithmic and exponential operations

use crate::context::Context;
use crate::error::Result;
use crate::object::Object;
use rug::ops::Pow;
use rug::Float;

/// LN: ln (natural logarithm, base e)
pub fn ln(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    match ctx.stack.get(0) {
        Some(Object::Number { .. }) => {
            // Extract base before popping
            let (value, base) = match ctx.stack.pop().unwrap() {
                Object::Number { value, base } => (value, base),
                _ => unreachable!(),
            };
            let result = value.ln();
            push_number!(ctx, result, base);
            Ok(())
        }
        Some(Object::Complex { .. }) => {
            let z = match ctx.stack.pop().unwrap() {
                Object::Complex { value, re_base, .. } => (value, re_base),
                _ => unreachable!(),
            };
            let result = z.0.ln();
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

/// EXP: exp (exponential function, e^x)
pub fn exp(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    match ctx.stack.get(0) {
        Some(Object::Number { .. }) => {
            // Extract base before popping
            let (value, base) = match ctx.stack.pop().unwrap() {
                Object::Number { value, base } => (value, base),
                _ => unreachable!(),
            };
            let result = value.exp();
            push_number!(ctx, result, base);
            Ok(())
        }
        Some(Object::Complex { .. }) => {
            let z = match ctx.stack.pop().unwrap() {
                Object::Complex { value, re_base, .. } => (value, re_base),
                _ => unreachable!(),
            };
            let result = z.0.exp();
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

/// LOG10: log10 (common logarithm, base 10)
pub fn log10(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    match ctx.stack.get(0) {
        Some(Object::Number { .. }) => {
            // Extract base before popping
            let (value, base) = match ctx.stack.pop().unwrap() {
                Object::Number { value, base } => (value, base),
                _ => unreachable!(),
            };
            let result = value.log10();
            push_number!(ctx, result, base);
            Ok(())
        }
        Some(Object::Complex { .. }) => {
            let z = match ctx.stack.pop().unwrap() {
                Object::Complex { value, re_base, .. } => (value, re_base),
                _ => unreachable!(),
            };
            let result = z.0.log10();
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

/// ALOG10: alog10 (anti-log base 10, i.e., 10^x)
pub fn alog10(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    match ctx.stack.get(0) {
        Some(Object::Number { .. }) => {
            // Extract base before popping
            let (value, num_base) = match ctx.stack.pop().unwrap() {
                Object::Number { value, base } => (value, base),
                _ => unreachable!(),
            };
            let base = Float::with_val(ctx.config.precision_bits, 10);
            let result = base.pow(&value);
            push_number!(ctx, result, num_base);
            Ok(())
        }
        Some(Object::Complex { .. }) => {
            let z = match ctx.stack.pop().unwrap() {
                Object::Complex { value, re_base, .. } => (value, re_base),
                _ => unreachable!(),
            };
            // 10^z = exp(z * ln(10))
            use rug::Complex;
            let prec = z.0.prec().0; // Get real part precision
            let ln10 = Float::with_val(prec, 10).ln();
            let ln10_complex = Complex::with_val(prec, (ln10, 0));
            let result = (z.0 * ln10_complex).exp();
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

/// LOG2: log2 (logarithm base 2)
pub fn log2(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    match ctx.stack.get(0) {
        Some(Object::Number { .. }) => {
            // Extract base before popping
            let (value, base) = match ctx.stack.pop().unwrap() {
                Object::Number { value, base } => (value, base),
                _ => unreachable!(),
            };
            let result = value.log2();
            push_number!(ctx, result, base);
            Ok(())
        }
        Some(Object::Complex { .. }) => {
            let z = match ctx.stack.pop().unwrap() {
                Object::Complex { value, re_base, .. } => (value, re_base),
                _ => unreachable!(),
            };
            // log2(z) = ln(z) / ln(2)
            use rug::float::Constant;
            let prec = z.0.prec().0; // Get real part precision
            let ln2 = Float::with_val(prec, Constant::Log2);
            let result = z.0.ln() / ln2;
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

/// ALOG2: alog2 (anti-log base 2, i.e., 2^x)
pub fn alog2(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    match ctx.stack.get(0) {
        Some(Object::Number { .. }) => {
            // Extract base before popping
            let (value, num_base) = match ctx.stack.pop().unwrap() {
                Object::Number { value, base } => (value, base),
                _ => unreachable!(),
            };
            let base = Float::with_val(ctx.config.precision_bits, 2);
            let result = base.pow(&value);
            push_number!(ctx, result, num_base);
            Ok(())
        }
        Some(Object::Complex { .. }) => {
            let z = match ctx.stack.pop().unwrap() {
                Object::Complex { value, re_base, .. } => (value, re_base),
                _ => unreachable!(),
            };
            // 2^z = exp(z * ln(2))
            use rug::float::Constant;
            use rug::Complex;
            let prec = z.0.prec().0; // Get real part precision
            let ln2 = Float::with_val(prec, Constant::Log2);
            let ln2_complex = Complex::with_val(prec, (ln2, 0));
            let result = (z.0 * ln2_complex).exp();
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

/// LNPI: lnp1 (ln(1+x), accurate for small x)
pub fn lnp1(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    match ctx.stack.get(0) {
        Some(Object::Number { .. }) => {
            // Extract base before popping
            let (value, base) = match ctx.stack.pop().unwrap() {
                Object::Number { value, base } => (value, base),
                _ => unreachable!(),
            };
            let result = value.ln_1p();
            push_number!(ctx, result, base);
            Ok(())
        }
        Some(Object::Complex { .. }) => {
            let z = match ctx.stack.pop().unwrap() {
                Object::Complex { value, re_base, .. } => (value, re_base),
                _ => unreachable!(),
            };
            // ln(1+z) = ln(z+1)
            use rug::Complex;
            let prec = z.0.prec().0; // Get real part precision
            let one = Complex::with_val(prec, (1, 0));
            let result = (z.0 + one).ln();
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

/// EXPM: expm (exp(x)-1, accurate for small x)
pub fn expm(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    match ctx.stack.get(0) {
        Some(Object::Number { .. }) => {
            // Extract base before popping
            let (value, base) = match ctx.stack.pop().unwrap() {
                Object::Number { value, base } => (value, base),
                _ => unreachable!(),
            };
            let result = value.exp_m1();
            push_number!(ctx, result, base);
            Ok(())
        }
        Some(Object::Complex { .. }) => {
            let z = match ctx.stack.pop().unwrap() {
                Object::Complex { value, re_base, .. } => (value, re_base),
                _ => unreachable!(),
            };
            // exp(z) - 1
            use rug::Complex;
            let prec = z.0.prec().0; // Get real part precision
            let one = Complex::with_val(prec, (1, 0));
            let result = z.0.exp() - one;
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

/// E: e (push Euler-Mascheroni constant γ onto stack)
pub fn e(ctx: &mut Context) -> Result<()> {
    // Euler-Mascheroni constant γ ≈ 0.5772156649...
    use rug::float::Constant;
    let euler = Float::with_val(ctx.config.precision_bits, Constant::Euler);
    push_number!(ctx, euler);
    Ok(())
}

/// LOGN: logn (arbitrary base logarithm)
/// Stack: x base -> log_base(x)
pub fn logn(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 2);
    arg_must_be!(ctx, 0, Number);
    arg_must_be!(ctx, 1, Number);

    // Extract base from left operand (stack position 1) before popping
    let num_base = match ctx.stack.get(1) {
        Some(Object::Number { base, .. }) => *base,
        _ => 10,
    };

    let base = pop_one_number!(ctx);
    let x = pop_one_number!(ctx);

    // log_b(x) = ln(x) / ln(b)
    let ln_x = x.ln();
    let ln_base = base.ln();
    let result = ln_x / ln_base;

    push_number!(ctx, result, num_base);
    Ok(())
}

/// ALOGN: alogn (arbitrary base anti-log, i.e., base^x)
/// Stack: x base -> base^x
pub fn alogn(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 2);
    arg_must_be!(ctx, 0, Number);
    arg_must_be!(ctx, 1, Number);

    // Extract base from left operand (stack position 1) before popping
    let num_base = match ctx.stack.get(1) {
        Some(Object::Number { base, .. }) => *base,
        _ => 10,
    };

    let base = pop_one_number!(ctx);
    let x = pop_one_number!(ctx);

    let result = base.pow(&x);
    push_number!(ctx, result, num_base);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object::Object;
    use rug::Float;

    #[test]
    fn test_ln_exp() {
        let mut ctx = Context::new();

        // ln(e) = 1
        let e_val = Float::with_val(128, 1).exp();
        ctx.stack.push(Object::Number {
            value: e_val,
            base: 10,
        });
        ln(&mut ctx).unwrap();

        match ctx.stack.get(0).unwrap() {
            Object::Number { value, .. } => {
                assert!((value.to_f64() - 1.0).abs() < 1e-10);
            }
            _ => panic!("Expected number"),
        }
        ctx.stack.pop();

        // exp(0) = 1
        push_number!(ctx, Float::with_val(128, 0));
        exp(&mut ctx).unwrap();

        match ctx.stack.get(0).unwrap() {
            Object::Number { value, .. } => {
                assert!((value.to_f64() - 1.0).abs() < 1e-10);
            }
            _ => panic!("Expected number"),
        }
    }

    #[test]
    fn test_log10() {
        let mut ctx = Context::new();

        // log10(100) = 2
        push_number!(ctx, Float::with_val(128, 100));
        log10(&mut ctx).unwrap();

        match ctx.stack.get(0).unwrap() {
            Object::Number { value, .. } => {
                assert!((value.to_f64() - 2.0).abs() < 1e-10);
            }
            _ => panic!("Expected number"),
        }
    }

    #[test]
    fn test_log2() {
        let mut ctx = Context::new();

        // log2(8) = 3
        push_number!(ctx, Float::with_val(128, 8));
        log2(&mut ctx).unwrap();

        match ctx.stack.get(0).unwrap() {
            Object::Number { value, .. } => {
                assert!((value.to_f64() - 3.0).abs() < 1e-10);
            }
            _ => panic!("Expected number"),
        }
    }

    #[test]
    fn test_e() {
        let mut ctx = Context::new();

        e(&mut ctx).unwrap();

        match ctx.stack.get(0).unwrap() {
            Object::Number { value, .. } => {
                // Euler-Mascheroni constant γ ≈ 0.5772156649...
                let euler_mascheroni = 0.5772156649015329;
                assert!((value.to_f64() - euler_mascheroni).abs() < 1e-10);
            }
            _ => panic!("Expected number"),
        }
    }

    #[test]
    fn test_alog10() {
        let mut ctx = Context::new();

        // alog10(2) = 10^2 = 100
        push_number!(ctx, Float::with_val(128, 2));
        alog10(&mut ctx).unwrap();

        match ctx.stack.get(0).unwrap() {
            Object::Number { value, .. } => {
                assert!((value.to_f64() - 100.0).abs() < 1e-10);
            }
            _ => panic!("Expected number"),
        }
    }
}
