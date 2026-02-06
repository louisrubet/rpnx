// SPDX-License-Identifier: GPL-3.0-only
// Parser for converting tokens to objects

use crate::error::{Error, Result};
use crate::lexer::Token;
use crate::object::{BranchArgs, Object};
use crate::operations;

/// Parser converts tokens to objects
pub struct Parser;

impl Parser {
    /// Build objects from tokens
    /// This is the main parsing function that converts lexer output to executable objects
    pub fn build_objects(tokens: Vec<Token>) -> Result<Vec<Object>> {
        let mut objects = Vec::new();

        for token in tokens {
            let obj = Self::token_to_object(token)?;
            objects.push(obj);
        }

        Ok(objects)
    }

    /// Extract local variable declarations from program tokens
    /// Pattern: [prefix_code...] -> var1 var2 ... << body >>
    /// Returns (prefix_objects, variable_names, body_objects)
    fn extract_local_vars(tokens: Vec<Token>) -> Result<(Vec<Object>, Vec<String>, Vec<Object>)> {
        if tokens.is_empty() {
            return Ok((Vec::new(), Vec::new(), Vec::new()));
        }

        // Look for '->' anywhere in the token list
        let mut arrow_index = None;
        for (i, token) in tokens.iter().enumerate() {
            if let Token::Symbol {
                name,
                quoted: false,
            } = token
            {
                if name == "->" {
                    arrow_index = Some(i);
                    break;
                } else if name.starts_with("->") && name.len() > 2 {
                    // Handle tight spacing like <<->n<<1>>
                    arrow_index = Some(i);
                    break;
                }
            }
        }

        if let Some(arrow_idx) = arrow_index {
            // Found local variable declaration
            let mut var_names = Vec::new();
            let mut prefix_objects = Vec::new();

            // Parse everything before -> as prefix code
            if arrow_idx > 0 {
                let prefix_tokens = tokens[..arrow_idx].to_vec();
                prefix_objects = Self::build_objects(prefix_tokens)?;
            }

            // Check if arrow token has embedded variable name (tight spacing)
            let first_var_name = if let Token::Symbol {
                name,
                quoted: false,
            } = &tokens[arrow_idx]
            {
                if name.starts_with("->") && name.len() > 2 {
                    Some(name[2..].to_string())
                } else {
                    None
                }
            } else {
                None
            };

            let mut i = arrow_idx + 1;

            // If we extracted a variable name from the arrow token
            if let Some(first_var) = first_var_name {
                var_names.push(first_var);
            }

            // Collect variable names until we hit a Program token
            while i < tokens.len() {
                match &tokens[i] {
                    Token::Symbol {
                        name,
                        quoted: false,
                    } => {
                        var_names.push(name.clone());
                        i += 1;
                    }
                    Token::Program(_) => {
                        // Found the inner program
                        break;
                    }
                    _ => {
                        // Not a valid local variable declaration
                        // The -> was probably part of a keyword like ->str
                        // Fall through to parse normally
                        let objects = Self::build_objects(tokens)?;
                        return Ok((Vec::new(), Vec::new(), objects));
                    }
                }
            }

            // The next token should be the Program (the body)
            if i < tokens.len() {
                if let Token::Program(content) = &tokens[i] {
                    // Parse the inner program (body)
                    // The body might itself have a -> pattern, so we recursively check
                    use crate::lexer::Lexer;
                    let inner_tokens = Lexer::analyze(content)?;
                    let (inner_prefix, inner_vars, inner_body) =
                        Self::extract_local_vars(inner_tokens)?;

                    let mut body_objects = Vec::new();

                    // If the body has its own local vars, create a nested Program object
                    if !inner_vars.is_empty() || !inner_prefix.is_empty() {
                        use crate::object::Program;
                        let nested_program = Object::Program(Box::new(Program {
                            objects: inner_body,
                            value: content.clone(),
                            local_vars: inner_vars,
                            prefix_objects: inner_prefix,
                            auto_eval: true, // Nested programs with local vars should auto-eval
                        }));
                        body_objects.push(nested_program);
                    } else {
                        // No nested local vars, just use the body objects directly
                        body_objects.extend(inner_body);
                    }

                    // Process remaining tokens after the Program (e.g., eval, other commands)
                    i += 1;
                    if i < tokens.len() {
                        let remaining_tokens = tokens[i..].to_vec();
                        let remaining_objects = Self::build_objects(remaining_tokens)?;
                        body_objects.extend(remaining_objects);
                    }

                    return Ok((prefix_objects, var_names, body_objects));
                }
            }

            // If we didn't find a Program token, this isn't actually a local variable declaration
            // It's probably a keyword like "->str" that happens to start with ->
            // Fall through to parse normally
        }

        // No local variable declaration (or false positive), parse normally
        let objects = Self::build_objects(tokens)?;
        Ok((Vec::new(), Vec::new(), objects))
    }

    /// Convert a single token to an object
    fn token_to_object(token: Token) -> Result<Object> {
        match token {
            Token::Number { value, base } => Ok(Object::Number { value, base }),

            Token::Complex {
                re,
                im,
                re_base,
                im_base,
            } => {
                use rug::Complex;
                let value = Complex::with_val(128, (re, im));
                Ok(Object::Complex {
                    value,
                    re_base,
                    im_base,
                })
            }

            Token::String(s) => Ok(Object::String(s)),

            Token::Symbol { name, quoted } => {
                // If quoted, it's always a symbol
                if quoted {
                    return Ok(Object::Symbol {
                        name,
                        auto_eval: false,
                    });
                }

                // Check if it's a keyword
                if let Some(handler) = operations::get_keyword(&name) {
                    return Ok(Object::Keyword {
                        name: Box::leak(name.into_boxed_str()),
                        handler,
                    });
                }

                // Check if it's a branch keyword
                if let Some(handler) = operations::get_branch(&name) {
                    return Ok(Object::Branch {
                        name: Box::leak(name.into_boxed_str()),
                        handler,
                        args: BranchArgs::default(),
                    });
                }

                // Otherwise, it's an auto-eval symbol (variable reference)
                Ok(Object::Symbol {
                    name,
                    auto_eval: true,
                })
            }

            Token::Program(content) => {
                // Recursively parse the program content
                use crate::lexer::Lexer;
                let tokens = Lexer::analyze(&content)?;

                // Check for local variable declaration: -> var1 var2 ... << body >>
                let (mut prefix_objects, local_vars, mut body_objects) =
                    Self::extract_local_vars(tokens)?;

                // Link control flow for both prefix and body objects
                crate::program::link_control_flow(&mut prefix_objects)?;
                crate::program::link_control_flow(&mut body_objects)?;

                use crate::object::Program;
                Ok(Object::Program(Box::new(Program {
                    objects: body_objects,
                    value: content,
                    local_vars,
                    prefix_objects,
                    auto_eval: false, // Top-level programs don't auto-eval
                })))
            }

            Token::Keyword(name) => {
                if let Some(handler) = operations::get_keyword(&name) {
                    Ok(Object::Keyword {
                        name: Box::leak(name.into_boxed_str()),
                        handler,
                    })
                } else {
                    Err(Error::SyntaxError(format!("unknown keyword: {}", name)))
                }
            }

            Token::Branch(name) => {
                if let Some(handler) = operations::get_branch(&name) {
                    Ok(Object::Branch {
                        name: Box::leak(name.into_boxed_str()),
                        handler,
                        args: BranchArgs::default(),
                    })
                } else {
                    Err(Error::SyntaxError(format!(
                        "unknown branch keyword: {}",
                        name
                    )))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_numbers() {
        let tokens = vec![
            Token::Number {
                value: rug::Float::with_val(128, 2),
                base: 10,
            },
            Token::Number {
                value: rug::Float::with_val(128, 3),
                base: 10,
            },
        ];

        let objects = Parser::build_objects(tokens).unwrap();
        assert_eq!(objects.len(), 2);

        match &objects[0] {
            Object::Number { base, .. } => assert_eq!(*base, 10),
            _ => panic!("Expected Number"),
        }
    }
}
