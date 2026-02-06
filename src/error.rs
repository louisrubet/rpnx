// SPDX-License-Identifier: GPL-3.0-only
// Error types for RPN calculator

use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum Error {
    #[error("unknown error")]
    UnknownError,

    #[error("missing operand")]
    MissingOperand,

    #[error("bad operand type")]
    BadOperandType,

    #[error("out of range")]
    OutOfRange,

    #[error("unknown variable: {0}")]
    UnknownVariable(String),

    #[error("internal error: {0}")]
    InternalError(String),

    #[error("deadly error: {0}")]
    DeadlyError(String),

    #[error("goodbye")]
    Goodbye, // Special error for quit command

    #[error("edit: {0}")]
    Edit(String), // Special signal for edit command - carries content to edit

    #[error("not implemented: {0}")]
    NotImplemented(String),

    #[error("syntax error: {0}")]
    SyntaxError(String),

    #[error("division by zero")]
    DivByZero,

    #[error("runtime error: {0}")]
    RuntimeError(String),

    #[error("abort current entry")]
    AbortCurrentEntry,

    #[error("out of memory")]
    OutOfMemory,

    #[error("bad value: {0}")]
    BadValue(String),

    #[error("test failed: {0}")]
    TestFailed(String),
}

impl Error {
    /// Get the integer error code for this error
    /// kOk=0, kUnknownError=1, kMissingOperand=2, kBadOperandType=3,
    /// kOutOfRange=4, kUnknownVariable=5, kInternalError=6, kDeadlyError=7,
    /// kGoodbye=8, kNotImplemented=9, kNop=10, kSyntaxError=11,
    /// kDivByZero=12, kRuntimeError=13, kAbortCurrentEntry=14,
    /// kOutOfMemory=15, kBadValue=16, kTestFailed=17
    pub fn error_code(&self) -> i32 {
        match self {
            Error::UnknownError => 1,
            Error::MissingOperand => 2,
            Error::BadOperandType => 3,
            Error::OutOfRange => 4,
            Error::UnknownVariable(_) => 5,
            Error::InternalError(_) => 6,
            Error::DeadlyError(_) => 7,
            Error::Goodbye => 8,
            Error::Edit(_) => 8, // Not a real error, use same as Goodbye
            Error::NotImplemented(_) => 9,
            // kNop=10 doesn't exist in Rust enum
            Error::SyntaxError(_) => 11,
            Error::DivByZero => 12,
            Error::RuntimeError(_) => 13,
            Error::AbortCurrentEntry => 14,
            Error::OutOfMemory => 15,
            Error::BadValue(_) => 16,
            Error::TestFailed(_) => 17,
        }
    }

    /// Get the error name as a string (without details)
    pub fn error_name(&self) -> &'static str {
        Self::name_from_code(self.error_code())
    }

    /// Get the error name for a given error code
    pub fn name_from_code(code: i32) -> &'static str {
        match code {
            0 => "ok",
            1 => "unknown error",
            2 => "missing operand",
            3 => "bad operand type",
            4 => "out of range",
            5 => "unknown variable",
            6 => "internal error",
            7 => "deadly error",
            8 => "goodbye",
            9 => "not implemented",
            10 => "nop",
            11 => "syntax error",
            12 => "division by zero",
            13 => "runtime error",
            14 => "abort current entry",
            15 => "out of memory",
            16 => "bad value",
            17 => "test failed",
            _ => "unknown error code",
        }
    }
}

/// Result type alias for RPN operations
pub type Result<T> = std::result::Result<T, Error>;
