// SPDX-License-Identifier: GPL-3.0-only
// Complex number operations

use crate::context::Context;
use crate::error::{Error, Result};
use crate::object::Object;
use rug::float::Constant;
use rug::{Complex, Float};

/// RE: re (extract real part of complex number)
/// Stack: complex -> real
pub fn re(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    match ctx.stack.pop() {
        Some(Object::Complex { value, re_base, .. }) => {
            let real_part = value.real().clone();
            push_number!(ctx, real_part, re_base);
            Ok(())
        }
        Some(obj @ Object::Number { .. }) => {
            // Real number: just return it (re of real is itself)
            ctx.stack.push(obj);
            Ok(())
        }
        _ => Err(Error::BadOperandType),
    }
}

/// IM: im (extract imaginary part of complex number)
/// Stack: complex -> imag
pub fn im(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    match ctx.stack.pop() {
        Some(Object::Complex { value, im_base, .. }) => {
            let imag_part = value.imag().clone();
            push_number!(ctx, imag_part, im_base);
            Ok(())
        }
        Some(Object::Number { .. }) => {
            // Real number: imaginary part is 0
            push_number!(ctx, Float::with_val(ctx.config.precision_bits, 0));
            Ok(())
        }
        _ => Err(Error::BadOperandType),
    }
}

/// ARG: arg (get argument/phase of complex number in radians)
/// Stack: complex -> angle
pub fn arg(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    match ctx.stack.pop() {
        Some(Object::Complex { value, re_base, .. }) => {
            // Extract real and imaginary parts
            let re = value.real();
            let im = value.imag();

            // Compute atan2(im, re) using the context's current precision
            let angle = Float::with_val(ctx.config.precision_bits, im)
                .atan2(&Float::with_val(ctx.config.precision_bits, re));
            push_number!(ctx, angle, re_base);
            Ok(())
        }
        Some(Object::Number { value, base }) => {
            // Real number: arg is 0 if positive, pi if negative
            let angle = if value >= 0 {
                Float::with_val(ctx.config.precision_bits, 0)
            } else {
                Float::with_val(ctx.config.precision_bits, Constant::Pi)
            };
            push_number!(ctx, angle, base);
            Ok(())
        }
        _ => Err(Error::BadOperandType),
    }
}

/// CONJ: conj (complex conjugate)
/// Stack: complex -> complex_conjugate
pub fn conj(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    match ctx.stack.pop() {
        Some(Object::Complex {
            value,
            re_base,
            im_base,
        }) => {
            let conjugate = value.conj();
            ctx.stack.push(Object::Complex {
                value: conjugate,
                re_base,
                im_base,
            });
            Ok(())
        }
        Some(obj @ Object::Number { .. }) => {
            // Real number: conjugate is itself
            ctx.stack.push(obj);
            Ok(())
        }
        _ => Err(Error::BadOperandType),
    }
}

/// C->R: c->r (convert complex to two reals: real imag)
/// Stack: complex -> real imag
pub fn c_to_r(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    match ctx.stack.pop() {
        Some(Object::Complex {
            value,
            re_base,
            im_base,
        }) => {
            let real_part = value.real().clone();
            let imag_part = value.imag().clone();

            push_number!(ctx, real_part, re_base);
            push_number!(ctx, imag_part, im_base);
            Ok(())
        }
        _ => Err(Error::BadOperandType),
    }
}

/// R->C: r->c (convert two reals to complex)
/// Stack: real imag -> complex
pub fn r_to_c(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 2);
    arg_must_be!(ctx, 0, Number);
    arg_must_be!(ctx, 1, Number);

    let (imag, im_base) = match ctx.stack.pop() {
        Some(Object::Number { value, base }) => (value, base),
        _ => unreachable!(),
    };

    let (real, re_base) = match ctx.stack.pop() {
        Some(Object::Number { value, base }) => (value, base),
        _ => unreachable!(),
    };

    let complex_value = Complex::with_val(ctx.config.precision_bits, (real, imag));

    ctx.stack.push(Object::Complex {
        value: complex_value,
        re_base,
        im_base,
    });

    Ok(())
}

/// P->R: p->r (convert polar to rectangular: magnitude angle -> complex)
/// Stack: magnitude angle_radians -> complex
pub fn p_to_r(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 2);
    arg_must_be!(ctx, 0, Number);
    arg_must_be!(ctx, 1, Number);

    let (angle, angle_base) = match ctx.stack.pop() {
        Some(Object::Number { value, base }) => (value, base),
        _ => unreachable!(),
    };

    let (magnitude, mag_base) = match ctx.stack.pop() {
        Some(Object::Number { value, base }) => (value, base),
        _ => unreachable!(),
    };

    // Convert polar to rectangular: x = r*cos(θ), y = r*sin(θ)
    let cos_angle = angle.clone().cos();
    let sin_angle = angle.sin();

    let real = magnitude.clone() * cos_angle;
    let imag = magnitude * sin_angle;

    let complex_value = Complex::with_val(ctx.config.precision_bits, (real, imag));

    ctx.stack.push(Object::Complex {
        value: complex_value,
        re_base: mag_base,
        im_base: angle_base,
    });

    Ok(())
}

/// R->P: r->p (convert rectangular to polar: complex -> (magnitude, angle))
/// Stack: complex -> complex(magnitude, angle_radians)
/// Returns a complex number where the real part is the magnitude and imaginary part is the angle
pub fn r_to_p(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    match ctx.stack.pop() {
        Some(Object::Complex {
            value,
            re_base,
            im_base,
        }) => {
            // Extract real and imaginary parts
            let re = value.real();
            let im = value.imag();

            // Compute magnitude and angle using the context's current precision
            let re_squared = Float::with_val(ctx.config.precision_bits, re).square();
            let im_squared = Float::with_val(ctx.config.precision_bits, im).square();
            let magnitude = (re_squared + im_squared).sqrt();

            let angle = Float::with_val(ctx.config.precision_bits, im)
                .atan2(&Float::with_val(ctx.config.precision_bits, re));

            // Return as complex (magnitude, angle)
            let polar = Complex::with_val(ctx.config.precision_bits, (magnitude, angle));
            ctx.stack.push(Object::Complex {
                value: polar,
                re_base,
                im_base,
            });
            Ok(())
        }
        Some(Object::Number { value, base }) => {
            // Real number: magnitude is abs(value), angle is 0 or pi
            let magnitude = Float::with_val(ctx.config.precision_bits, value.abs_ref());
            let angle = if value >= 0 {
                Float::with_val(ctx.config.precision_bits, 0)
            } else {
                Float::with_val(ctx.config.precision_bits, Constant::Pi)
            };

            // Return as complex (magnitude, angle)
            let polar = Complex::with_val(ctx.config.precision_bits, (magnitude, angle));
            ctx.stack.push(Object::Complex {
                value: polar,
                re_base: base,
                im_base: base,
            });
            Ok(())
        }
        _ => Err(Error::BadOperandType),
    }
}

/// ABS: abs for complex numbers (already implemented in real.rs, but extended here)
/// Stack: complex -> magnitude
pub fn abs_complex(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    match ctx.stack.get(0) {
        Some(Object::Complex { .. }) => {
            let obj = ctx.stack.pop().unwrap();
            if let Object::Complex { value, re_base, .. } = obj {
                let magnitude = value.abs().real().clone();
                push_number!(ctx, magnitude, re_base);
                Ok(())
            } else {
                unreachable!()
            }
        }
        Some(Object::Number { .. }) => {
            // Delegate to real.rs abs
            crate::operations::real::abs(ctx)
        }
        _ => Err(Error::BadOperandType),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rug::Float;

    #[test]
    fn test_r_to_c() {
        let mut ctx = Context::new();

        // Create complex from 3 + 4i
        push_number!(ctx, Float::with_val(128, 3));
        push_number!(ctx, Float::with_val(128, 4));

        r_to_c(&mut ctx).unwrap();

        assert_eq!(ctx.stack.len(), 1);
        match ctx.stack.get(0).unwrap() {
            Object::Complex { value, .. } => {
                assert_eq!(value.real().to_f64(), 3.0);
                assert_eq!(value.imag().to_f64(), 4.0);
            }
            _ => panic!("Expected complex number"),
        }
    }

    #[test]
    fn test_c_to_r() {
        let mut ctx = Context::new();

        // Create complex 3 + 4i
        let complex_value =
            Complex::with_val(128, (Float::with_val(128, 3), Float::with_val(128, 4)));
        ctx.stack.push(Object::Complex {
            value: complex_value,
            re_base: 10,
            im_base: 10,
        });

        c_to_r(&mut ctx).unwrap();

        assert_eq!(ctx.stack.len(), 2);
        match ctx.stack.get(1).unwrap() {
            Object::Number { value, .. } => assert_eq!(value.to_f64(), 3.0),
            _ => panic!("Expected number"),
        }
        match ctx.stack.get(0).unwrap() {
            Object::Number { value, .. } => assert_eq!(value.to_f64(), 4.0),
            _ => panic!("Expected number"),
        }
    }

    #[test]
    fn test_re_im() {
        let mut ctx = Context::new();

        // Create complex 5 + 12i
        let complex_value =
            Complex::with_val(128, (Float::with_val(128, 5), Float::with_val(128, 12)));
        ctx.stack.push(Object::Complex {
            value: complex_value,
            re_base: 10,
            im_base: 10,
        });

        // Test re
        ctx.stack.push(ctx.stack.get(0).unwrap().clone());
        re(&mut ctx).unwrap();
        match ctx.stack.get(0).unwrap() {
            Object::Number { value, .. } => assert_eq!(value.to_f64(), 5.0),
            _ => panic!("Expected number"),
        }
        ctx.stack.pop();

        // Test im
        im(&mut ctx).unwrap();
        match ctx.stack.get(0).unwrap() {
            Object::Number { value, .. } => assert_eq!(value.to_f64(), 12.0),
            _ => panic!("Expected number"),
        }
    }
}
