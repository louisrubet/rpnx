// SPDX-License-Identifier: GPL-3.0-only
// Object types for RPN calculator

use crate::context::Context;
use crate::error::Result;
use crate::number::{format_number, NumberConfig};
use rug::{Complex, Float};

#[derive(Debug, Clone)]
pub struct Program {
    pub objects: Vec<Object>,
    pub value: String,
    pub local_vars: Vec<String>, // Local variable names from -> declaration
    pub prefix_objects: Vec<Object>, // Code to execute before binding local vars
    pub auto_eval: bool,         // If true, auto-evaluate when encountered in run_objects
}

/// Branch arguments for control flow operations
#[derive(Debug, Clone, Copy)]
pub struct BranchArgs {
    pub arg1: usize,
    pub arg2: usize,
    pub arg3: usize,
    pub first_index: f64, // Start value for loops (never modified after init)
    pub last_index: f64,  // End value for loops
    pub arg_bool: bool,
    pub condition: usize,       // For IF: stores condition result (0 or 1)
    pub iteration_count: usize, // For STEP: tracks iteration number to avoid accumulation errors
    pub step_value: f64,        // For STEP: stores the step increment
}

impl Default for BranchArgs {
    fn default() -> Self {
        Self {
            arg1: usize::MAX,
            arg2: usize::MAX,
            arg3: usize::MAX,
            first_index: 0.0,
            last_index: 0.0,
            arg_bool: false,
            condition: 0,
            iteration_count: 0,
            step_value: 1.0,
        }
    }
}

/// Function pointer type for keyword operations
pub type KeywordFn = fn(&mut Context) -> Result<()>;

/// Function pointer type for branch operations (returns next instruction index)
pub type BranchFn = fn(&mut Context, &mut BranchArgs) -> Result<usize>;

/// Object enum representing all stack object types
#[derive(Debug, Clone)]
pub enum Object {
    /// Number with arbitrary precision (MPFR-based)
    /// Format: value in given base (2-62)
    Number { value: Float, base: u8 },

    /// Complex number with arbitrary precision
    /// Both real and imaginary parts have separate bases
    Complex {
        value: Complex,
        re_base: u8,
        im_base: u8,
    },

    /// String value
    String(String),

    /// Symbol (variable name or auto-eval reference)
    Symbol { name: String, auto_eval: bool },

    /// Program (sequence of objects)
    /// Can be nested and evaluated
    Program(Box<Program>),

    /// Keyword (built-in operation)
    /// Handler is a function pointer to the operation
    Keyword {
        name: &'static str,
        handler: KeywordFn,
    },

    /// Branch (control flow keyword)
    /// Handler returns next instruction index
    Branch {
        name: &'static str,
        handler: BranchFn,
        args: BranchArgs,
    },
}

impl Object {
    /// Get the type name of this object (for debugging/display)
    pub fn type_name(&self) -> &'static str {
        match self {
            Object::Number { .. } => "number",
            Object::Complex { .. } => "complex",
            Object::String(_) => "string",
            Object::Symbol { .. } => "symbol",
            Object::Program(_) => "program",
            Object::Keyword { .. } => "keyword",
            Object::Branch { .. } => "branch",
        }
    }

    /// Display this object as a string
    pub fn display(&self, config: &NumberConfig) -> String {
        match self {
            Object::Number { value, base } => {
                format_number(value, *base, config.mode, config.digits)
            }
            Object::Complex {
                value,
                re_base,
                im_base,
            } => {
                let re_str = format_number(value.real(), *re_base, config.mode, config.digits);
                let im_str = format_number(value.imag(), *im_base, config.mode, config.digits);
                format!("({},{})", re_str, im_str)
            }
            Object::String(s) => format!("\"{}\"", s),
            Object::Symbol { name, auto_eval } => {
                if *auto_eval {
                    name.clone()
                } else {
                    format!("'{}'", name)
                }
            }
            Object::Program(p) => {
                // Display with French guillemets and normalized spacing
                if !p.local_vars.is_empty() {
                    // Program with local variables: « [prefix] -> var1 var2 ... « body » »
                    let mut result = String::from("« ");

                    // Display prefix code if present
                    if !p.prefix_objects.is_empty() {
                        let prefix_parts: Vec<String> = p
                            .prefix_objects
                            .iter()
                            .map(|obj| obj.display(config))
                            .collect();
                        result.push_str(&prefix_parts.join(" "));
                        result.push_str(" ");
                    }

                    result.push_str("-> ");
                    result.push_str(&p.local_vars.join(" "));
                    result.push_str(" « ");

                    // Display the body objects
                    let inner_parts: Vec<String> =
                        p.objects.iter().map(|obj| obj.display(config)).collect();
                    result.push_str(&inner_parts.join(" "));

                    result.push_str(" » »");
                    result
                } else {
                    // Normal program without local variables
                    let mut all_objects = p.prefix_objects.clone();
                    all_objects.extend(p.objects.clone());

                    if all_objects.is_empty() {
                        "«  »".to_string()
                    } else {
                        // Display objects with normalized spacing
                        let parts: Vec<String> =
                            all_objects.iter().map(|obj| obj.display(config)).collect();
                        format!("« {} »", parts.join(" "))
                    }
                }
            }
            Object::Keyword { name, .. } => name.to_string(),
            Object::Branch { name, .. } => format!("<branch:{}>", name),
        }
    }
}
