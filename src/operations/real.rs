// SPDX-License-Identifier: GPL-3.0-only
// Real number arithmetic operations

use crate::context::Context;
use crate::error::{Error, Result};
use crate::object::Object;
use rug::ops::Pow;
use rug::{Complex, Float};

/// Addition: +
pub fn plus(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 2);

    // Handle different type combinations
    match (ctx.stack.get(1), ctx.stack.get(0)) {
        // Number + Number
        (Some(Object::Number { .. }), Some(Object::Number { .. })) => {
            // Extract base from left operand (stack position 1) before popping
            let base = match ctx.stack.get(1) {
                Some(Object::Number { base, .. }) => *base,
                _ => 10,
            };
            let (a, b) = pop_two_numbers!(ctx);
            let result = a + b;
            push_number!(ctx, result, base);
            Ok(())
        }
        // Complex + Complex
        (Some(Object::Complex { .. }), Some(Object::Complex { .. })) => {
            let b = match ctx.stack.pop().unwrap() {
                Object::Complex {
                    value,
                    re_base,
                    im_base,
                } => (value, re_base, im_base),
                _ => unreachable!(),
            };
            let a = match ctx.stack.pop().unwrap() {
                Object::Complex {
                    value,
                    re_base,
                    im_base,
                } => (value, re_base, im_base),
                _ => unreachable!(),
            };
            let result = a.0 + b.0;
            ctx.stack.push(Object::Complex {
                value: result,
                re_base: a.1,
                im_base: a.2,
            });
            Ok(())
        }
        // Number + Complex
        (Some(Object::Number { .. }), Some(Object::Complex { .. })) => {
            let b = match ctx.stack.pop().unwrap() {
                Object::Complex {
                    value,
                    re_base,
                    im_base,
                } => (value, re_base, im_base),
                _ => unreachable!(),
            };
            let a = match ctx.stack.pop().unwrap() {
                Object::Number { value, base } => {
                    let complex = Complex::with_val(
                        ctx.config.precision_bits,
                        (value, Float::new(ctx.config.precision_bits)),
                    );
                    (complex, base, base)
                }
                _ => unreachable!(),
            };
            let result = a.0 + b.0;
            ctx.stack.push(Object::Complex {
                value: result,
                re_base: a.1,
                im_base: a.2,
            });
            Ok(())
        }
        // Complex + Number
        (Some(Object::Complex { .. }), Some(Object::Number { .. })) => {
            let b = match ctx.stack.pop().unwrap() {
                Object::Number { value, base } => {
                    let complex = Complex::with_val(
                        ctx.config.precision_bits,
                        (value, Float::new(ctx.config.precision_bits)),
                    );
                    (complex, base, base)
                }
                _ => unreachable!(),
            };
            let a = match ctx.stack.pop().unwrap() {
                Object::Complex {
                    value,
                    re_base,
                    im_base,
                } => (value, re_base, im_base),
                _ => unreachable!(),
            };
            let result = a.0 + b.0;
            ctx.stack.push(Object::Complex {
                value: result,
                re_base: a.1,
                im_base: a.2,
            });
            Ok(())
        }
        // String + String (concatenation)
        (Some(Object::String(_)), Some(Object::String(_))) => {
            let b = match ctx.stack.pop().unwrap() {
                Object::String(s) => s,
                _ => unreachable!(),
            };
            let a = match ctx.stack.pop().unwrap() {
                Object::String(s) => s,
                _ => unreachable!(),
            };
            ctx.stack.push(Object::String(a + &b));
            Ok(())
        }
        _ => Err(Error::BadOperandType),
    }
}

/// Subtraction: -
pub fn minus(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 2);

    match (ctx.stack.get(1), ctx.stack.get(0)) {
        // Number - Number
        (Some(Object::Number { .. }), Some(Object::Number { .. })) => {
            // Extract base from left operand (stack position 1) before popping
            let base = match ctx.stack.get(1) {
                Some(Object::Number { base, .. }) => *base,
                _ => 10,
            };
            let (a, b) = pop_two_numbers!(ctx);
            let result = a - b;
            push_number!(ctx, result, base);
            Ok(())
        }
        // Complex - Complex
        (Some(Object::Complex { .. }), Some(Object::Complex { .. })) => {
            let b = match ctx.stack.pop().unwrap() {
                Object::Complex { value, .. } => value,
                _ => unreachable!(),
            };
            let a = match ctx.stack.pop().unwrap() {
                Object::Complex {
                    value,
                    re_base,
                    im_base,
                } => (value, re_base, im_base),
                _ => unreachable!(),
            };
            let result = a.0 - b;
            ctx.stack.push(Object::Complex {
                value: result,
                re_base: a.1,
                im_base: a.2,
            });
            Ok(())
        }
        // Number - Complex
        (Some(Object::Number { .. }), Some(Object::Complex { .. })) => {
            let b = match ctx.stack.pop().unwrap() {
                Object::Complex { value, .. } => value,
                _ => unreachable!(),
            };
            let a = match ctx.stack.pop().unwrap() {
                Object::Number { value, base } => {
                    let complex = Complex::with_val(
                        ctx.config.precision_bits,
                        (value, Float::new(ctx.config.precision_bits)),
                    );
                    (complex, base, base)
                }
                _ => unreachable!(),
            };
            let result = a.0 - b;
            ctx.stack.push(Object::Complex {
                value: result,
                re_base: a.1,
                im_base: a.2,
            });
            Ok(())
        }
        // Complex - Number
        (Some(Object::Complex { .. }), Some(Object::Number { .. })) => {
            let b = match ctx.stack.pop().unwrap() {
                Object::Number { value, .. } => Complex::with_val(
                    ctx.config.precision_bits,
                    (value, Float::new(ctx.config.precision_bits)),
                ),
                _ => unreachable!(),
            };
            let a = match ctx.stack.pop().unwrap() {
                Object::Complex {
                    value,
                    re_base,
                    im_base,
                } => (value, re_base, im_base),
                _ => unreachable!(),
            };
            let result = a.0 - b;
            ctx.stack.push(Object::Complex {
                value: result,
                re_base: a.1,
                im_base: a.2,
            });
            Ok(())
        }
        _ => Err(Error::BadOperandType),
    }
}

/// Multiplication: *
pub fn multiply(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 2);

    match (ctx.stack.get(1), ctx.stack.get(0)) {
        // Number * Number
        (Some(Object::Number { .. }), Some(Object::Number { .. })) => {
            // Extract base from left operand (stack position 1) before popping
            let base = match ctx.stack.get(1) {
                Some(Object::Number { base, .. }) => *base,
                _ => 10,
            };
            let (a, b) = pop_two_numbers!(ctx);
            let result = a * b;
            push_number!(ctx, result, base);
            Ok(())
        }
        // Complex * Complex
        (Some(Object::Complex { .. }), Some(Object::Complex { .. })) => {
            let b = match ctx.stack.pop().unwrap() {
                Object::Complex { value, .. } => value,
                _ => unreachable!(),
            };
            let a = match ctx.stack.pop().unwrap() {
                Object::Complex {
                    value,
                    re_base,
                    im_base,
                } => (value, re_base, im_base),
                _ => unreachable!(),
            };
            let result = a.0 * b;
            ctx.stack.push(Object::Complex {
                value: result,
                re_base: a.1,
                im_base: a.2,
            });
            Ok(())
        }
        // Number * Complex
        (Some(Object::Number { .. }), Some(Object::Complex { .. })) => {
            let b = match ctx.stack.pop().unwrap() {
                Object::Complex { value, .. } => value,
                _ => unreachable!(),
            };
            let a = match ctx.stack.pop().unwrap() {
                Object::Number { value, base } => {
                    let complex = Complex::with_val(
                        ctx.config.precision_bits,
                        (value, Float::new(ctx.config.precision_bits)),
                    );
                    (complex, base, base)
                }
                _ => unreachable!(),
            };
            let result = a.0 * b;
            ctx.stack.push(Object::Complex {
                value: result,
                re_base: a.1,
                im_base: a.2,
            });
            Ok(())
        }
        // Complex * Number
        (Some(Object::Complex { .. }), Some(Object::Number { .. })) => {
            let b = match ctx.stack.pop().unwrap() {
                Object::Number { value, .. } => Complex::with_val(
                    ctx.config.precision_bits,
                    (value, Float::new(ctx.config.precision_bits)),
                ),
                _ => unreachable!(),
            };
            let a = match ctx.stack.pop().unwrap() {
                Object::Complex {
                    value,
                    re_base,
                    im_base,
                } => (value, re_base, im_base),
                _ => unreachable!(),
            };
            let result = a.0 * b;
            ctx.stack.push(Object::Complex {
                value: result,
                re_base: a.1,
                im_base: a.2,
            });
            Ok(())
        }
        _ => Err(Error::BadOperandType),
    }
}

/// Division: /
pub fn divide(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 2);

    match (ctx.stack.get(1), ctx.stack.get(0)) {
        // Number / Number
        (Some(Object::Number { .. }), Some(Object::Number { .. })) => {
            // Extract base from left operand (stack position 1) before popping
            let base = match ctx.stack.get(1) {
                Some(Object::Number { base, .. }) => *base,
                _ => 10,
            };
            let (a, b) = pop_two_numbers!(ctx);

            // Division by zero produces inf or nan (IEEE 754 behavior)
            let result = a / b;
            push_number!(ctx, result, base);
            Ok(())
        }
        // Complex / Complex
        (Some(Object::Complex { .. }), Some(Object::Complex { .. })) => {
            let b = match ctx.stack.pop().unwrap() {
                Object::Complex { value, .. } => value,
                _ => unreachable!(),
            };
            let a = match ctx.stack.pop().unwrap() {
                Object::Complex {
                    value,
                    re_base,
                    im_base,
                } => (value, re_base, im_base),
                _ => unreachable!(),
            };
            let result = a.0 / b;
            ctx.stack.push(Object::Complex {
                value: result,
                re_base: a.1,
                im_base: a.2,
            });
            Ok(())
        }
        // Number / Complex
        (Some(Object::Number { .. }), Some(Object::Complex { .. })) => {
            let b = match ctx.stack.pop().unwrap() {
                Object::Complex { value, .. } => value,
                _ => unreachable!(),
            };
            let a = match ctx.stack.pop().unwrap() {
                Object::Number { value, base } => {
                    let complex = Complex::with_val(
                        ctx.config.precision_bits,
                        (value, Float::new(ctx.config.precision_bits)),
                    );
                    (complex, base, base)
                }
                _ => unreachable!(),
            };
            let result = a.0 / b;
            ctx.stack.push(Object::Complex {
                value: result,
                re_base: a.1,
                im_base: a.2,
            });
            Ok(())
        }
        // Complex / Number
        (Some(Object::Complex { .. }), Some(Object::Number { .. })) => {
            let b = match ctx.stack.pop().unwrap() {
                Object::Number { value, .. } => Complex::with_val(
                    ctx.config.precision_bits,
                    (value, Float::new(ctx.config.precision_bits)),
                ),
                _ => unreachable!(),
            };
            let a = match ctx.stack.pop().unwrap() {
                Object::Complex {
                    value,
                    re_base,
                    im_base,
                } => (value, re_base, im_base),
                _ => unreachable!(),
            };
            let result = a.0 / b;
            ctx.stack.push(Object::Complex {
                value: result,
                re_base: a.1,
                im_base: a.2,
            });
            Ok(())
        }
        _ => Err(Error::BadOperandType),
    }
}

/// Negation: neg or chs (change sign)
/// Works on both real numbers and complex numbers (negates both real and imaginary parts)
pub fn neg(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    let obj = ctx.stack.pop().unwrap();

    match obj {
        Object::Number { value, base } => {
            let result = -value;
            ctx.stack.push(Object::Number {
                value: result,
                base,
            });
            Ok(())
        }
        Object::Complex {
            value,
            re_base,
            im_base,
        } => {
            // Negate both real and imaginary parts
            let result = -value;
            ctx.stack.push(Object::Complex {
                value: result,
                re_base,
                im_base,
            });
            Ok(())
        }
        _ => {
            ctx.stack.push(obj);
            Err(Error::BadOperandType)
        }
    }
}

/// Inverse: inv (1/x or 1/z)
/// For complex numbers: 1/z = conj(z) / |z|²
pub fn inv(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    let obj = ctx.stack.pop().unwrap();

    match obj {
        Object::Number { value, base } => {
            // Division by zero produces inf (IEEE 754 behavior)
            let result = Float::with_val(ctx.config.precision_bits, 1) / value;
            push_number!(ctx, result, base);
            Ok(())
        }
        Object::Complex {
            value,
            re_base,
            im_base,
        } => {
            // 1/z = conj(z) / |z|²
            let result = value.recip();
            ctx.stack.push(Object::Complex {
                value: result,
                re_base,
                im_base,
            });
            Ok(())
        }
        _ => {
            ctx.stack.push(obj);
            Err(Error::BadOperandType)
        }
    }
}

/// Power: ^ or pow
/// Supports real^real, real^complex, complex^real, and complex^complex
pub fn power(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 2);

    let exponent = ctx.stack.pop().unwrap();
    let base_obj = ctx.stack.pop().unwrap();

    match (base_obj, exponent) {
        // Real ^ Real
        (
            Object::Number {
                value: a,
                base: base_a,
            },
            Object::Number { value: b, base: _ },
        ) => {
            // Check if base is negative and exponent is not an integer
            // In this case, the result will be complex
            if a < 0 && !b.is_integer() {
                // Convert to complex and compute
                let base_complex = Complex::with_val(
                    ctx.config.precision_bits,
                    (a.clone(), Float::with_val(ctx.config.precision_bits, 0)),
                );
                let exp_complex = Complex::with_val(
                    ctx.config.precision_bits,
                    (b, Float::with_val(ctx.config.precision_bits, 0)),
                );
                let result = base_complex.pow(&exp_complex);
                ctx.stack.push(Object::Complex {
                    value: result,
                    re_base: base_a,
                    im_base: base_a,
                });
            } else {
                let result = a.pow(b);
                push_number!(ctx, result, base_a);
            }
            Ok(())
        }

        // Complex ^ Real
        (
            Object::Complex {
                value: z,
                re_base,
                im_base,
            },
            Object::Number {
                value: exp,
                base: _,
            },
        ) => {
            // z^exp using rug's Complex pow method
            let result = z.pow(exp);
            ctx.stack.push(Object::Complex {
                value: result,
                re_base,
                im_base,
            });
            Ok(())
        }

        // Real ^ Complex (might produce complex result)
        (
            Object::Number {
                value: base,
                base: base_num,
            },
            Object::Complex {
                value: exp,
                re_base: _,
                im_base,
            },
        ) => {
            // Convert real to complex and compute
            let base_complex = Complex::with_val(
                ctx.config.precision_bits,
                (base, Float::with_val(ctx.config.precision_bits, 0)),
            );
            let result = base_complex.pow(&exp);
            ctx.stack.push(Object::Complex {
                value: result,
                re_base: base_num,
                im_base,
            });
            Ok(())
        }

        // Complex ^ Complex
        (
            Object::Complex {
                value: base,
                re_base,
                im_base,
            },
            Object::Complex { value: exp, .. },
        ) => {
            // z1^z2 = exp(z2 * ln(z1))
            let result = base.pow(&exp);
            ctx.stack.push(Object::Complex {
                value: result,
                re_base,
                im_base,
            });
            Ok(())
        }

        (base_obj, exp) => {
            ctx.stack.push(base_obj);
            ctx.stack.push(exp);
            Err(Error::BadOperandType)
        }
    }
}

/// Square root: sqrt
/// For complex numbers, returns the principal square root
/// For negative real numbers, returns a complex result (0, sqrt(|x|))
pub fn sqrt(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    let obj = ctx.stack.pop().unwrap();

    match obj {
        Object::Number { value, base } => {
            if value < 0 {
                // Negative real: sqrt(-x) = i * sqrt(x)
                let abs_val = value.abs();
                let sqrt_abs = abs_val.sqrt();
                let zero = Float::with_val(ctx.config.precision_bits, 0);
                let complex_result = Complex::with_val(ctx.config.precision_bits, (zero, sqrt_abs));
                ctx.stack.push(Object::Complex {
                    value: complex_result,
                    re_base: base,
                    im_base: base,
                });
            } else {
                let result = value.sqrt();
                push_number!(ctx, result, base);
            }
            Ok(())
        }
        Object::Complex {
            value,
            re_base,
            im_base,
        } => {
            let result = value.sqrt();
            ctx.stack.push(Object::Complex {
                value: result,
                re_base,
                im_base,
            });
            Ok(())
        }
        _ => {
            ctx.stack.push(obj);
            Err(Error::BadOperandType)
        }
    }
}

/// Square: sq (x^2 or z^2)
pub fn sq(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    let obj = ctx.stack.pop().unwrap();

    match obj {
        Object::Number { value, base } => {
            let result = value.square();
            push_number!(ctx, result, base);
            Ok(())
        }
        Object::Complex {
            value,
            re_base,
            im_base,
        } => {
            let result = value.square();
            ctx.stack.push(Object::Complex {
                value: result,
                re_base,
                im_base,
            });
            Ok(())
        }
        _ => {
            ctx.stack.push(obj);
            Err(Error::BadOperandType)
        }
    }
}

/// Absolute value: abs
/// For complex numbers, returns the modulus (magnitude)
pub fn abs(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    let obj = ctx.stack.pop().unwrap();

    match obj {
        Object::Number { value, base } => {
            let result = value.abs();
            push_number!(ctx, result, base);
            Ok(())
        }
        Object::Complex { value, re_base, .. } => {
            // For complex numbers, abs returns the modulus (magnitude)
            // Compute magnitude using the context's current precision
            let re = value.real();
            let im = value.imag();
            let re_squared = Float::with_val(ctx.config.precision_bits, re).square();
            let im_squared = Float::with_val(ctx.config.precision_bits, im).square();
            let magnitude = (re_squared + im_squared).sqrt();
            push_number!(ctx, magnitude, re_base);
            Ok(())
        }
        _ => {
            ctx.stack.push(obj);
            Err(Error::BadOperandType)
        }
    }
}

/// Sign: sign (returns -1, 0, or 1 for reals; z/|z| for complex)
/// For complex numbers, returns the complex number divided by its norm
pub fn sign(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    let obj = ctx.stack.pop().unwrap();

    match obj {
        Object::Number { value, base } => {
            let result = if value.is_sign_positive() {
                if value == 0 {
                    Float::with_val(ctx.config.precision_bits, 0)
                } else {
                    Float::with_val(ctx.config.precision_bits, 1)
                }
            } else {
                Float::with_val(ctx.config.precision_bits, -1)
            };
            push_number!(ctx, result, base);
            Ok(())
        }
        Object::Complex {
            value,
            re_base,
            im_base,
        } => {
            // For complex numbers, sign(z) = z / |z|
            let magnitude = value.clone().abs();
            let result = value / magnitude;
            ctx.stack.push(Object::Complex {
                value: result,
                re_base,
                im_base,
            });
            Ok(())
        }
        _ => {
            ctx.stack.push(obj);
            Err(Error::BadOperandType)
        }
    }
}

/// Minimum: min
pub fn min(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 2);
    arg_must_be!(ctx, 0, Number);
    arg_must_be!(ctx, 1, Number);

    // Extract base from left operand (stack position 1) before popping
    let base = match ctx.stack.get(1) {
        Some(Object::Number { base, .. }) => *base,
        _ => 10,
    };

    let (a, b) = pop_two_numbers!(ctx);
    let result = a.min(&b);
    push_number!(ctx, result, base);
    Ok(())
}

/// Maximum: max
pub fn max(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 2);
    arg_must_be!(ctx, 0, Number);
    arg_must_be!(ctx, 1, Number);

    // Extract base from left operand (stack position 1) before popping
    let base = match ctx.stack.get(1) {
        Some(Object::Number { base, .. }) => *base,
        _ => 10,
    };

    let (a, b) = pop_two_numbers!(ctx);
    let result = a.max(&b);
    push_number!(ctx, result, base);
    Ok(())
}

/// Modulo: mod or %
pub fn modulo(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 2);
    arg_must_be!(ctx, 0, Number);
    arg_must_be!(ctx, 1, Number);

    // Extract base from left operand (stack position 1) before popping
    let base = match ctx.stack.get(1) {
        Some(Object::Number { base, .. }) => *base,
        _ => 10,
    };

    let (a, b) = pop_two_numbers!(ctx);

    if b == 0 {
        return Err(Error::DivByZero);
    }

    // Compute a % b (remainder)
    let result = a % b;
    push_number!(ctx, result, base);
    Ok(())
}

/// Percent: % (compute percentage)
/// For "2 30 %", computes 30% of 2 = 2 * (30/100) = 0.6
pub fn percent(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 2);
    arg_must_be!(ctx, 0, Number);
    arg_must_be!(ctx, 1, Number);

    // Extract base from left operand (stack position 1) before popping
    let base = match ctx.stack.get(1) {
        Some(Object::Number { base, .. }) => *base,
        _ => 10,
    };

    let (a, b) = pop_two_numbers!(ctx);

    // Compute a * (b / 100)
    let result = a * (b / 100.0);
    push_number!(ctx, result, base);
    Ok(())
}

/// Inverse percent: %CH (what percentage of a gives b)
/// For "2 0.6 %CH", computes what percentage of 2 gives 0.6 = (0.6/2)*100 = 30
pub fn percent_change(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 2);
    arg_must_be!(ctx, 0, Number);
    arg_must_be!(ctx, 1, Number);

    // Extract base from left operand (stack position 1) before popping
    let base = match ctx.stack.get(1) {
        Some(Object::Number { base, .. }) => *base,
        _ => 10,
    };

    let (a, b) = pop_two_numbers!(ctx);

    if a == 0 {
        return Err(Error::DivByZero);
    }

    // Inverse percent: (b / a) * 100
    let result = (b / a) * 100;
    push_number!(ctx, result, base);
    Ok(())
}

/// Factorial: fact
pub fn factorial(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);
    arg_must_be!(ctx, 0, Number);

    // Extract base before popping
    let (value, base) = match ctx.stack.pop().unwrap() {
        Object::Number { value, base } => (value, base),
        _ => unreachable!(),
    };

    // Convert to integer via f64
    let f = value.to_f64();
    if !f.is_finite() || f < 0.0 || f.fract() != 0.0 {
        return Err(Error::BadOperandType);
    }

    let n = f as u32;
    if n > 100000 {
        return Err(Error::OutOfRange);
    }

    // Compute factorial
    let mut result = Float::with_val(ctx.config.precision_bits, 1);
    for i in 2..=n {
        result *= i;
    }

    push_number!(ctx, result, base);
    Ok(())
}

/// Floor: floor (round down)
pub fn floor(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);
    arg_must_be!(ctx, 0, Number);

    // Extract base before popping
    let (value, base) = match ctx.stack.pop().unwrap() {
        Object::Number { value, base } => (value, base),
        _ => unreachable!(),
    };

    let result = value.floor();
    push_number!(ctx, result, base);
    Ok(())
}

/// Ceiling: ceil (round up)
pub fn ceil(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);
    arg_must_be!(ctx, 0, Number);

    // Extract base before popping
    let (value, base) = match ctx.stack.pop().unwrap() {
        Object::Number { value, base } => (value, base),
        _ => unreachable!(),
    };

    let result = value.ceil();
    push_number!(ctx, result, base);
    Ok(())
}

/// Integer part: ip
pub fn integer_part(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);
    arg_must_be!(ctx, 0, Number);

    // Extract base before popping
    let (value, base) = match ctx.stack.pop().unwrap() {
        Object::Number { value, base } => (value, base),
        _ => unreachable!(),
    };

    let result = value.trunc();
    push_number!(ctx, result, base);
    Ok(())
}

/// Fractional part: fp
pub fn fractional_part(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);
    arg_must_be!(ctx, 0, Number);

    // Extract base before popping
    let (value, base) = match ctx.stack.pop().unwrap() {
        Object::Number { value, base } => (value, base),
        _ => unreachable!(),
    };

    let result = value.clone() - value.trunc();
    push_number!(ctx, result, base);
    Ok(())
}

/// Mantissa: mant
pub fn mantissa(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);
    arg_must_be!(ctx, 0, Number);

    // Extract base before popping
    let (value, base) = match ctx.stack.pop().unwrap() {
        Object::Number { value, base } => (value, base),
        _ => unreachable!(),
    };

    // Get mantissa in base 10 (range [1, 10) for positive values)
    if !value.is_finite() {
        // Special value (inf, -inf, nan) - return error 4 (OutOfRange)
        return Err(Error::OutOfRange);
    }

    if value.is_zero() {
        // Zero: mantissa is 0
        push_number!(ctx, value, base);
        Ok(())
    } else {
        // Compute base 10 exponent: floor(log10(abs(value)))
        let abs_val = value.clone().abs();
        let log10_val = abs_val.log10();
        let exponent_float = log10_val.floor();

        // Mantissa = value / 10^exponent
        let ten = Float::with_val(ctx.config.precision_bits, 10);
        let power_of_ten = ten.pow(&exponent_float);
        let mantissa = value / power_of_ten;

        push_number!(ctx, mantissa, base);
        Ok(())
    }
}

/// Exponent: xpon (base 10 exponent)
pub fn exponent(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);
    arg_must_be!(ctx, 0, Number);

    // Extract base before popping
    let (value, base) = match ctx.stack.pop().unwrap() {
        Object::Number { value, base } => (value, base),
        _ => unreachable!(),
    };

    // Get base 10 exponent
    if !value.is_finite() {
        // Special value (inf, -inf, nan) - return error 4 (OutOfRange)
        return Err(Error::OutOfRange);
    }

    if value.is_zero() {
        // Zero: exponent is 0
        push_number!(ctx, Float::with_val(ctx.config.precision_bits, 0), base);
        Ok(())
    } else {
        // Compute base 10 exponent: floor(log10(abs(value)))
        let abs_val = value.abs();
        let log10_val = abs_val.log10();
        let exponent = log10_val.floor();
        push_number!(ctx, exponent, base);
        Ok(())
    }
}

/// Bitwise AND: &
pub fn bitwise_and(ctx: &mut Context) -> Result<()> {
    use rug::Integer;

    min_arguments!(ctx, 2);
    arg_must_be!(ctx, 0, Number);
    arg_must_be!(ctx, 1, Number);

    // Extract base from left operand (stack position 1) before popping
    let base = match ctx.stack.get(1) {
        Some(Object::Number { base, .. }) => *base,
        _ => 10,
    };

    let (a, b) = pop_two_numbers!(ctx);

    // Convert to integers (truncate)
    let a_int = a.to_integer().unwrap_or(Integer::new());
    let b_int = b.to_integer().unwrap_or(Integer::new());

    // Perform bitwise AND
    let result_int = a_int & b_int;

    // Convert back to Float
    let result = Float::with_val(ctx.config.precision_bits, &result_int);
    push_number!(ctx, result, base);

    Ok(())
}

/// Bitwise OR: |
pub fn bitwise_or(ctx: &mut Context) -> Result<()> {
    use rug::Integer;

    min_arguments!(ctx, 2);
    arg_must_be!(ctx, 0, Number);
    arg_must_be!(ctx, 1, Number);

    // Extract base from left operand (stack position 1) before popping
    let base = match ctx.stack.get(1) {
        Some(Object::Number { base, .. }) => *base,
        _ => 10,
    };

    let (a, b) = pop_two_numbers!(ctx);

    // Convert to integers (truncate)
    let a_int = a.to_integer().unwrap_or(Integer::new());
    let b_int = b.to_integer().unwrap_or(Integer::new());

    // Perform bitwise OR
    let result_int = a_int | b_int;

    // Convert back to Float
    let result = Float::with_val(ctx.config.precision_bits, &result_int);
    push_number!(ctx, result, base);

    Ok(())
}

/// Bitwise XOR: ^
pub fn bitwise_xor(ctx: &mut Context) -> Result<()> {
    use rug::Integer;

    min_arguments!(ctx, 2);
    arg_must_be!(ctx, 0, Number);
    arg_must_be!(ctx, 1, Number);

    // Extract base from left operand (stack position 1) before popping
    let base = match ctx.stack.get(1) {
        Some(Object::Number { base, .. }) => *base,
        _ => 10,
    };

    let (a, b) = pop_two_numbers!(ctx);

    // Convert to integers (truncate)
    let a_int = a.to_integer().unwrap_or(Integer::new());
    let b_int = b.to_integer().unwrap_or(Integer::new());

    // Perform bitwise XOR
    let result_int = a_int ^ b_int;

    // Convert back to Float
    let result = Float::with_val(ctx.config.precision_bits, &result_int);
    push_number!(ctx, result, base);

    Ok(())
}

/// Bitwise NOT: ~
/// Inverts only the significant bits (bit length of the number)
/// Example: 0b110 (6) -> 0b001 (1)
pub fn bitwise_not(ctx: &mut Context) -> Result<()> {
    use rug::Integer;

    min_arguments!(ctx, 1);
    arg_must_be!(ctx, 0, Number);

    // Extract value and base
    let (value, base) = match ctx.stack.pop().unwrap() {
        Object::Number { value, base } => (value, base),
        _ => unreachable!(),
    };

    // Convert to integer (truncate)
    let int_val = value.to_integer().unwrap_or(Integer::new());

    // Get the number of significant bits
    let bit_count = int_val.significant_bits();

    if bit_count == 0 {
        // Special case: 0 inverts to 0 (no bits to invert)
        push_number!(ctx, Float::with_val(ctx.config.precision_bits, 0), base);
    } else {
        // Create a mask with all 1s for the significant bits
        let mask = (Integer::from(1) << bit_count) - 1;
        // XOR with mask to invert only significant bits
        let result_int = int_val ^ mask;
        let result = Float::with_val(ctx.config.precision_bits, &result_int);
        push_number!(ctx, result, base);
    }

    Ok(())
}
