// SPDX-License-Identifier: GPL-3.0-only
// String operations

use crate::context::Context;
use crate::error::Result;
use crate::object::Object;
use rug::Float;

/// ->STR: ->str (convert object to string)
/// Stack: any -> string
pub fn to_str(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);

    // Stringify only if not already a string
    if !matches!(ctx.stack.get(0), Some(Object::String(_))) {
        let obj = ctx.stack.pop().unwrap();
        let string_repr = obj.display(&ctx.config);
        ctx.stack.push(Object::String(string_repr));
    }

    Ok(())
}

/// STR->: str-> (parse and execute string as RPN program)
/// Stack: string -> (executes string as program)
pub fn str_to(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);
    arg_must_be!(ctx, 0, String);

    let program_string = match ctx.stack.pop().unwrap() {
        Object::String(s) => s,
        _ => unreachable!(),
    };

    // Parse and execute the string as a program
    crate::program::execute(&program_string, ctx)?;

    Ok(())
}

/// CHR: chr (convert ASCII code to character)
/// Stack: number -> string
pub fn chr(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);
    arg_must_be!(ctx, 0, Number);

    let code = pop_one_number!(ctx);
    let code_int = code.to_f64() as i64;

    // Only printable ASCII characters (32-126)
    let ch = if code_int >= 32 && code_int <= 126 {
        char::from_u32(code_int as u32).unwrap_or('.')
    } else {
        '.'
    };

    ctx.stack.push(Object::String(ch.to_string()));
    Ok(())
}

/// NUM: num (get ASCII code of first character)
/// Stack: string -> number
pub fn num(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);
    arg_must_be!(ctx, 0, String);

    let string = match ctx.stack.pop().unwrap() {
        Object::String(s) => s,
        _ => unreachable!(),
    };

    let code = if let Some(first_char) = string.chars().next() {
        first_char as u32 as f64
    } else {
        0.0
    };

    push_number!(ctx, Float::with_val(ctx.config.precision_bits, code));
    Ok(())
}

/// SIZE: size (get string length)
/// Stack: string -> number
pub fn size(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 1);
    arg_must_be!(ctx, 0, String);

    let len = match ctx.stack.get(0).unwrap() {
        Object::String(s) => s.len(),
        _ => unreachable!(),
    };

    ctx.stack.pop();
    push_number!(ctx, Float::with_val(ctx.config.precision_bits, len));
    Ok(())
}

/// POS: pos (find substring position)
/// Stack: string substring -> number (1-indexed position, 0 if not found)
pub fn pos(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 2);
    arg_must_be!(ctx, 0, String);
    arg_must_be!(ctx, 1, String);

    let substring = match ctx.stack.pop().unwrap() {
        Object::String(s) => s,
        _ => unreachable!(),
    };

    let string = match ctx.stack.pop().unwrap() {
        Object::String(s) => s,
        _ => unreachable!(),
    };

    let position = if let Some(pos) = string.find(&substring) {
        pos + 1 // 1-indexed
    } else {
        0
    };

    push_number!(ctx, Float::with_val(ctx.config.precision_bits, position));
    Ok(())
}

/// SUB: sub (extract substring)
/// Stack: string start end -> substring
/// start and end are 1-indexed positions
pub fn sub(ctx: &mut Context) -> Result<()> {
    min_arguments!(ctx, 3);
    arg_must_be!(ctx, 0, Number);
    arg_must_be!(ctx, 1, Number);
    arg_must_be!(ctx, 2, String);

    let end_pos_f64 = pop_one_number!(ctx).to_f64();
    let start_pos_f64 = pop_one_number!(ctx).to_f64();

    let string = match ctx.stack.pop().unwrap() {
        Object::String(s) => s,
        _ => unreachable!(),
    };

    // Return empty string if either position is negative or zero
    if start_pos_f64 <= 0.0 || end_pos_f64 <= 0.0 {
        ctx.stack.push(Object::String(String::new()));
        return Ok(());
    }

    let start_pos = start_pos_f64 as usize;
    let end_pos = end_pos_f64 as usize;

    // Convert from 1-indexed to 0-indexed
    let start = start_pos - 1;
    let len = if end_pos >= start_pos {
        end_pos - start_pos + 1
    } else {
        0
    };

    // Extract substring
    let substring = if start < string.len() {
        string.chars().skip(start).take(len).collect()
    } else {
        String::new()
    };

    ctx.stack.push(Object::String(substring));
    Ok(())
}

/// ENDL: endl (push platform-specific end-of-line character)
/// Stack: -> eol_string
pub fn endl(ctx: &mut Context) -> Result<()> {
    // Platform-specific line ending
    let eol = if cfg!(target_os = "windows") {
        "\r\n"
    } else {
        "\n"
    };

    ctx.stack.push(Object::String(eol.to_string()));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rug::Float;

    #[test]
    fn test_to_str() {
        let mut ctx = Context::new();

        // Convert number to string
        push_number!(ctx, Float::with_val(128, 42));
        to_str(&mut ctx).unwrap();

        match ctx.stack.get(0).unwrap() {
            Object::String(s) => assert!(s.starts_with("42")),
            _ => panic!("Expected string"),
        }
    }

    #[test]
    fn test_chr_num() {
        let mut ctx = Context::new();

        // chr: 65 -> 'A'
        push_number!(ctx, Float::with_val(128, 65));
        chr(&mut ctx).unwrap();

        match ctx.stack.get(0).unwrap() {
            Object::String(s) => assert_eq!(s, "A"),
            _ => panic!("Expected string"),
        }

        // num: 'A' -> 65
        num(&mut ctx).unwrap();

        match ctx.stack.get(0).unwrap() {
            Object::Number { value, .. } => assert_eq!(value.to_f64(), 65.0),
            _ => panic!("Expected number"),
        }
    }

    #[test]
    fn test_size() {
        let mut ctx = Context::new();

        ctx.stack.push(Object::String("hello".to_string()));
        size(&mut ctx).unwrap();

        match ctx.stack.get(0).unwrap() {
            Object::Number { value, .. } => assert_eq!(value.to_f64(), 5.0),
            _ => panic!("Expected number"),
        }
    }

    #[test]
    fn test_pos() {
        let mut ctx = Context::new();

        ctx.stack.push(Object::String("hello world".to_string()));
        ctx.stack.push(Object::String("world".to_string()));
        pos(&mut ctx).unwrap();

        match ctx.stack.get(0).unwrap() {
            Object::Number { value, .. } => assert_eq!(value.to_f64(), 7.0), // 1-indexed
            _ => panic!("Expected number"),
        }
    }

    #[test]
    fn test_sub() {
        let mut ctx = Context::new();

        ctx.stack.push(Object::String("hello world".to_string()));
        push_number!(ctx, Float::with_val(128, 1)); // start
        push_number!(ctx, Float::with_val(128, 5)); // end
        sub(&mut ctx).unwrap();

        match ctx.stack.get(0).unwrap() {
            Object::String(s) => assert_eq!(s, "hello"),
            _ => panic!("Expected string"),
        }
    }
}
