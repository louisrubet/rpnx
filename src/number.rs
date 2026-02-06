// SPDX-License-Identifier: GPL-3.0-only
// Number formatting and configuration

use rug::ops::Pow;
use rug::Float;

/// Display mode for numbers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayMode {
    /// Standard mode (auto-select between fixed and scientific)
    Std,
    /// Fixed-point notation
    Fix,
    /// Scientific notation
    Sci,
}

impl Default for DisplayMode {
    fn default() -> Self {
        DisplayMode::Std
    }
}

/// Number formatting configuration
#[derive(Debug, Clone)]
pub struct NumberConfig {
    /// Display mode (std, fix, sci)
    pub mode: DisplayMode,

    /// Precision in bits for MPFR (default 128)
    pub precision_bits: u32,

    /// Number of decimal digits to display (default 38)
    pub digits: i32,
}

impl NumberConfig {
    pub fn new() -> Self {
        Self {
            mode: DisplayMode::Std,
            precision_bits: 128, // kMpfrDefaultPrecBits
            digits: 38,          // kDefaultDecimalDigits
        }
    }

    /// Create a Float with the configured precision
    pub fn new_float(&self) -> Float {
        Float::with_val(self.precision_bits, 0)
    }
}

impl Default for NumberConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Format a number for display
pub fn format_number(value: &Float, base: u8, mode: DisplayMode, digits: i32) -> String {
    // Special values (inf, nan) are always displayed without base prefixes
    if value.is_nan() {
        return "nan".to_string();
    }
    if value.is_infinite() {
        return if value.is_sign_negative() {
            "-inf".to_string()
        } else {
            "inf".to_string()
        };
    }

    match base {
        10 => {
            // Decimal formatting
            match mode {
                DisplayMode::Std => {
                    // Standard mode: show significant digits, removing trailing zeros
                    format_std(value, digits as usize)
                }
                DisplayMode::Fix => {
                    // Fixed mode: show exactly N decimal places
                    format_fixed(value, digits as usize)
                }
                DisplayMode::Sci => {
                    // Scientific notation with explicit +/- and zero-padded exponent
                    format_scientific(value, digits as usize)
                }
            }
        }
        2 => {
            // Binary formatting: -0b... not 0b-...
            let base_str = format_base(value, 2);
            if base_str.starts_with('-') {
                format!("-0b{}", &base_str[1..])
            } else {
                format!("0b{}", base_str)
            }
        }
        16 => {
            // Hexadecimal formatting: -0x... not 0x-...
            let base_str = format_base(value, 16);
            if base_str.starts_with('-') {
                format!("-0x{}", &base_str[1..])
            } else {
                format!("0x{}", base_str)
            }
        }
        _ => {
            // Other bases (2-62): -Nb... not Nb-...
            let base_str = format_base(value, base as i32);
            if base_str.starts_with('-') {
                format!("-{}b{}", base, &base_str[1..])
            } else {
                format!("{}b{}", base, base_str)
            }
        }
    }
}

/// Format a Float in standard mode with intelligent precision
/// Removes trailing zeros and unnecessary decimal points
fn format_std(value: &Float, max_digits: usize) -> String {
    // Handle special values
    if value.is_nan() {
        return "nan".to_string();
    }
    if value.is_infinite() {
        return if value.is_sign_negative() {
            "-inf".to_string()
        } else {
            "inf".to_string()
        };
    }

    // Check if the value is an integer
    if value.is_integer() {
        // Format as integer (no decimal point)
        return value.to_integer().unwrap().to_string();
    }

    // Get the number of significant decimal digits
    // If max_digits is 0, use full precision from bit count
    // For 128 bits: roughly 128 * log10(2) ≈ 38.5 decimal digits
    let decimal_digits = if max_digits == 0 {
        ((value.prec() as f64) * 0.30102999566398119).floor() as usize
    } else {
        max_digits
    };

    // Get string representation with specified precision
    let s = value.to_string_radix_round(10, Some(decimal_digits), rug::float::Round::Nearest);

    // Convert scientific notation to decimal if needed
    let decimal_str = if s.contains('e') || s.contains('E') {
        scientific_to_decimal(&s, decimal_digits)
    } else {
        s
    };

    // Remove trailing zeros if there's a decimal point
    if decimal_str.contains('.') {
        let trimmed = decimal_str.trim_end_matches('0');
        let trimmed = trimmed.trim_end_matches('.');
        trimmed.to_string()
    } else {
        decimal_str
    }
}

/// Convert scientific notation string to decimal notation
/// e.g., "3.333...e-1" -> "0.3333..."
fn scientific_to_decimal(s: &str, _max_digits: usize) -> String {
    // Parse the scientific notation
    let parts: Vec<&str> = if s.contains('e') {
        s.split('e').collect()
    } else {
        s.split('E').collect()
    };

    if parts.len() != 2 {
        return s.to_string();
    }

    let mantissa_str = parts[0];
    let exponent: i32 = match parts[1].parse() {
        Ok(exp) => exp,
        Err(_) => return s.to_string(),
    };

    // Remove sign and decimal point from mantissa to get digits
    let is_negative = mantissa_str.starts_with('-');
    let mantissa_clean = mantissa_str.trim_start_matches('-').replace('.', "");

    // Build the result based on exponent
    let result = if exponent >= 0 {
        // Positive exponent: shift decimal right
        let shift = exponent as usize;
        if shift < mantissa_clean.len() {
            // Insert decimal point within the mantissa
            let mut result = String::new();
            result.push_str(&mantissa_clean[..=shift]);
            result.push('.');
            result.push_str(&mantissa_clean[shift + 1..]);
            result
        } else {
            // Append zeros
            let len = mantissa_clean.len();
            let mut result = mantissa_clean;
            for _ in 0..(shift - len + 1) {
                result.push('0');
            }
            result
        }
    } else {
        // Negative exponent: shift decimal left
        let shift = (-exponent) as usize;
        let mut result = String::from("0.");
        for _ in 0..shift.saturating_sub(1) {
            result.push('0');
        }
        result.push_str(&mantissa_clean);
        result
    };

    // Add negative sign if needed
    if is_negative {
        format!("-{}", result)
    } else {
        result
    }
}

/// Format a Float in fixed-point notation with exactly N decimal places
fn format_fixed(value: &Float, decimal_places: usize) -> String {
    use std::fmt::Write;

    // Handle special values
    if value.is_nan() {
        return "nan".to_string();
    }
    if value.is_infinite() {
        return if value.is_sign_negative() {
            "-inf".to_string()
        } else {
            "inf".to_string()
        };
    }

    // Round the value to the specified number of decimal places
    let scale_factor = Float::with_val(value.prec(), 10u32).pow(decimal_places as u32);

    // Scale up, round, scale down
    let scaled = Float::with_val(value.prec(), value * &scale_factor);
    let rounded = Float::with_val(value.prec(), scaled.round());
    let result_val = Float::with_val(value.prec(), rounded / &scale_factor);

    // Extract integer and fractional parts
    let is_negative = result_val.is_sign_negative();
    let abs_val = result_val.abs();
    let integer_part = Float::with_val(value.prec(), abs_val.clone().floor());
    let fractional_part = Float::with_val(value.prec(), abs_val - &integer_part);

    // Get integer part as string (convert to Integer to avoid f64 precision loss)
    let int_str = if let Some(int_val) = integer_part.to_integer() {
        int_val.to_string()
    } else {
        // Fallback for very large numbers
        format!("{}", integer_part.to_f64() as i64)
    };

    // Get fractional digits by multiplying by 10^decimal_places and rounding
    let frac_scaled = Float::with_val(value.prec(), fractional_part * &scale_factor);
    let frac_rounded = Float::with_val(value.prec(), frac_scaled.round());
    let frac_int = if let Some(frac_val) = frac_rounded.to_integer() {
        frac_val
    } else {
        // Fallback for very large numbers
        use rug::Integer;
        Integer::from(frac_rounded.to_f64() as u64)
    };

    // Assemble the result
    let mut output = String::new();
    if is_negative {
        output.push('-');
    }

    if decimal_places == 0 {
        // No decimal point for 0 decimal places
        write!(&mut output, "{}", int_str).unwrap();
    } else {
        // Format fractional part with leading zeros if needed
        let frac_str = format!("{:0width$}", frac_int, width = decimal_places);
        write!(&mut output, "{}.{}", int_str, frac_str).unwrap();
    }

    output
}

/// Format a Float in scientific notation with explicit +/- and zero-padded exponent
/// Example: 1.000000000000e+00 instead of 1.0e0
fn format_scientific(value: &Float, decimal_places: usize) -> String {
    // Handle special values
    if value.is_nan() {
        return "nan".to_string();
    }
    if value.is_infinite() {
        return if value.is_sign_negative() {
            "-inf".to_string()
        } else {
            "inf".to_string()
        };
    }
    if value.is_zero() {
        return format!("0.{}e+00", "0".repeat(decimal_places));
    }

    // Get sign
    let is_negative = value.is_sign_negative();
    let abs_val = value.clone().abs();

    // Calculate exponent (base 10)
    let log10 = abs_val.clone().log10();
    let exponent = log10.to_f64().floor() as i32;

    // Calculate mantissa: value / 10^exponent
    let ten = Float::with_val(value.prec(), 10);
    let scale = Float::with_val(value.prec(), ten.pow(exponent));
    let mantissa = Float::with_val(value.prec(), abs_val / scale);

    // Format mantissa with requested decimal places
    let mantissa_str = format!("{:.prec$}", mantissa.to_f64(), prec = decimal_places);

    // Build final string
    format!(
        "{}{}e{:+03}",
        if is_negative { "-" } else { "" },
        mantissa_str,
        exponent
    )
}

/// Format number in arbitrary base (helper function)
fn format_base(value: &Float, base: i32) -> String {
    // Handle special values
    if value.is_nan() {
        return "nan".to_string();
    }
    if value.is_infinite() {
        return if value.is_sign_negative() {
            "-inf".to_string()
        } else {
            "inf".to_string()
        };
    }
    if value.is_zero() {
        return "0".to_string();
    }

    if base < 2 || base > 62 {
        return format!("{}", value);
    }

    let is_negative = value.is_sign_negative();
    let prec = value.prec();
    let abs_val = value.clone().abs();

    // Split into integer and fractional parts
    let integer_part = Float::with_val(prec, abs_val.clone().floor());
    let integer_val = match integer_part.to_integer() {
        Some(int) => int,
        None => return format!("{}", value),
    };

    // Integer part
    let mut result = if base <= 36 {
        integer_val.to_string_radix(base)
    } else {
        format_base_37_to_62(&integer_val, base)
    };

    // No fractional part display for non-decimal bases
    // The integer part is already extracted above

    if is_negative {
        result.insert(0, '-');
    }

    result
}

/// Format an integer in bases 37-62
/// Digit mapping: 0-9 (0-9), A-Z (10-35), a-z (36-61)
fn format_base_37_to_62(value: &rug::Integer, base: i32) -> String {
    use rug::Integer;

    if base < 37 || base > 62 {
        return value.to_string();
    }

    // Handle zero specially
    if *value == 0 {
        return "0".to_string();
    }

    // Handle negative numbers
    let is_negative = *value < 0;
    let mut abs_value = value.clone().abs();
    let base_int = Integer::from(base);

    let mut digits = Vec::new();

    while abs_value > 0 {
        let remainder = Integer::from(&abs_value % &base_int).to_i32().unwrap();
        abs_value /= &base_int;

        let digit_char = match remainder {
            0..=9 => (b'0' + remainder as u8) as char,
            10..=35 => (b'A' + (remainder - 10) as u8) as char,
            36..=61 => (b'a' + (remainder - 36) as u8) as char,
            _ => '?',
        };
        digits.push(digit_char);
    }

    digits.reverse();
    let mut result = digits.into_iter().collect::<String>();

    if is_negative {
        result.insert(0, '-');
    }

    result
}
