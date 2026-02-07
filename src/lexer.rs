// SPDX-License-Identifier: GPL-3.0-only
// Lexer for RPN calculator

use crate::error::{Error, Result};
use rug::ops::CompleteRound;
use rug::Float;
use rug::Integer;

/// Parse a number string in bases 37-62
/// rug's parse_radix only supports up to base 36, so we implement custom parsing
/// Digit mapping: 0-9 (0-9), A-Z (10-35), a-z (36-61)
fn parse_base_37_to_62(digits_str: &str, base: i32) -> Option<Float> {
    if base < 37 || base > 62 {
        return None;
    }

    let mut result = Integer::from(0);
    let base_int = Integer::from(base);

    for ch in digits_str.chars() {
        let digit_value = match ch {
            '0'..='9' => (ch as i32) - ('0' as i32),
            'A'..='Z' => (ch as i32) - ('A' as i32) + 10,
            'a'..='z' => (ch as i32) - ('a' as i32) + 36,
            _ => return None, // Invalid character
        };

        if digit_value >= base {
            return None; // Digit out of range for this base
        }

        result *= &base_int;
        result += digit_value;
    }

    Some(Float::with_val(128, result))
}

/// Token types produced by lexical analysis
#[derive(Debug, Clone)]
pub enum Token {
    /// Number with value and base (2-62)
    Number { value: Float, base: u8 },

    /// Complex number with real and imaginary parts
    Complex {
        re: Float,
        im: Float,
        re_base: u8,
        im_base: u8,
    },

    /// Symbol (variable name)
    /// quoted: true if 'symbol' syntax, false if bare word
    Symbol { name: String, quoted: bool },

    /// Program (nested sequence)
    Program(String),

    /// Reserved keyword
    Keyword(String),

    /// Branch keyword (control flow)
    Branch(String),
}

/// Lexer for tokenizing RPN expressions
pub struct Lexer;

impl Lexer {
    /// Analyze source string and produce tokens
    pub fn analyze(source: &str) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        let mut chars = source.chars().peekable();
        let mut position = 0;

        while let Some(ch) = chars.peek() {
            match ch {
                // Skip whitespace
                ' ' | '\t' | '\n' | '\r' => {
                    chars.next();
                    position += 1;
                }

                // Symbol: '...'
                '\'' => {
                    chars.next();
                    position += 1;
                    let token = Self::parse_symbol(&mut chars, &mut position)?;
                    tokens.push(token);
                }

                // Program: << ... >> or comparison operators: <, <=
                '<' => {
                    let mut temp_chars = chars.clone();
                    temp_chars.next(); // skip '<'

                    if let Some(&second) = temp_chars.peek() {
                        if second == '<' {
                            // It's a program start: <<
                            chars.next();
                            chars.next();
                            position += 2;
                            let token =
                                Self::parse_program(&mut chars, &mut position, ">>", false)?;
                            tokens.push(token);
                            continue;
                        } else if second == '=' {
                            // It's the <= operator
                            chars.next();
                            chars.next();
                            position += 2;
                            tokens.push(Token::Symbol {
                                name: "<=".to_string(),
                                quoted: false,
                            });
                            continue;
                        }
                    }
                    // Single < operator
                    chars.next();
                    position += 1;
                    tokens.push(Token::Symbol {
                        name: "<".to_string(),
                        quoted: false,
                    });
                }

                '«' => {
                    chars.next();
                    position += 1;
                    let token = Self::parse_program(&mut chars, &mut position, "»", true)?;
                    tokens.push(token);
                }

                // Arrow for local variables: ->
                // But also handle symbol names like ->str
                '-' => {
                    let mut temp_chars = chars.clone();
                    temp_chars.next(); // skip '-'
                    if let Some(second) = temp_chars.peek() {
                        if *second == '>' {
                            temp_chars.next(); // skip '>'
                                               // Check what comes after ->
                            let is_standalone = if let Some(&next_ch) = temp_chars.peek() {
                                // -> is standalone if followed by whitespace, delimiter, or nothing
                                next_ch.is_whitespace()
                                    || next_ch == '"'
                                    || next_ch == '\''
                                    || next_ch == '('
                                    || next_ch == ')'
                                    || next_ch == '<'
                                    || next_ch == '«'
                            } else {
                                // End of input, -> is standalone
                                true
                            };

                            if is_standalone {
                                // Parse -> as a standalone operator
                                chars.next();
                                chars.next();
                                position += 2;
                                tokens.push(Token::Symbol {
                                    name: "->".to_string(),
                                    quoted: false,
                                });
                                continue;
                            }
                        }
                    }
                    // Not a standalone ->, parse as word (could be negative number or ->str etc.)
                    let token = Self::parse_word(&mut chars, &mut position)?;
                    tokens.push(token);
                }

                // Complex or parenthetical: (...)
                '(' => {
                    // Try to parse as complex number
                    let saved_chars = chars.clone();
                    let saved_pos = position;

                    match Self::parse_complex(&mut chars, &mut position) {
                        Ok(token) => tokens.push(token),
                        Err(_) => {
                            // Not a valid complex number, collect as a symbol
                            chars = saved_chars;
                            position = saved_pos;

                            // Collect everything from ( to ) or end of input
                            let mut symbol_name = String::new();
                            symbol_name.push(chars.next().unwrap()); // '('
                            position += 1;

                            while let Some(&ch) = chars.peek() {
                                if ch.is_whitespace() {
                                    break;
                                }
                                if ch == ')' {
                                    symbol_name.push(chars.next().unwrap());
                                    position += 1;
                                    break;
                                }
                                symbol_name.push(chars.next().unwrap());
                                position += 1;
                            }

                            tokens.push(Token::Symbol {
                                name: symbol_name,
                                quoted: true,
                            });
                        }
                    }
                }

                // Greater than: > or >=
                '>' => {
                    chars.next();
                    position += 1;

                    // Check if followed by '='
                    if let Some(&'=') = chars.peek() {
                        chars.next();
                        position += 1;
                        tokens.push(Token::Symbol {
                            name: ">=".to_string(),
                            quoted: false,
                        });
                    } else {
                        tokens.push(Token::Symbol {
                            name: ">".to_string(),
                            quoted: false,
                        });
                    }
                }

                // Everything else: numbers, keywords, or unknown symbols
                _ => {
                    let token = Self::parse_word(&mut chars, &mut position)?;
                    tokens.push(token);
                }
            }
        }

        Ok(tokens)
    }

    /// Parse a quoted symbol '...'
    fn parse_symbol(
        chars: &mut std::iter::Peekable<std::str::Chars>,
        position: &mut usize,
    ) -> Result<Token> {
        let mut name = String::new();

        while let Some(ch) = chars.next() {
            *position += 1;
            match ch {
                '\'' => {
                    // End of symbol
                    return Ok(Token::Symbol { name, quoted: true });
                }
                _ => name.push(ch),
            }
        }

        // End of input without closing quote - treat as valid symbol
        Ok(Token::Symbol { name, quoted: true })
    }

    /// Parse a program << ... >> or « ... »
    fn parse_program(
        chars: &mut std::iter::Peekable<std::str::Chars>,
        position: &mut usize,
        _end_delimiter: &str,
        unicode: bool,
    ) -> Result<Token> {
        let mut content = String::new();
        let mut depth = 1;

        while let Some(&ch) = chars.peek() {
            if unicode {
                // Unicode delimiter »
                if ch == '»' {
                    chars.next();
                    *position += 1;
                    depth -= 1;
                    if depth == 0 {
                        return Ok(Token::Program(content));
                    }
                    content.push(ch);
                    continue;
                } else if ch == '«' {
                    depth += 1;
                } else if ch == '<' {
                    // << also increments depth in unicode mode
                    let mut temp_chars = chars.clone();
                    temp_chars.next();
                    if let Some('<') = temp_chars.peek() {
                        depth += 1;
                    }
                } else if ch == '>' {
                    // >> also decrements depth in unicode mode
                    let mut temp_chars = chars.clone();
                    temp_chars.next();
                    if let Some('>') = temp_chars.peek() {
                        chars.next();
                        chars.next();
                        *position += 2;
                        depth -= 1;
                        if depth == 0 {
                            return Ok(Token::Program(content));
                        }
                        content.push_str(">>");
                        continue;
                    }
                }
            } else {
                // ASCII delimiter >>
                if ch == '>' {
                    let mut temp_chars = chars.clone();
                    temp_chars.next();
                    if let Some('>') = temp_chars.peek() {
                        chars.next();
                        chars.next();
                        *position += 2;
                        depth -= 1;
                        if depth == 0 {
                            return Ok(Token::Program(content));
                        }
                        content.push_str(">>");
                        continue;
                    }
                } else if ch == '<' {
                    let mut temp_chars = chars.clone();
                    temp_chars.next();
                    if let Some('<') = temp_chars.peek() {
                        depth += 1;
                    }
                } else if ch == '«' {
                    // « also increments depth in ASCII mode
                    depth += 1;
                } else if ch == '»' {
                    // » also decrements depth in ASCII mode
                    chars.next();
                    *position += 1;
                    depth -= 1;
                    if depth == 0 {
                        return Ok(Token::Program(content));
                    }
                    content.push(ch);
                    continue;
                }
            }

            let ch = chars.next().unwrap();
            *position += 1;
            content.push(ch);
        }

        // Unterminated program: auto-close with matching delimiters
        // Add appropriate closing delimiters for the remaining depth
        if unicode {
            for _ in 1..depth {
                content.push_str(" »");
            }
        } else {
            for _ in 1..depth {
                content.push_str(" >>");
            }
        }

        Ok(Token::Program(content))
    }

    /// Parse a number string with base support (helper for complex numbers)
    fn parse_number_with_base(num_str: &str) -> Result<(Float, u8)> {
        // Check for leading minus sign
        let (is_negative, num_without_sign) = if num_str.starts_with('-') {
            (true, &num_str[1..])
        } else if num_str.starts_with('+') {
            (false, &num_str[1..])
        } else {
            (false, num_str)
        };

        // Try to parse with base prefix (using num_without_sign consistently)
        // Reject fractional parts in hex/bin/base (no decimal points allowed)
        let (mut value, base) = if num_without_sign.starts_with("0x")
            || num_without_sign.starts_with("0X")
        {
            // Hex - reject if contains decimal point
            let hex_str = &num_without_sign[2..];
            if hex_str.contains('.') {
                return Err(Error::SyntaxError(format!("invalid hex number: {}", num_str)));
            }
            let val = Float::parse_radix(hex_str, 16)
                .map_err(|_| Error::SyntaxError(format!("invalid hex number: {}", num_str)))?
                .complete(128);
            (val, 16)
        } else if num_without_sign.starts_with("0b") || num_without_sign.starts_with("0B") {
            // Binary - reject if contains decimal point
            let bin_str = &num_without_sign[2..];
            if bin_str.contains('.') {
                return Err(Error::SyntaxError(format!("invalid binary number: {}", num_str)));
            }
            let val = Float::parse_radix(bin_str, 2)
                .map_err(|_| Error::SyntaxError(format!("invalid binary number: {}", num_str)))?
                .complete(128);
            (val, 2)
        } else if let Some(b_pos) = num_without_sign.find(|c| c == 'b' || c == 'B') {
            // Arbitrary base: Nb... or NB... - reject if contains decimal point
            if b_pos > 0 && b_pos < num_without_sign.len() - 1 {
                let base_str = &num_without_sign[..b_pos];
                let digits_str = &num_without_sign[b_pos + 1..];

                if digits_str.contains('.') {
                    return Err(Error::SyntaxError(format!("invalid base number: {}", num_str)));
                }

                if let Ok(base) = base_str.parse::<i32>() {
                    if base >= 2 && base <= 62 {
                        let val = if base <= 36 {
                            Float::parse_radix(digits_str, base)
                                .map_err(|_| {
                                    Error::SyntaxError(format!(
                                        "invalid base {} number: {}",
                                        base, num_str
                                    ))
                                })?
                                .complete(128)
                        } else {
                            parse_base_37_to_62(digits_str, base).ok_or_else(|| {
                                Error::SyntaxError(format!(
                                    "invalid base {} number: {}",
                                    base, num_str
                                ))
                            })?
                        };
                        (val, base as u8)
                    } else {
                        // Invalid base, parse as decimal
                        let val = Float::parse(num_without_sign)
                            .map_err(|_| {
                                Error::SyntaxError(format!("invalid number: {}", num_str))
                            })?
                            .complete(128);
                        (val, 10)
                    }
                } else {
                    // Not a valid base number, parse as decimal
                    let val = Float::parse(num_without_sign)
                        .map_err(|_| Error::SyntaxError(format!("invalid number: {}", num_str)))?
                        .complete(128);
                    (val, 10)
                }
            } else {
                // Not a valid base format, parse as decimal
                let val = Float::parse(num_without_sign)
                    .map_err(|_| Error::SyntaxError(format!("invalid number: {}", num_str)))?
                    .complete(128);
                (val, 10)
            }
        } else {
            // Decimal (use num_without_sign to avoid double negation)
            let val = Float::parse(num_without_sign)
                .map_err(|_| Error::SyntaxError(format!("invalid number: {}", num_str)))?
                .complete(128);
            (val, 10)
        };

        // Apply sign
        if is_negative {
            value = -value;
        }

        Ok((value, base))
    }

    /// Parse complex number (re,im)
    fn parse_complex(
        chars: &mut std::iter::Peekable<std::str::Chars>,
        position: &mut usize,
    ) -> Result<Token> {
        // Skip opening paren
        chars.next();
        *position += 1;

        // Collect everything until closing paren
        let mut content = String::new();
        while let Some(ch) = chars.peek() {
            if *ch == ')' {
                chars.next();
                *position += 1;
                break;
            }
            content.push(chars.next().unwrap());
            *position += 1;
        }

        // Split by comma
        let parts: Vec<&str> = content.split(',').collect();
        if parts.len() != 2 {
            return Err(Error::SyntaxError(
                "complex number must have format (re,im)".to_string(),
            ));
        }

        // Parse real and imaginary parts with base support
        let re_str = parts[0].trim();
        let im_str = parts[1].trim();

        let (re, re_base) = Self::parse_number_with_base(re_str)?;
        let (im, im_base) = Self::parse_number_with_base(im_str)?;

        Ok(Token::Complex {
            re,
            im,
            re_base,
            im_base,
        })
    }

    /// Parse a word (number, keyword, or unknown symbol)
    fn parse_word(
        chars: &mut std::iter::Peekable<std::str::Chars>,
        position: &mut usize,
    ) -> Result<Token> {
        let mut word = String::new();

        // Collect non-whitespace characters
        // Break on delimiters but NOT on operators (to allow sto+, sto-, etc.)
        while let Some(&ch) = chars.peek() {
            if ch.is_whitespace()
                || ch == '"'
                || ch == '\''
                || ch == '('
                || ch == ')'
                || ch == '<'
                || ch == '«'
            {
                break;
            }
            word.push(chars.next().unwrap());
            *position += 1;
        }

        if word.is_empty() {
            return Err(Error::SyntaxError("unexpected end of input".to_string()));
        }

        // Try to parse as number
        if let Ok(token) = Self::parse_number_token(&word) {
            return Ok(token);
        }

        // Otherwise, it's a keyword or unknown symbol
        // We'll classify it later in the parser
        Ok(Token::Symbol {
            name: word,
            quoted: false,
        })
    }

    /// Try to parse a word as a number with base support
    fn parse_number_token(word: &str) -> Result<Token> {
        // Check for leading minus sign
        let (is_negative, word_without_sign) = if word.starts_with('-') {
            (true, &word[1..])
        } else if word.starts_with('+') {
            (false, &word[1..])
        } else {
            (false, word)
        };

        // Hex: 0x... or 0x-...
        // Reject if contains decimal point (treat as symbol)
        if word_without_sign.starts_with("0x") || word_without_sign.starts_with("0X") {
            let hex_str = &word_without_sign[2..];
            // Handle 0x-... format (minus after prefix)
            let (hex_negative, hex_digits) = if hex_str.starts_with('-') {
                (true, &hex_str[1..])
            } else if hex_str.starts_with('+') {
                (false, &hex_str[1..])
            } else {
                (false, hex_str)
            };

            // Reject fractional hex numbers (treat as symbol)
            if !hex_digits.contains('.') {
                if let Ok(value) = Float::parse_radix(hex_digits, 16) {
                    let mut result = value.complete(128);
                    if is_negative ^ hex_negative {
                        // XOR: negate if exactly one minus sign
                        result = -result;
                    }
                    return Ok(Token::Number {
                        value: result,
                        base: 16,
                    });
                }
            }
        }

        // Binary: 0b... or 0b-...
        // Reject if contains decimal point (treat as symbol)
        if word_without_sign.starts_with("0b") || word_without_sign.starts_with("0B") {
            let bin_str = &word_without_sign[2..];
            // Handle 0b-... format (minus after prefix)
            let (bin_negative, bin_digits) = if bin_str.starts_with('-') {
                (true, &bin_str[1..])
            } else if bin_str.starts_with('+') {
                (false, &bin_str[1..])
            } else {
                (false, bin_str)
            };

            // Reject fractional binary numbers (treat as symbol)
            if !bin_digits.contains('.') {
                if let Ok(value) = Float::parse_radix(bin_digits, 2) {
                    let mut result = value.complete(128);
                    if is_negative ^ bin_negative {
                        // XOR: negate if exactly one minus sign
                        result = -result;
                    }
                    return Ok(Token::Number {
                        value: result,
                        base: 2,
                    });
                }
            }
        }

        // Arbitrary base: Nb... or Nb-...
        // Format: <base>b<digits>
        // Reject if contains decimal point (treat as symbol)
        if let Some(b_pos) = word_without_sign.find(|c| c == 'b' || c == 'B') {
            if b_pos > 0 && b_pos < word_without_sign.len() - 1 {
                let base_str = &word_without_sign[..b_pos];
                let digits_str = &word_without_sign[b_pos + 1..];

                // Handle Nb-... format (minus after 'b')
                let (base_negative, base_digits) = if digits_str.starts_with('-') {
                    (true, &digits_str[1..])
                } else if digits_str.starts_with('+') {
                    (false, &digits_str[1..])
                } else {
                    (false, digits_str)
                };

                // Reject fractional base numbers (treat as symbol)
                if !base_digits.contains('.') {
                    if let Ok(base) = base_str.parse::<i32>() {
                        if base >= 2 && base <= 62 {
                            // rug's parse_radix only supports bases 2-36
                            // For bases 37-62, we need custom parsing
                            let value = if base <= 36 {
                                Float::parse_radix(base_digits, base)
                                    .ok()
                                    .map(|v| v.complete(128))
                            } else {
                                parse_base_37_to_62(base_digits, base)
                            };

                            if let Some(mut value) = value {
                                if is_negative ^ base_negative {
                                    // XOR: negate if exactly one minus sign
                                    value = -value;
                                }
                                return Ok(Token::Number {
                                    value,
                                    base: base as u8,
                                });
                            }
                        }
                    }
                }
            }
        }

        // Decimal: standard float (use original word to preserve the sign)
        if let Ok(value) = Float::parse(word) {
            return Ok(Token::Number {
                value: value.complete(128),
                base: 10,
            });
        }

        Err(Error::SyntaxError(format!("not a number: {}", word)))
    }
}
