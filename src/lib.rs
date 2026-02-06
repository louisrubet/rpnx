// SPDX-License-Identifier: GPL-3.0-only
//! # rpnx
//!
//! A modern Reverse Polish Notation (RPN) calculator with arbitrary precision arithmetic.
//!
//! This library provides a complete RPN calculator implementation with:
//! - Arbitrary precision arithmetic using MPFR/GMP
//! - 145+ mathematical, logical, and stack manipulation operations
//! - Complex number support
//! - Multiple number bases (2-62)
//! - String operations
//! - Control flow (if/then/else, for/next, while/repeat)
//! - Variables with three-level scoping
//! - Programs as first-class objects
//!
//! ## Quick Start
//!
//! ```no_run
//! use rpnx::{Context, program};
//!
//! let mut ctx = Context::new();
//!
//! // Simple calculation: 2 + 3
//! program::execute("2 3 +", &mut ctx).unwrap();
//! println!("Result: {}", ctx.stack.get(0).unwrap().display(&ctx.config));
//!
//! // With variables
//! program::execute("42 'answer' sto answer", &mut ctx).unwrap();
//! ```
//!
//! ## Architecture
//!
//! The library is organized into several core modules:
//!
//! - [`object`]: Tagged enum representing all RPN object types
//! - [`stack`]: Stack and heap data structures
//! - [`context`]: Execution context with stack, heaps, and configuration
//! - [`lexer`]: Tokenization of RPN expressions
//! - [`parser`]: Token to object conversion
//! - [`program`]: Program execution engine
//! - [`operations`]: All built-in operations (145+)
//! - [`repl`]: Interactive REPL interface
//!
//! ## Example: Custom Calculator
//!
//! ```no_run
//! use rpnx::{Context, program};
//!
//! fn calculate(expression: &str) -> Result<String, Box<dyn std::error::Error>> {
//!     let mut ctx = Context::new();
//!     program::execute(expression, &mut ctx)?;
//!
//!     if let Some(result) = ctx.stack.get(0) {
//!         Ok(result.display(&ctx.config))
//!     } else {
//!         Ok("(empty stack)".to_string())
//!     }
//! }
//!
//! let result = calculate("pi 2 / sin")?;
//! println!("sin(π/2) = {}", result);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

#[macro_use]
pub mod macros;

pub mod context;
pub mod error;
pub mod lexer;
pub mod number;
pub mod object;
pub mod operations;
pub mod parser;
pub mod program;
pub mod repl;
pub mod stack;
pub mod test_framework;

// Re-export commonly used types
pub use context::Context;
pub use error::{Error, Result};
pub use object::Object;
pub use stack::{Heap, RpnStack};
