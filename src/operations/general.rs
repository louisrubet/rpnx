// SPDX-License-Identifier: GPL-3.0-only
// General operations: modes, bases, precision, utility

use crate::context::Context;
use crate::error::{Error, Result};
use crate::number::DisplayMode;
use crate::object::Object;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Quit: quit, q, exit
pub fn quit(_ctx: &mut Context) -> Result<()> {
    Err(Error::Goodbye)
}

/// Help: help, h, ?
pub fn help(ctx: &mut Context) -> Result<()> {
    // Color codes
    const R: &str = "\x1b[0m"; // Reset
    const T: &str = "\x1b[1;37m"; // Title: bold white
    const K: &str = "\x1b[33m"; // Keyword: yellow
    const O: &str = "\x1b[1m"; // Operator: bold
    const N: &str = "\x1b[36m"; // Number: cyan
    const Y: &str = "\x1b[35m"; // Symbol: magenta
    const P: &str = "\x1b[34m"; // Program: blue

    println!(
        "\n\x1b[1mrpnx\x1b[0m v{} - Reverse Polish Notation Calculator",
        VERSION
    );
    println!("License: LGPL-3.0-only");
    println!("\nAvailable commands:\n");

    // Arithmetic operations
    println!("{T}ARITHMETIC{R}");
    println!("  {O}+{R}, {O}-{R}, {O}*{R}, {O}/{R}              Basic arithmetic");
    println!("  {K}neg{R}, {K}chs{R}                Negate");
    println!("  {K}inv{R}                     Inverse");
    println!("  {K}pow{R}                     Power");
    println!("  {K}sqrt{R}                    Square root");
    println!("  {K}sq{R}                      Square");
    println!("  {K}abs{R}                     Absolute value of real, magnitude of complex");
    println!("  {K}sign{R}                    Sign of a real number ({N}-1{R}, {N}0{R}, {N}1{R}), signum of a complex number");
    println!("  {K}min{R}, {K}max{R}                Minimum, maximum");
    println!("  {K}mod{R}                     Modulo");
    println!("  {O}%{R}                       Percent");
    println!("  {K}%inv{R}                    Inverse percent");
    println!("  {K}fact{R}                    Factorial");
    println!("  {K}floor{R}, {K}ceil{R}             Rounding");
    println!("  {K}ip{R}, {K}fp{R}                  Integer and fractional parts");
    println!("  {K}mant{R}, {K}xpon{R}              Mantissa and exponent");

    // Bitwise operations
    println!("\n{T}BITWISE{R}");
    println!("  {O}&{R}, {O}|{R}, {O}^{R}, {O}~{R}              And, or, xor, not (complement)");

    // Stack operations
    println!("\n{T}STACK{R}");
    println!("  {K}swap{R}                    Swap top items");
    println!("  {K}dup{R}, {K}dup2{R}, {K}dupn{R}         Duplicate top item(s)");
    println!("  {K}drop{R}, {K}drop2{R}, {K}dropn{R}      Remove top item(s)");
    println!("  {K}pop{R}                     Alias for {K}drop{R}");
    println!("  {K}erase{R}, {K}del{R}, {K}clear{R}       Clear entire stack");
    println!("  {K}pick{R}                    Copy level n item to top");
    println!("  {K}depth{R}                   Push stack size");
    println!("  {K}rot{R}                     Rotate top 3 items");
    println!("  {K}roll{R}, {K}rolld{R}             Roll stack items");
    println!("  {K}over{R}                    Copy second item to top");

    // Comparison and logic
    println!("\n{T}COMPARISON & LOGIC{R}");
    println!("  {O}>{R}, {O}>={R}, {O}<{R}, {O}<={R}            Comparison operators");
    println!("  {O}=={R}, {K}same{R}, {O}!={R}            Equality operators");
    println!("  {K}and{R}, {K}or{R}, {K}xor{R}, {K}not{R}       Logical operators");

    // Complex numbers
    println!("\n{T}COMPLEX NUMBERS{R}");
    println!("  {N}(re,im){R}                 Complex number notation");
    println!("  {K}re{R}, {K}im{R}                  Extract real and imaginary part");
    println!("  {K}arg{R}                     Argument (angle)");
    println!("  {K}conj{R}                    Conjugate");
    println!("  {K}c->r{R}, {K}r->c{R}              real to complex, complex to real");
    println!("  {K}p->r{R}, {K}r->p{R}              polar / rectangular conversions");

    // Trigonometry
    println!("\n{T}TRIGONOMETRY{R}");
    println!("  {K}sin{R}, {K}cos{R}, {K}tan{R}           Sine, cosine, tangent");
    println!("  {K}asin{R}, {K}acos{R}, {K}atan{R}        Arc functions");
    println!("  {K}atan2{R}                   Two-argument arctangent");
    println!("  {K}sinh{R}, {K}cosh{R}, {K}tanh{R}        Hyperbolic functions");
    println!("  {K}asinh{R}, {K}acosh{R}, {K}atanh{R}     Inverse hyperbolic");
    println!("  {K}d->r{R}, {K}r->d{R}              Degrees / radians conversions");
    println!("  {K}pi{R}                      Pi constant");

    // Logarithms
    println!("\n{T}LOGARITHMS{R}");
    println!("  {K}ln{R}, {K}log{R}                 Natural logarithm");
    println!("  {K}exp{R}                     Exponential");
    println!("  {K}log10{R}, {K}alog10{R}           Base 10 logarithm / antilog");
    println!("  {K}exp10{R}                   Alias for {K}alog10{R}");
    println!("  {K}log2{R}, {K}alog2{R}             Base 2 logarithm / antilog");
    println!("  {K}exp2{R}                    Alias for {K}alog2{R}");
    println!("  {K}logn{R}, {K}alogn{R}             Arbitrary base log / antilog");
    println!("  {K}lnp1{R}, {K}expm{R}              ln({N}1{R}+x), exp(x)-{N}1{R}");
    println!("  {K}e{R}                       Euler's number");

    // Variables
    println!("\n{T}VARIABLES{R}");
    println!("  {Y}'name'{R}                  Variable notation");
    println!("  {K}sto{R}                     Store in a variable");
    println!("  {K}rcl{R}                     Recall a variable");
    println!("  {K}purge{R}                   Delete a variable");
    println!("  {K}sto+{R}, {K}sto-{R}, {K}sto*{R}, {K}sto/{R}  Arithmetic with a variable");
    println!("  {K}sneg{R}, {K}stoneg{R}            Negate a variable");
    println!("  {K}sinv{R}, {K}stoinv{R}            Invert a variable");
    println!("  {K}vars{R}                    List all variables");
    println!("  {K}clusr{R}                   Clear all variables");

    // Control flow
    println!("\n{T}CONTROL FLOW{R}");
    println!("  {K}if{R}, {K}then{R}, {K}else{R}, {K}end{R}     Conditional execution");
    println!("  {K}ift{R}, {K}ifte{R}               Inline conditionals");
    println!("  {K}for{R}, {K}next{R}               For loop with variable");
    println!("  {K}start{R}, {K}next{R}             For loop without variable");
    println!("  {K}step{R}                    For loop with custom step");
    println!("  {K}while{R}, {K}repeat{R}, {K}end{R}      While loop");
    println!("  {K}do{R}, {K}until{R}, {K}end{R}          Do-until loop");

    // Programs
    println!("\n{T}PROGRAMS{R}");
    println!("  {P}<< ... >>{R}               Program notation");
    println!("  {P}« ... »{R}                 Alternative delimiter");
    println!("  {K}eval{R}                    Execute a program");

    // Configuration
    println!("\n{T}CONFIGURATION{R}");
    println!("  {K}std{R}                     Standard display mode");
    println!("  {K}fix{R}                     Fixed-point mode");
    println!("  {K}sci{R}                     Scientific notation");
    println!("  {K}prec{R}                    Set precision (bits)");
    println!("  {K}default{R}                 Reset to defaults");
    println!("  {K}hex{R}, {K}dec{R}, {K}bin{R}           Set number base");
    println!("  {K}base{R}                    Set arbitrary base ({N}2{R}-{N}62{R})");
    println!("  {K}type{R}                    Show object type");

    // General
    println!("\n{T}GENERAL{R}");
    println!("  {K}help{R}, {K}h{R}, {K}?{R}              This help");
    println!("  {K}history{R}                 Show the command history");
    println!("  {K}version{R}                 Show rpnx version");
    println!("  {K}uname{R}                   Show system info");
    println!("  {K}error{R}                   Push last error code");
    println!("  {K}strerror{R}                Push last error message");
    println!("  {K}test{R}                    Run test file");
    println!("  {K}quit{R}, {K}q{R}, {K}exit{R}           Exit rpnx");

    println!("\n{T}CURRENT SETTINGS{R}");
    println!("  Mode: {:?}", ctx.config.mode);
    println!(
        "  Precision: {N}{}{R} bits ({N}{}{R} decimal digits)",
        ctx.config.precision_bits, ctx.config.digits
    );
    println!("\nFor more information: \x1b]8;;https://github.com/louisrubet/rpnx\x1b\\\x1b[4mhttps://github.com/louisrubet/rpnx\x1b[0m\x1b]8;;\x1b\\");
    Ok(())
}

/// Version: version
pub fn version(ctx: &mut Context) -> Result<()> {
    ctx.stack.push(Object::Symbol {
        name: VERSION.to_string(),
        auto_eval: false,
    });
    Ok(())
}

/// Uname: uname
pub fn uname(ctx: &mut Context) -> Result<()> {
    let info = format!("rpnx {} (Rust)", VERSION);
    ctx.stack.push(Object::Symbol {
        name: info,
        auto_eval: false,
    });
    Ok(())
}

/// Type: type (push type name of top object)
pub fn type_of(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    let type_name = ctx.stack.get(0).unwrap().type_name();
    ctx.stack.pop();
    ctx.stack.push(Object::Symbol {
        name: type_name.to_string(),
        auto_eval: false,
    });
    Ok(())
}

/// Standard mode: std
pub fn std_mode(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);
    arg_must_be!(ctx, 0, Number);

    let digits = match ctx.stack.pop().unwrap() {
        Object::Number { value, .. } => {
            let f = value.to_f64();
            if !f.is_finite() || f < 0.0 || f.fract() != 0.0 {
                return Err(Error::BadOperandType);
            }
            f as i32
        }
        _ => unreachable!(),
    };

    ctx.config.mode = DisplayMode::Std;
    ctx.config.digits = digits;
    Ok(())
}

/// Fixed mode: fix
pub fn fix_mode(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);
    arg_must_be!(ctx, 0, Number);

    let digits = match ctx.stack.pop().unwrap() {
        Object::Number { value, .. } => {
            let f = value.to_f64();
            if !f.is_finite() || f.fract() != 0.0 {
                return Err(Error::BadOperandType);
            }
            if f < 0.0 {
                return Err(Error::OutOfRange);
            }
            f as i32
        }
        _ => unreachable!(),
    };

    ctx.config.mode = DisplayMode::Fix;
    ctx.config.digits = digits;
    Ok(())
}

/// Scientific mode: sci
pub fn sci_mode(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);
    arg_must_be!(ctx, 0, Number);

    let digits = match ctx.stack.pop().unwrap() {
        Object::Number { value, .. } => {
            let f = value.to_f64();
            if !f.is_finite() || f.fract() != 0.0 {
                return Err(Error::BadOperandType);
            }
            if f < 0.0 {
                return Err(Error::OutOfRange);
            }
            f as i32
        }
        _ => unreachable!(),
    };

    ctx.config.mode = DisplayMode::Sci;
    ctx.config.digits = digits;
    Ok(())
}

/// Precision: prec (set bit precision)
pub fn precision(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);
    arg_must_be!(ctx, 0, Number);

    let bits = match ctx.stack.pop().unwrap() {
        Object::Number { value, .. } => {
            let f = value.to_f64();
            if !f.is_finite() || f < 2.0 || f > 1000000.0 || f.fract() != 0.0 {
                return Err(Error::OutOfRange);
            }
            f as u32
        }
        _ => unreachable!(),
    };

    ctx.config.precision_bits = bits;

    // In std mode, also update displayed digits to match precision
    if ctx.config.mode == DisplayMode::Std {
        ctx.config.digits = ((bits as f64) * 0.30102999566398119).floor() as i32;
    }

    Ok(())
}

/// Default: default (reset to default settings)
pub fn default(ctx: &mut Context) -> Result<()> {
    ctx.config.mode = DisplayMode::Std;
    ctx.config.precision_bits = 128;
    ctx.config.digits = 38;
    Ok(())
}

/// Hexadecimal: hex (set base 16 for top number)
pub fn hex(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    match ctx.stack.get_mut(0).unwrap() {
        Object::Number { base, .. } => {
            *base = 16;
            Ok(())
        }
        Object::Complex {
            re_base, im_base, ..
        } => {
            *re_base = 16;
            *im_base = 16;
            Ok(())
        }
        _ => Err(Error::BadOperandType),
    }
}

/// Decimal: dec (set base 10 for top number)
pub fn dec(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    match ctx.stack.get_mut(0).unwrap() {
        Object::Number { base, .. } => {
            *base = 10;
            Ok(())
        }
        Object::Complex {
            re_base, im_base, ..
        } => {
            *re_base = 10;
            *im_base = 10;
            Ok(())
        }
        _ => Err(Error::BadOperandType),
    }
}

/// Binary: bin (set base 2 for top number)
pub fn bin(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    match ctx.stack.get_mut(0).unwrap() {
        Object::Number { base, .. } => {
            *base = 2;
            Ok(())
        }
        Object::Complex {
            re_base, im_base, ..
        } => {
            *re_base = 2;
            *im_base = 2;
            Ok(())
        }
        _ => Err(Error::BadOperandType),
    }
}

/// Arbitrary base: base (set arbitrary base 2-62)
pub fn base(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 2);
    arg_must_be!(ctx, 0, Number);

    let new_base = match ctx.stack.pop().unwrap() {
        Object::Number { value, .. } => {
            let f = value.to_f64();
            if !f.is_finite() || f < 2.0 || f > 62.0 || f.fract() != 0.0 {
                return Err(Error::OutOfRange);
            }
            f as u8
        }
        _ => unreachable!(),
    };

    match ctx.stack.get_mut(0).unwrap() {
        Object::Number { base, .. } => {
            *base = new_base;
            Ok(())
        }
        Object::Complex {
            re_base, im_base, ..
        } => {
            *re_base = new_base;
            *im_base = new_base;
            Ok(())
        }
        _ => Err(Error::BadOperandType),
    }
}

/// Test: test (run tests from file)
/// Stack: "filename" or 'filename' -> (empty)
pub fn test(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    // Get filename from stack (symbol)
    let filename = match ctx.stack.pop() {
        Some(Object::Symbol { name, .. }) => name,
        _ => return Err(Error::BadOperandType),
    };

    // Convert to path
    use std::path::Path;
    let path = Path::new(&filename);

    // Check if file exists
    if !path.exists() {
        eprintln!("Error: Test file '{}' not found", filename);
        return Err(Error::RuntimeError(format!("file not found: {}", filename)));
    }

    // Run the test file
    match crate::test_framework::run_test_file(path) {
        Ok((test_file, results)) => {
            let mut tests_passed = 0;
            let mut tests_failed = 0;
            let mut total_steps_passed = 0;
            let mut total_steps_failed = 0;

            // Print test file header
            println!();
            println!("{}: {}", filename, test_file.description);

            // Print results, tracking source file for headers
            let mut current_source: Option<String> = None;
            for (test_case, result) in test_file.test_cases.iter().zip(results.iter()) {
                // Print source file header when it changes
                if test_case.source_file != current_source {
                    if let Some(ref src) = test_case.source_file {
                        println!();
                        println!("# {}", src);
                    }
                    current_source = test_case.source_file.clone();
                }

                if result.passed {
                    println!("## {} \x1b[32mPASSED\x1b[0m", result.name);
                    tests_passed += 1;
                } else {
                    println!("## {} \x1b[31mFAILED\x1b[0m", result.name);
                    tests_failed += 1;
                }
                total_steps_passed += result.steps_passed;
                total_steps_failed += result.steps_failed;
            }

            // Print summary
            let total_tests = results.len();
            let total_steps = total_steps_passed + total_steps_failed;
            println!(
                "run {} tests: {} passed, {} failed ({} steps: {} passed, {} failed)",
                total_tests,
                tests_passed,
                tests_failed,
                total_steps,
                total_steps_passed,
                total_steps_failed
            );
            println!();
            println!(
                "Total: run {} tests: {} passed, {} failed ({} steps: {} passed, {} failed)",
                total_tests,
                tests_passed,
                tests_failed,
                total_steps,
                total_steps_passed,
                total_steps_failed
            );

            Ok(())
        }
        Err(e) => {
            eprintln!("Error running tests: {}", e);
            Err(Error::RuntimeError(format!("test execution failed: {}", e)))
        }
    }
}

/// Date: date (push current date as symbol in ISO format)
/// Stack: -> 'YYYY-MM-DD'
pub fn date(ctx: &mut Context) -> Result<()> {
    use chrono::Local;

    let now = Local::now();
    let date_str = now.format("%Y-%m-%d").to_string();
    ctx.stack.push(Object::Symbol {
        name: date_str,
        auto_eval: false,
    });

    Ok(())
}

/// Time: time (push current time as symbol in HH:MM:SS format)
/// Stack: -> 'HH:MM:SS'
pub fn time(ctx: &mut Context) -> Result<()> {
    use chrono::Local;

    let now = Local::now();
    let time_str = now.format("%H:%M:%S").to_string();
    ctx.stack.push(Object::Symbol {
        name: time_str,
        auto_eval: false,
    });

    Ok(())
}

/// Ticks: ticks (push current timestamp in microseconds since Unix epoch)
/// Stack: -> number
pub fn ticks(ctx: &mut Context) -> Result<()> {
    use rug::Float;
    use std::time::SystemTime;

    let now = SystemTime::now();
    let duration = now
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|_| Error::RuntimeError("system time error".to_string()))?;

    // Convert to microseconds
    let micros = duration.as_secs() * 1_000_000 + duration.subsec_micros() as u64;

    let value = Float::with_val(ctx.config.precision_bits, micros);
    push_number!(ctx, value);

    Ok(())
}

/// Error: error (push last error code as integer)
/// Stack: -> error_code
pub fn error(ctx: &mut Context) -> Result<()> {
    use rug::Float;

    let code = match &ctx.last_error {
        Some(e) => e.error_code(),
        None => 0, // No error
    };

    let value = Float::with_val(ctx.config.precision_bits, code);
    push_number!(ctx, value);

    // Preserve last_error so it can be queried again
    ctx.preserve_last_error = true;

    Ok(())
}

/// Strerror: strerror (push last error as symbol)
/// Stack: -> 'error message'
pub fn strerror(ctx: &mut Context) -> Result<()> {
    let msg = match &ctx.last_error {
        Some(e) => Error::name_from_code(e.error_code()),
        None => "ok",
    };

    ctx.stack.push(Object::Symbol {
        name: msg.to_string(),
        auto_eval: false,
    });

    // Preserve last_error so it can be queried again
    ctx.preserve_last_error = true;

    Ok(())
}

/// History: history (display command history)
/// Stack: -> (unchanged)
pub fn history(_ctx: &mut Context) -> Result<()> {
    use std::fs;

    let path = crate::repl::history_path();

    if !path.exists() {
        println!("No history yet.");
        return Ok(());
    }

    match fs::read_to_string(&path) {
        Ok(contents) => {
            let lines: Vec<&str> = contents.lines().collect();
            for (i, line) in lines.iter().enumerate() {
                println!("{:5}  {}", i + 1, line);
            }
        }
        Err(e) => {
            eprintln!("Cannot read history: {}", e);
        }
    }

    Ok(())
}
