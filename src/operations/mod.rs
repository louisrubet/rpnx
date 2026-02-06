// SPDX-License-Identifier: GPL-3.0-only
// Operations modules for RPN calculator
// Registry of all built-in operations

use crate::object::{BranchFn, KeywordFn};
use lazy_static::lazy_static;
use std::collections::HashMap;

// Operation modules
pub mod branch;
pub mod complex;
pub mod general;
pub mod logs;
pub mod program_ops;
pub mod real;
pub mod stack_ops;
pub mod store;
pub mod string;
pub mod test;
pub mod trig;
// pub mod time;

// Placeholder functions removed - using real implementations from stack_ops.rs

lazy_static! {
    /// Keyword registry mapping names to function pointers
    pub static ref KEYWORDS: HashMap<&'static str, KeywordFn> = {
        let mut m = HashMap::new();

        // Arithmetic operations (real.rs)
        m.insert("+", real::plus as KeywordFn);
        m.insert("-", real::minus as KeywordFn);
        m.insert("*", real::multiply as KeywordFn);
        m.insert("/", real::divide as KeywordFn);
        m.insert("neg", real::neg as KeywordFn);
        m.insert("chs", real::neg as KeywordFn);
        m.insert("inv", real::inv as KeywordFn);
        m.insert("^", real::power as KeywordFn);
        m.insert("pow", real::power as KeywordFn);
        m.insert("sqrt", real::sqrt as KeywordFn);
        m.insert("sq", real::sq as KeywordFn);
        m.insert("abs", real::abs as KeywordFn);
        m.insert("sign", real::sign as KeywordFn);
        m.insert("min", real::min as KeywordFn);
        m.insert("max", real::max as KeywordFn);
        m.insert("mod", real::modulo as KeywordFn);
        m.insert("%", real::percent as KeywordFn);
        m.insert("%inv", real::percent_change as KeywordFn);
        m.insert("fact", real::factorial as KeywordFn);
        m.insert("floor", real::floor as KeywordFn);
        m.insert("ceil", real::ceil as KeywordFn);
        m.insert("ip", real::integer_part as KeywordFn);
        m.insert("fp", real::fractional_part as KeywordFn);
        m.insert("mant", real::mantissa as KeywordFn);
        m.insert("xpon", real::exponent as KeywordFn);

        // Bitwise operations
        m.insert("&", real::bitwise_and as KeywordFn);
        m.insert("|", real::bitwise_or as KeywordFn);
        m.insert("^", real::bitwise_xor as KeywordFn);
        m.insert("~", real::bitwise_not as KeywordFn);

        // Stack operations (stack_ops.rs)
        m.insert("swap", stack_ops::swap as KeywordFn);
        m.insert("dup", stack_ops::dup as KeywordFn);
        m.insert("dup2", stack_ops::dup2 as KeywordFn);
        m.insert("dupn", stack_ops::dupn as KeywordFn);
        m.insert("drop", stack_ops::drop as KeywordFn);
        m.insert("pop", stack_ops::drop as KeywordFn);
        m.insert("drop2", stack_ops::drop2 as KeywordFn);
        m.insert("dropn", stack_ops::dropn as KeywordFn);
        m.insert("erase", stack_ops::erase as KeywordFn);
        m.insert("del", stack_ops::erase as KeywordFn);
        m.insert("clear", stack_ops::erase as KeywordFn);
        m.insert("pick", stack_ops::pick as KeywordFn);
        m.insert("rot", stack_ops::rot as KeywordFn);
        m.insert("roll", stack_ops::roll as KeywordFn);
        m.insert("rolld", stack_ops::rolld as KeywordFn);
        m.insert("over", stack_ops::over as KeywordFn);
        m.insert("depth", stack_ops::depth as KeywordFn);
        m.insert("edit", stack_ops::edit as KeywordFn);

        // Comparison operations (test.rs)
        m.insert(">", test::greater as KeywordFn);
        m.insert(">=", test::greater_eq as KeywordFn);
        m.insert("<", test::less as KeywordFn);
        m.insert("<=", test::less_eq as KeywordFn);
        m.insert("==", test::equal as KeywordFn);
        m.insert("!=", test::not_equal as KeywordFn);
        m.insert("and", test::and as KeywordFn);
        m.insert("or", test::or as KeywordFn);
        m.insert("xor", test::xor as KeywordFn);
        m.insert("not", test::not as KeywordFn);
        m.insert("same", test::same as KeywordFn);

        // General operations (general.rs)
        m.insert("quit", general::quit as KeywordFn);
        m.insert("q", general::quit as KeywordFn);
        m.insert("exit", general::quit as KeywordFn);
        m.insert("help", general::help as KeywordFn);
        m.insert("h", general::help as KeywordFn);
        m.insert("?", general::help as KeywordFn);
        m.insert("version", general::version as KeywordFn);
        m.insert("uname", general::uname as KeywordFn);
        m.insert("type", general::type_of as KeywordFn);
        m.insert("test", general::test as KeywordFn);
        m.insert("std", general::std_mode as KeywordFn);
        m.insert("fix", general::fix_mode as KeywordFn);
        m.insert("sci", general::sci_mode as KeywordFn);
        m.insert("prec", general::precision as KeywordFn);
        m.insert("default", general::default as KeywordFn);
        m.insert("hex", general::hex as KeywordFn);
        m.insert("dec", general::dec as KeywordFn);
        m.insert("bin", general::bin as KeywordFn);
        m.insert("base", general::base as KeywordFn);
        m.insert("date", general::date as KeywordFn);
        m.insert("time", general::time as KeywordFn);
        m.insert("ticks", general::ticks as KeywordFn);
        m.insert("error", general::error as KeywordFn);
        m.insert("strerror", general::strerror as KeywordFn);
        m.insert("history", general::history as KeywordFn);

        // Variable storage operations (store.rs)
        m.insert("sto", store::sto as KeywordFn);
        m.insert("rcl", store::rcl as KeywordFn);
        m.insert("purge", store::purge as KeywordFn);
        m.insert("sto+", store::sto_add as KeywordFn);
        m.insert("sto-", store::sto_sub as KeywordFn);
        m.insert("sto*", store::sto_mul as KeywordFn);
        m.insert("sto/", store::sto_div as KeywordFn);
        m.insert("stoneg", store::sto_neg as KeywordFn);
        m.insert("sneg", store::sto_neg as KeywordFn); // Alias for stoneg
        m.insert("stoinv", store::sto_inv as KeywordFn);
        m.insert("sinv", store::sto_inv as KeywordFn); // Alias for stoinv
        m.insert("vars", store::vars as KeywordFn);
        m.insert("clusr", store::clusr as KeywordFn);

        // Complex number operations (complex.rs)
        m.insert("re", complex::re as KeywordFn);
        m.insert("im", complex::im as KeywordFn);
        m.insert("arg", complex::arg as KeywordFn);
        m.insert("conj", complex::conj as KeywordFn);
        m.insert("c->r", complex::c_to_r as KeywordFn);
        m.insert("r->c", complex::r_to_c as KeywordFn);
        m.insert("p->r", complex::p_to_r as KeywordFn);
        m.insert("r->p", complex::r_to_p as KeywordFn);

        // String operations (string.rs)
        m.insert("->str", string::to_str as KeywordFn);
        m.insert("str->", string::str_to as KeywordFn);
        m.insert("chr", string::chr as KeywordFn);
        m.insert("num", string::num as KeywordFn);
        m.insert("size", string::size as KeywordFn);
        m.insert("pos", string::pos as KeywordFn);
        m.insert("sub", string::sub as KeywordFn);
        m.insert("endl", string::endl as KeywordFn);

        // Trigonometric operations (trig.rs)
        m.insert("sin", trig::sin as KeywordFn);
        m.insert("cos", trig::cos as KeywordFn);
        m.insert("tan", trig::tan as KeywordFn);
        m.insert("asin", trig::asin as KeywordFn);
        m.insert("acos", trig::acos as KeywordFn);
        m.insert("atan", trig::atan as KeywordFn);
        m.insert("atan2", trig::atan2 as KeywordFn);
        m.insert("d->r", trig::d_to_r as KeywordFn);
        m.insert("r->d", trig::r_to_d as KeywordFn);
        m.insert("pi", trig::pi as KeywordFn);
        m.insert("sinh", trig::sinh as KeywordFn);
        m.insert("cosh", trig::cosh as KeywordFn);
        m.insert("tanh", trig::tanh as KeywordFn);
        m.insert("asinh", trig::asinh as KeywordFn);
        m.insert("acosh", trig::acosh as KeywordFn);
        m.insert("atanh", trig::atanh as KeywordFn);

        // Logarithmic operations (logs.rs)
        m.insert("ln", logs::ln as KeywordFn);
        m.insert("log", logs::ln as KeywordFn); // Alias for ln
        m.insert("exp", logs::exp as KeywordFn);
        m.insert("log10", logs::log10 as KeywordFn);
        m.insert("alog10", logs::alog10 as KeywordFn);
        m.insert("exp10", logs::alog10 as KeywordFn); // Alias for alog10
        m.insert("log2", logs::log2 as KeywordFn);
        m.insert("alog2", logs::alog2 as KeywordFn);
        m.insert("exp2", logs::alog2 as KeywordFn); // Alias for alog2
        m.insert("lnp1", logs::lnp1 as KeywordFn);
        m.insert("expm", logs::expm as KeywordFn);
        m.insert("e", logs::e as KeywordFn);
        m.insert("logn", logs::logn as KeywordFn); // Arbitrary base logarithm (2 args)
        m.insert("alogn", logs::alogn as KeywordFn); // Arbitrary base antilog (2 args)

        // Program operations (program_ops.rs)
        m.insert("eval", program_ops::eval as KeywordFn);

        m
    };

    /// Branch keyword registry (control flow)
    pub static ref BRANCHES: HashMap<&'static str, BranchFn> = {
        let mut m = HashMap::new();

        // IF/THEN/ELSE/END control flow
        m.insert("if", branch::rpnx_if as BranchFn);
        m.insert("then", branch::rpnx_then as BranchFn);
        m.insert("else", branch::rpnx_else as BranchFn);
        m.insert("end", branch::rpnx_end as BranchFn);

        // Inline IF operations
        m.insert("ift", branch::rpnx_ift as BranchFn);
        m.insert("ifte", branch::rpnx_ifte as BranchFn);

        // FOR/NEXT/STEP loops
        m.insert("for", branch::rpnx_for as BranchFn);
        m.insert("next", branch::rpnx_next as BranchFn);
        m.insert("step", branch::rpnx_step as BranchFn);
        m.insert("start", branch::rpnx_start as BranchFn);

        // WHILE/REPEAT/END loops
        m.insert("while", branch::rpnx_while as BranchFn);
        m.insert("repeat", branch::rpnx_repeat as BranchFn);

        // DO/UNTIL/END loops
        m.insert("do", branch::rpnx_do as BranchFn);
        m.insert("until", branch::rpnx_until as BranchFn);

        m
    };
}

/// Check if a word is a keyword
pub fn is_keyword(word: &str) -> bool {
    KEYWORDS.contains_key(word)
}

/// Check if a word is a branch keyword
pub fn is_branch(word: &str) -> bool {
    BRANCHES.contains_key(word)
}

/// Get keyword function pointer
pub fn get_keyword(word: &str) -> Option<KeywordFn> {
    KEYWORDS.get(word).copied()
}

/// Get branch function pointer
pub fn get_branch(word: &str) -> Option<BranchFn> {
    BRANCHES.get(word).copied()
}
