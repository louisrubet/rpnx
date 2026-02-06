// SPDX-License-Identifier: GPL-3.0-only
// Helper macros for operation implementations

/// Check minimum number of arguments on stack
/// Returns MissingOperand error if insufficient
#[macro_export]
macro_rules! min_arguments {
    ($ctx:expr, $n:expr) => {
        if $ctx.stack.len() < $n {
            return Err($crate::error::Error::MissingOperand);
        }
    };
}

/// Check that argument at level is of specific object type
#[macro_export]
macro_rules! arg_must_be {
    ($ctx:expr, $level:expr, Number) => {
        if !matches!(
            $ctx.stack.get($level),
            Some($crate::object::Object::Number { .. })
        ) {
            return Err($crate::error::Error::BadOperandType);
        }
    };
    ($ctx:expr, $level:expr, Complex) => {
        if !matches!(
            $ctx.stack.get($level),
            Some($crate::object::Object::Complex { .. })
        ) {
            return Err($crate::error::Error::BadOperandType);
        }
    };
    ($ctx:expr, $level:expr, String) => {
        if !matches!(
            $ctx.stack.get($level),
            Some($crate::object::Object::String(_))
        ) {
            return Err($crate::error::Error::BadOperandType);
        }
    };
    ($ctx:expr, $level:expr, Program) => {
        if !matches!(
            $ctx.stack.get($level),
            Some($crate::object::Object::Program(_))
        ) {
            return Err($crate::error::Error::BadOperandType);
        }
    };
}

/// Pop two numbers from stack, returning their Float values
#[macro_export]
macro_rules! pop_two_numbers {
    ($ctx:expr) => {{
        min_arguments!($ctx, 2);
        arg_must_be!($ctx, 0, Number);
        arg_must_be!($ctx, 1, Number);

        let b = match $ctx.stack.pop().unwrap() {
            $crate::object::Object::Number { value, .. } => value,
            _ => unreachable!(),
        };
        let a = match $ctx.stack.pop().unwrap() {
            $crate::object::Object::Number { value, .. } => value,
            _ => unreachable!(),
        };
        (a, b)
    }};
}

/// Pop one number from stack, returning its Float value
#[macro_export]
macro_rules! pop_one_number {
    ($ctx:expr) => {{
        min_arguments!($ctx, 1);
        arg_must_be!($ctx, 0, Number);

        match $ctx.stack.pop().unwrap() {
            $crate::object::Object::Number { value, .. } => value,
            _ => unreachable!(),
        }
    }};
}

/// Push a Float value as a Number object onto the stack
#[macro_export]
macro_rules! push_number {
    ($ctx:expr, $value:expr) => {
        $ctx.stack.push($crate::object::Object::Number {
            value: $value,
            base: 10,
        });
    };
    ($ctx:expr, $value:expr, $base:expr) => {
        $ctx.stack.push($crate::object::Object::Number {
            value: $value,
            base: $base,
        });
    };
}
