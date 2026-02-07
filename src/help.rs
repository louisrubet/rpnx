// SPDX-License-Identifier: GPL-3.0-only
// Command help database

use std::collections::HashMap;
use lazy_static::lazy_static;

/// Help entry for a command
pub struct CommandHelp {
    /// Command name
    pub name: &'static str,
    /// One-line description
    pub description: &'static str,
    /// Syntax with arguments
    pub syntax: &'static str,
    /// Argument descriptions (can be empty)
    pub args: &'static [(&'static str, &'static str)],
    /// Example usage
    pub example: &'static str,
}

lazy_static! {
    pub static ref HELP_DB: HashMap<&'static str, CommandHelp> = {
        let mut m = HashMap::new();

        // Arithmetic operations
        m.insert("+", CommandHelp {
            name: "+",
            description: "Add two numbers or concatenate symbols",
            syntax: "a b +",
            args: &[
                ("a", "number, complex, or symbol"),
                ("b", "number, complex, or symbol"),
            ],
            example: "1 2 +",
        });
        m.insert("-", CommandHelp {
            name: "-",
            description: "Subtract second number from first",
            syntax: "a b -",
            args: &[
                ("a", "number or complex"),
                ("b", "number or complex"),
            ],
            example: "5 3 -",
        });
        m.insert("*", CommandHelp {
            name: "*",
            description: "Multiply two numbers",
            syntax: "a b *",
            args: &[
                ("a", "number or complex"),
                ("b", "number or complex"),
            ],
            example: "3 4 *",
        });
        m.insert("/", CommandHelp {
            name: "/",
            description: "Divide first number by second",
            syntax: "a b /",
            args: &[
                ("a", "number or complex"),
                ("b", "number or complex, non-zero"),
            ],
            example: "10 2 /",
        });
        m.insert("neg", CommandHelp {
            name: "neg",
            description: "Negate a number (change sign)",
            syntax: "x neg",
            args: &[("x", "number or complex")],
            example: "5 neg",
        });
        m.insert("chs", CommandHelp {
            name: "chs",
            description: "Change sign (alias for neg)",
            syntax: "x chs",
            args: &[("x", "number or complex")],
            example: "-3 chs",
        });
        m.insert("inv", CommandHelp {
            name: "inv",
            description: "Compute multiplicative inverse (1/x)",
            syntax: "x inv",
            args: &[("x", "number or complex, non-zero")],
            example: "4 inv",
        });
        m.insert("pow", CommandHelp {
            name: "pow",
            description: "Raise first number to power of second",
            syntax: "x y pow",
            args: &[
                ("x", "base, number or complex"),
                ("y", "exponent, number or complex"),
            ],
            example: "2 10 pow",
        });
        m.insert("^", CommandHelp {
            name: "^",
            description: "Bitwise XOR of two integers",
            syntax: "a b ^",
            args: &[
                ("a", "integer"),
                ("b", "integer"),
            ],
            example: "0b1111 0b1010 ^",
        });
        m.insert("sqrt", CommandHelp {
            name: "sqrt",
            description: "Compute square root",
            syntax: "x sqrt",
            args: &[("x", "number or complex")],
            example: "9 sqrt",
        });
        m.insert("sq", CommandHelp {
            name: "sq",
            description: "Compute square (x*x)",
            syntax: "x sq",
            args: &[("x", "number or complex")],
            example: "5 sq",
        });
        m.insert("abs", CommandHelp {
            name: "abs",
            description: "Absolute value of real or magnitude of complex",
            syntax: "x abs",
            args: &[("x", "number or complex")],
            example: "-5 abs",
        });
        m.insert("sign", CommandHelp {
            name: "sign",
            description: "Sign of real (-1, 0, 1) or unit vector for complex",
            syntax: "x sign",
            args: &[("x", "number or complex")],
            example: "-42 sign",
        });
        m.insert("min", CommandHelp {
            name: "min",
            description: "Return the smaller of two numbers",
            syntax: "a b min",
            args: &[
                ("a", "number"),
                ("b", "number"),
            ],
            example: "3 7 min",
        });
        m.insert("max", CommandHelp {
            name: "max",
            description: "Return the larger of two numbers",
            syntax: "a b max",
            args: &[
                ("a", "number"),
                ("b", "number"),
            ],
            example: "3 7 max",
        });
        m.insert("mod", CommandHelp {
            name: "mod",
            description: "Compute remainder of division",
            syntax: "a b mod",
            args: &[
                ("a", "dividend, number"),
                ("b", "divisor, number"),
            ],
            example: "17 5 mod",
        });
        m.insert("%", CommandHelp {
            name: "%",
            description: "Compute percentage (a * b / 100)",
            syntax: "a b %",
            args: &[
                ("a", "base value, number"),
                ("b", "percentage, number"),
            ],
            example: "200 15 %",
        });
        m.insert("%inv", CommandHelp {
            name: "%inv",
            description: "Compute inverse percentage (b * 100 / a)",
            syntax: "a b %inv",
            args: &[
                ("a", "base value, number"),
                ("b", "result value, number"),
            ],
            example: "200 30 %inv",
        });
        m.insert("fact", CommandHelp {
            name: "fact",
            description: "Factorial for integers, gamma(x+1) for reals",
            syntax: "n fact",
            args: &[("n", "non-negative integer or real")],
            example: "6 fact",
        });
        m.insert("floor", CommandHelp {
            name: "floor",
            description: "Round down to nearest integer",
            syntax: "x floor",
            args: &[("x", "number or complex")],
            example: "3.7 floor",
        });
        m.insert("ceil", CommandHelp {
            name: "ceil",
            description: "Round up to nearest integer",
            syntax: "x ceil",
            args: &[("x", "number or complex")],
            example: "3.2 ceil",
        });
        m.insert("round", CommandHelp {
            name: "round",
            description: "Round to nearest integer (half away from zero)",
            syntax: "x round",
            args: &[("x", "number or complex")],
            example: "3.5 round",
        });
        m.insert("ip", CommandHelp {
            name: "ip",
            description: "Extract integer part (truncate toward zero)",
            syntax: "x ip",
            args: &[("x", "number")],
            example: "-3.7 ip",
        });
        m.insert("fp", CommandHelp {
            name: "fp",
            description: "Extract fractional part",
            syntax: "x fp",
            args: &[("x", "number")],
            example: "3.14159 fp",
        });
        m.insert("mant", CommandHelp {
            name: "mant",
            description: "Extract mantissa (significand) of a number",
            syntax: "x mant",
            args: &[("x", "finite number")],
            example: "123.456 mant",
        });
        m.insert("xpon", CommandHelp {
            name: "xpon",
            description: "Extract exponent (power of 10) of a number",
            syntax: "x xpon",
            args: &[("x", "finite number")],
            example: "123.456 xpon",
        });

        // Bitwise operations
        m.insert("&", CommandHelp {
            name: "&",
            description: "Bitwise AND of two integers",
            syntax: "a b &",
            args: &[
                ("a", "integer"),
                ("b", "integer"),
            ],
            example: "0xff 0x0f &",
        });
        m.insert("|", CommandHelp {
            name: "|",
            description: "Bitwise OR of two integers",
            syntax: "a b |",
            args: &[
                ("a", "integer"),
                ("b", "integer"),
            ],
            example: "0b1100 0b0011 |",
        });
        m.insert("~", CommandHelp {
            name: "~",
            description: "Bitwise NOT (complement within significant bits)",
            syntax: "x ~",
            args: &[("x", "integer")],
            example: "0b1010 ~",
        });

        // Stack operations
        m.insert("swap", CommandHelp {
            name: "swap",
            description: "Exchange the top two stack items",
            syntax: "a b swap",
            args: &[
                ("a", "any object"),
                ("b", "any object"),
            ],
            example: "1 2 swap",
        });
        m.insert("dup", CommandHelp {
            name: "dup",
            description: "Duplicate the top stack item",
            syntax: "x dup",
            args: &[("x", "any object")],
            example: "42 dup",
        });
        m.insert("dup2", CommandHelp {
            name: "dup2",
            description: "Duplicate the top two stack items",
            syntax: "a b dup2",
            args: &[
                ("a", "any object"),
                ("b", "any object"),
            ],
            example: "1 2 dup2",
        });
        m.insert("dupn", CommandHelp {
            name: "dupn",
            description: "Duplicate the top n stack items",
            syntax: "x1 ... xn n dupn",
            args: &[("n", "positive integer, number of items to duplicate")],
            example: "1 2 3 3 dupn",
        });
        m.insert("drop", CommandHelp {
            name: "drop",
            description: "Remove the top stack item",
            syntax: "x drop",
            args: &[("x", "any object")],
            example: "1 2 drop",
        });
        m.insert("pop", CommandHelp {
            name: "pop",
            description: "Remove the top stack item (alias for drop)",
            syntax: "x pop",
            args: &[("x", "any object")],
            example: "1 2 pop",
        });
        m.insert("drop2", CommandHelp {
            name: "drop2",
            description: "Remove the top two stack items",
            syntax: "a b drop2",
            args: &[
                ("a", "any object"),
                ("b", "any object"),
            ],
            example: "1 2 3 drop2",
        });
        m.insert("dropn", CommandHelp {
            name: "dropn",
            description: "Remove the top n stack items",
            syntax: "x1 ... xn n dropn",
            args: &[("n", "positive integer, number of items to remove")],
            example: "1 2 3 4 3 dropn",
        });
        m.insert("del", CommandHelp {
            name: "del",
            description: "Clear all items from the stack",
            syntax: "del",
            args: &[],
            example: "1 2 3 del",
        });
        m.insert("erase", CommandHelp {
            name: "erase",
            description: "Clear all items from the stack (alias for del)",
            syntax: "erase",
            args: &[],
            example: "1 2 3 erase",
        });
        m.insert("clear", CommandHelp {
            name: "clear",
            description: "Clear all items from the stack (alias for del)",
            syntax: "clear",
            args: &[],
            example: "1 2 3 clear",
        });
        m.insert("pick", CommandHelp {
            name: "pick",
            description: "Copy item at level n to top of stack",
            syntax: "n pick",
            args: &[("n", "positive integer, stack level (1 = top)")],
            example: "10 20 30 2 pick",
        });
        m.insert("depth", CommandHelp {
            name: "depth",
            description: "Push the current stack depth",
            syntax: "depth",
            args: &[],
            example: "1 2 3 depth",
        });
        m.insert("rot", CommandHelp {
            name: "rot",
            description: "Rotate top 3 items: level 3 moves to top",
            syntax: "a b c rot",
            args: &[
                ("a", "any object (level 3)"),
                ("b", "any object (level 2)"),
                ("c", "any object (level 1)"),
            ],
            example: "1 2 3 rot",
        });
        m.insert("roll", CommandHelp {
            name: "roll",
            description: "Move item at level n to top of stack",
            syntax: "n roll",
            args: &[("n", "positive integer, stack level")],
            example: "10 20 30 40 3 roll",
        });
        m.insert("rolld", CommandHelp {
            name: "rolld",
            description: "Move top item to level n",
            syntax: "n rolld",
            args: &[("n", "positive integer, target stack level")],
            example: "10 20 30 40 3 rolld",
        });
        m.insert("over", CommandHelp {
            name: "over",
            description: "Copy item at level 2 to top of stack",
            syntax: "a b over",
            args: &[
                ("a", "any object (level 2)"),
                ("b", "any object (level 1)"),
            ],
            example: "1 2 over",
        });

        // Comparison and logic
        m.insert(">", CommandHelp {
            name: ">",
            description: "Test if first is greater than second",
            syntax: "a b >",
            args: &[
                ("a", "number"),
                ("b", "number"),
            ],
            example: "5 3 >",
        });
        m.insert(">=", CommandHelp {
            name: ">=",
            description: "Test if first is greater than or equal to second",
            syntax: "a b >=",
            args: &[
                ("a", "number"),
                ("b", "number"),
            ],
            example: "5 5 >=",
        });
        m.insert("<", CommandHelp {
            name: "<",
            description: "Test if first is less than second",
            syntax: "a b <",
            args: &[
                ("a", "number"),
                ("b", "number"),
            ],
            example: "3 5 <",
        });
        m.insert("<=", CommandHelp {
            name: "<=",
            description: "Test if first is less than or equal to second",
            syntax: "a b <=",
            args: &[
                ("a", "number"),
                ("b", "number"),
            ],
            example: "5 5 <=",
        });
        m.insert("==", CommandHelp {
            name: "==",
            description: "Test if two values are equal",
            syntax: "a b ==",
            args: &[
                ("a", "number, complex, or symbol"),
                ("b", "same type as a"),
            ],
            example: "42 42 ==",
        });
        m.insert("!=", CommandHelp {
            name: "!=",
            description: "Test if two values are not equal",
            syntax: "a b !=",
            args: &[
                ("a", "number, complex, or symbol"),
                ("b", "same type as a"),
            ],
            example: "1 2 !=",
        });
        m.insert("same", CommandHelp {
            name: "same",
            description: "Test if two values are equal (alias for ==)",
            syntax: "a b same",
            args: &[
                ("a", "number, complex, or symbol"),
                ("b", "same type as a"),
            ],
            example: "'hello' 'hello' same",
        });
        m.insert("and", CommandHelp {
            name: "and",
            description: "Logical AND (true if both non-zero)",
            syntax: "a b and",
            args: &[
                ("a", "number (0 = false, non-zero = true)"),
                ("b", "number (0 = false, non-zero = true)"),
            ],
            example: "1 1 and",
        });
        m.insert("or", CommandHelp {
            name: "or",
            description: "Logical OR (true if either non-zero)",
            syntax: "a b or",
            args: &[
                ("a", "number (0 = false, non-zero = true)"),
                ("b", "number (0 = false, non-zero = true)"),
            ],
            example: "0 1 or",
        });
        m.insert("xor", CommandHelp {
            name: "xor",
            description: "Logical XOR (true if exactly one is true)",
            syntax: "a b xor",
            args: &[
                ("a", "number (0 = false, non-zero = true)"),
                ("b", "number (0 = false, non-zero = true)"),
            ],
            example: "1 0 xor",
        });
        m.insert("not", CommandHelp {
            name: "not",
            description: "Logical NOT (true becomes false, false becomes true)",
            syntax: "x not",
            args: &[("x", "number (0 = false, non-zero = true)")],
            example: "0 not",
        });

        // Complex numbers
        m.insert("re", CommandHelp {
            name: "re",
            description: "Extract real part of a complex number",
            syntax: "z re",
            args: &[("z", "complex number")],
            example: "(3,4) re",
        });
        m.insert("im", CommandHelp {
            name: "im",
            description: "Extract imaginary part of a complex number",
            syntax: "z im",
            args: &[("z", "complex number")],
            example: "(3,4) im",
        });
        m.insert("arg", CommandHelp {
            name: "arg",
            description: "Compute argument (angle) of a complex number in radians",
            syntax: "z arg",
            args: &[("z", "complex number")],
            example: "(1,1) arg",
        });
        m.insert("conj", CommandHelp {
            name: "conj",
            description: "Compute complex conjugate",
            syntax: "z conj",
            args: &[("z", "complex number")],
            example: "(3,4) conj",
        });
        m.insert("c->r", CommandHelp {
            name: "c->r",
            description: "Split complex into real and imaginary parts",
            syntax: "z c->r",
            args: &[("z", "complex number")],
            example: "(3,4) c->r",
        });
        m.insert("r->c", CommandHelp {
            name: "r->c",
            description: "Combine two reals into a complex number",
            syntax: "re im r->c",
            args: &[
                ("re", "real part, number"),
                ("im", "imaginary part, number"),
            ],
            example: "3 4 r->c",
        });
        m.insert("p->r", CommandHelp {
            name: "p->r",
            description: "Convert polar to rectangular coordinates",
            syntax: "z p->r",
            args: &[("z", "complex in polar form (magnitude, angle)")],
            example: "(1,0.785398) p->r",
        });
        m.insert("r->p", CommandHelp {
            name: "r->p",
            description: "Convert rectangular to polar coordinates",
            syntax: "z r->p",
            args: &[("z", "complex in rectangular form")],
            example: "(1,1) r->p",
        });

        // Trigonometry
        m.insert("sin", CommandHelp {
            name: "sin",
            description: "Compute sine (argument in radians)",
            syntax: "x sin",
            args: &[("x", "number or complex, angle in radians")],
            example: "pi 6 / sin",
        });
        m.insert("cos", CommandHelp {
            name: "cos",
            description: "Compute cosine (argument in radians)",
            syntax: "x cos",
            args: &[("x", "number or complex, angle in radians")],
            example: "pi 3 / cos",
        });
        m.insert("tan", CommandHelp {
            name: "tan",
            description: "Compute tangent (argument in radians)",
            syntax: "x tan",
            args: &[("x", "number or complex, angle in radians")],
            example: "pi 4 / tan",
        });
        m.insert("asin", CommandHelp {
            name: "asin",
            description: "Compute arc sine (result in radians)",
            syntax: "x asin",
            args: &[("x", "number or complex, -1 to 1 for real result")],
            example: "0.5 asin",
        });
        m.insert("acos", CommandHelp {
            name: "acos",
            description: "Compute arc cosine (result in radians)",
            syntax: "x acos",
            args: &[("x", "number or complex, -1 to 1 for real result")],
            example: "0.5 acos",
        });
        m.insert("atan", CommandHelp {
            name: "atan",
            description: "Compute arc tangent (result in radians)",
            syntax: "x atan",
            args: &[("x", "number or complex")],
            example: "1 atan",
        });
        m.insert("atan2", CommandHelp {
            name: "atan2",
            description: "Two-argument arc tangent (result in radians)",
            syntax: "y x atan2",
            args: &[
                ("y", "y-coordinate, number"),
                ("x", "x-coordinate, number"),
            ],
            example: "1 1 atan2",
        });
        m.insert("sinh", CommandHelp {
            name: "sinh",
            description: "Compute hyperbolic sine",
            syntax: "x sinh",
            args: &[("x", "number or complex")],
            example: "1 sinh",
        });
        m.insert("cosh", CommandHelp {
            name: "cosh",
            description: "Compute hyperbolic cosine",
            syntax: "x cosh",
            args: &[("x", "number or complex")],
            example: "1 cosh",
        });
        m.insert("tanh", CommandHelp {
            name: "tanh",
            description: "Compute hyperbolic tangent",
            syntax: "x tanh",
            args: &[("x", "number or complex")],
            example: "1 tanh",
        });
        m.insert("asinh", CommandHelp {
            name: "asinh",
            description: "Compute inverse hyperbolic sine",
            syntax: "x asinh",
            args: &[("x", "number or complex")],
            example: "1 asinh",
        });
        m.insert("acosh", CommandHelp {
            name: "acosh",
            description: "Compute inverse hyperbolic cosine",
            syntax: "x acosh",
            args: &[("x", "number or complex, >= 1 for real result")],
            example: "2 acosh",
        });
        m.insert("atanh", CommandHelp {
            name: "atanh",
            description: "Compute inverse hyperbolic tangent",
            syntax: "x atanh",
            args: &[("x", "number or complex, -1 to 1 for real result")],
            example: "0.5 atanh",
        });
        m.insert("d->r", CommandHelp {
            name: "d->r",
            description: "Convert degrees to radians",
            syntax: "deg d->r",
            args: &[("deg", "angle in degrees, number")],
            example: "180 d->r",
        });
        m.insert("r->d", CommandHelp {
            name: "r->d",
            description: "Convert radians to degrees",
            syntax: "rad r->d",
            args: &[("rad", "angle in radians, number")],
            example: "pi r->d",
        });
        m.insert("pi", CommandHelp {
            name: "pi",
            description: "Push the constant pi",
            syntax: "pi",
            args: &[],
            example: "pi",
        });

        // Logarithms
        m.insert("ln", CommandHelp {
            name: "ln",
            description: "Compute natural logarithm (base e)",
            syntax: "x ln",
            args: &[("x", "number or complex, positive for real result")],
            example: "e ln",
        });
        m.insert("log", CommandHelp {
            name: "log",
            description: "Compute natural logarithm (alias for ln)",
            syntax: "x log",
            args: &[("x", "number or complex, positive for real result")],
            example: "10 log",
        });
        m.insert("exp", CommandHelp {
            name: "exp",
            description: "Compute exponential (e^x)",
            syntax: "x exp",
            args: &[("x", "number or complex")],
            example: "1 exp",
        });
        m.insert("log10", CommandHelp {
            name: "log10",
            description: "Compute base-10 logarithm",
            syntax: "x log10",
            args: &[("x", "number or complex, positive for real result")],
            example: "100 log10",
        });
        m.insert("alog10", CommandHelp {
            name: "alog10",
            description: "Compute 10^x (antilogarithm base 10)",
            syntax: "x alog10",
            args: &[("x", "number or complex")],
            example: "2 alog10",
        });
        m.insert("exp10", CommandHelp {
            name: "exp10",
            description: "Compute 10^x (alias for alog10)",
            syntax: "x exp10",
            args: &[("x", "number or complex")],
            example: "3 exp10",
        });
        m.insert("log2", CommandHelp {
            name: "log2",
            description: "Compute base-2 logarithm",
            syntax: "x log2",
            args: &[("x", "number or complex, positive for real result")],
            example: "8 log2",
        });
        m.insert("alog2", CommandHelp {
            name: "alog2",
            description: "Compute 2^x (antilogarithm base 2)",
            syntax: "x alog2",
            args: &[("x", "number or complex")],
            example: "10 alog2",
        });
        m.insert("exp2", CommandHelp {
            name: "exp2",
            description: "Compute 2^x (alias for alog2)",
            syntax: "x exp2",
            args: &[("x", "number or complex")],
            example: "8 exp2",
        });
        m.insert("logn", CommandHelp {
            name: "logn",
            description: "Compute logarithm with arbitrary base",
            syntax: "x n logn",
            args: &[
                ("x", "number or complex, positive for real result"),
                ("n", "base, positive number"),
            ],
            example: "81 3 logn",
        });
        m.insert("alogn", CommandHelp {
            name: "alogn",
            description: "Compute n^x (antilogarithm with arbitrary base)",
            syntax: "x n alogn",
            args: &[
                ("x", "exponent, number or complex"),
                ("n", "base, positive number"),
            ],
            example: "4 3 alogn",
        });
        m.insert("lnp1", CommandHelp {
            name: "lnp1",
            description: "Compute ln(1+x), accurate for small x",
            syntax: "x lnp1",
            args: &[("x", "number or complex, > -1 for real result")],
            example: "0.001 lnp1",
        });
        m.insert("expm", CommandHelp {
            name: "expm",
            description: "Compute exp(x)-1, accurate for small x",
            syntax: "x expm",
            args: &[("x", "number or complex")],
            example: "0.001 expm",
        });
        m.insert("e", CommandHelp {
            name: "e",
            description: "Push Euler's number (base of natural logarithm)",
            syntax: "e",
            args: &[],
            example: "e",
        });

        // Variables
        m.insert("sto", CommandHelp {
            name: "sto",
            description: "Store a value in a variable",
            syntax: "value 'name' sto",
            args: &[
                ("value", "any object to store"),
                ("name", "quoted symbol, variable name"),
            ],
            example: "42 'answer' sto",
        });
        m.insert("rcl", CommandHelp {
            name: "rcl",
            description: "Recall a value from a variable",
            syntax: "'name' rcl",
            args: &[("name", "quoted symbol, existing variable name")],
            example: "'answer' rcl",
        });
        m.insert("purge", CommandHelp {
            name: "purge",
            description: "Delete a variable",
            syntax: "'name' purge",
            args: &[("name", "quoted symbol, existing variable name")],
            example: "'answer' purge",
        });
        m.insert("sto+", CommandHelp {
            name: "sto+",
            description: "Add a value to a stored variable",
            syntax: "value 'name' sto+",
            args: &[
                ("value", "number to add"),
                ("name", "quoted symbol, existing variable name"),
            ],
            example: "10 'counter' sto+",
        });
        m.insert("sto-", CommandHelp {
            name: "sto-",
            description: "Subtract a value from a stored variable",
            syntax: "value 'name' sto-",
            args: &[
                ("value", "number to subtract"),
                ("name", "quoted symbol, existing variable name"),
            ],
            example: "5 'counter' sto-",
        });
        m.insert("sto*", CommandHelp {
            name: "sto*",
            description: "Multiply a stored variable by a value",
            syntax: "value 'name' sto*",
            args: &[
                ("value", "number to multiply by"),
                ("name", "quoted symbol, existing variable name"),
            ],
            example: "2 'value' sto*",
        });
        m.insert("sto/", CommandHelp {
            name: "sto/",
            description: "Divide a stored variable by a value",
            syntax: "value 'name' sto/",
            args: &[
                ("value", "number to divide by, non-zero"),
                ("name", "quoted symbol, existing variable name"),
            ],
            example: "2 'value' sto/",
        });
        m.insert("sneg", CommandHelp {
            name: "sneg",
            description: "Negate a stored variable",
            syntax: "'name' sneg",
            args: &[("name", "quoted symbol, existing variable name")],
            example: "'value' sneg",
        });
        m.insert("stoneg", CommandHelp {
            name: "stoneg",
            description: "Negate a stored variable (alias for sneg)",
            syntax: "'name' stoneg",
            args: &[("name", "quoted symbol, existing variable name")],
            example: "'value' stoneg",
        });
        m.insert("sinv", CommandHelp {
            name: "sinv",
            description: "Invert a stored variable (compute 1/x)",
            syntax: "'name' sinv",
            args: &[("name", "quoted symbol, existing variable name with non-zero value")],
            example: "'value' sinv",
        });
        m.insert("stoinv", CommandHelp {
            name: "stoinv",
            description: "Invert a stored variable (alias for sinv)",
            syntax: "'name' stoinv",
            args: &[("name", "quoted symbol, existing variable name with non-zero value")],
            example: "'value' stoinv",
        });
        m.insert("vars", CommandHelp {
            name: "vars",
            description: "List all defined variables",
            syntax: "vars",
            args: &[],
            example: "vars",
        });
        m.insert("clusr", CommandHelp {
            name: "clusr",
            description: "Clear all user-defined variables",
            syntax: "clusr",
            args: &[],
            example: "clusr",
        });

        // Control flow
        m.insert("if", CommandHelp {
            name: "if",
            description: "Start conditional block (condition between if and then)",
            syntax: "if <condition> then <true-branch> [else <false-branch>] end",
            args: &[],
            example: "if 1 2 < then 'less' end",
        });
        m.insert("then", CommandHelp {
            name: "then",
            description: "Evaluate condition and begin true branch",
            syntax: "if <condition> then <true-branch> end",
            args: &[],
            example: "if x 0 > then x sqrt end",
        });
        m.insert("else", CommandHelp {
            name: "else",
            description: "Begin false branch of conditional",
            syntax: "if <cond> then <true> else <false> end",
            args: &[],
            example: "if x 0 >= then x sqrt else 'negative' end",
        });
        m.insert("end", CommandHelp {
            name: "end",
            description: "End a control structure (if, while, do)",
            syntax: "... end",
            args: &[],
            example: "if 1 then 'yes' end",
        });
        m.insert("ift", CommandHelp {
            name: "ift",
            description: "Inline if-then: execute object if condition is true",
            syntax: "condition object ift",
            args: &[
                ("condition", "number, 0 = false, non-zero = true"),
                ("object", "program or value to execute/push if true"),
            ],
            example: "1 << 'yes' >> ift",
        });
        m.insert("ifte", CommandHelp {
            name: "ifte",
            description: "Inline if-then-else: execute one of two objects",
            syntax: "condition true-obj false-obj ifte",
            args: &[
                ("condition", "number, 0 = false, non-zero = true"),
                ("true-obj", "program or value if true"),
                ("false-obj", "program or value if false"),
            ],
            example: "0 'yes' 'no' ifte",
        });
        m.insert("for", CommandHelp {
            name: "for",
            description: "Start counted loop with loop variable",
            syntax: "start end for var <body> next|step",
            args: &[
                ("start", "starting value, number"),
                ("end", "ending value, number"),
                ("var", "unquoted symbol, loop variable name"),
            ],
            example: "1 10 for i i next",
        });
        m.insert("start", CommandHelp {
            name: "start",
            description: "Start counted loop without loop variable",
            syntax: "start end start <body> next|step",
            args: &[
                ("start", "starting value, number"),
                ("end", "ending value, number"),
            ],
            example: "1 5 start 'hello' next",
        });
        m.insert("next", CommandHelp {
            name: "next",
            description: "End loop with increment of 1",
            syntax: "for/start ... next",
            args: &[],
            example: "1 3 for i i next",
        });
        m.insert("step", CommandHelp {
            name: "step",
            description: "End loop with custom step value",
            syntax: "for/start ... <step-value> step",
            args: &[("step-value", "increment value, number")],
            example: "0 10 for i i 2 step",
        });
        m.insert("while", CommandHelp {
            name: "while",
            description: "Start while loop (test before each iteration)",
            syntax: "while <condition> repeat <body> end",
            args: &[],
            example: "while dup 10 < repeat 1+ end",
        });
        m.insert("repeat", CommandHelp {
            name: "repeat",
            description: "Begin body of while loop",
            syntax: "while <condition> repeat <body> end",
            args: &[],
            example: "1 while dup 5 <= repeat dup 1+ end",
        });
        m.insert("do", CommandHelp {
            name: "do",
            description: "Start do-until loop (test after each iteration)",
            syntax: "do <body> until <condition> end",
            args: &[],
            example: "1 do dup 1+ until dup 10 > end",
        });
        m.insert("until", CommandHelp {
            name: "until",
            description: "Test condition for do loop exit",
            syntax: "do <body> until <condition> end",
            args: &[],
            example: "do 1+ until dup 10 >= end",
        });

        // Programs
        m.insert("eval", CommandHelp {
            name: "eval",
            description: "Execute a program or recall a variable",
            syntax: "object eval",
            args: &[("object", "program to execute or symbol to recall")],
            example: "<< 2 3 + >> eval",
        });
        m.insert("->", CommandHelp {
            name: "->",
            description: "Define local variables in a program",
            syntax: "<< -> var1 var2 ... << body >> >>",
            args: &[("var1 var2 ...", "unquoted symbols, local variable names")],
            example: "<< -> x y << x y + >> >>",
        });

        // Configuration
        m.insert("std", CommandHelp {
            name: "std",
            description: "Set standard display mode with n significant digits",
            syntax: "n std",
            args: &[("n", "number of significant digits, positive integer")],
            example: "10 std",
        });
        m.insert("fix", CommandHelp {
            name: "fix",
            description: "Set fixed-point display mode with n decimal places",
            syntax: "n fix",
            args: &[("n", "number of decimal places, non-negative integer")],
            example: "4 fix",
        });
        m.insert("sci", CommandHelp {
            name: "sci",
            description: "Set scientific notation with n significant digits",
            syntax: "n sci",
            args: &[("n", "number of significant digits, positive integer")],
            example: "6 sci",
        });
        m.insert("prec", CommandHelp {
            name: "prec",
            description: "Set floating-point precision in bits",
            syntax: "n prec",
            args: &[("n", "precision in bits, 2 to 100000")],
            example: "256 prec",
        });
        m.insert("default", CommandHelp {
            name: "default",
            description: "Reset display mode and precision to defaults",
            syntax: "default",
            args: &[],
            example: "default",
        });
        m.insert("hex", CommandHelp {
            name: "hex",
            description: "Convert top number to hexadecimal representation",
            syntax: "n hex",
            args: &[("n", "integer")],
            example: "255 hex",
        });
        m.insert("dec", CommandHelp {
            name: "dec",
            description: "Convert top number to decimal representation",
            syntax: "n dec",
            args: &[("n", "number")],
            example: "0xff dec",
        });
        m.insert("bin", CommandHelp {
            name: "bin",
            description: "Convert top number to binary representation",
            syntax: "n bin",
            args: &[("n", "integer")],
            example: "15 bin",
        });
        m.insert("base", CommandHelp {
            name: "base",
            description: "Convert top number to arbitrary base representation",
            syntax: "n b base",
            args: &[
                ("n", "integer to convert"),
                ("b", "target base, 2 to 62"),
            ],
            example: "100 8 base",
        });
        m.insert("type", CommandHelp {
            name: "type",
            description: "Push the type name of the top stack item",
            syntax: "obj type",
            args: &[("obj", "any object")],
            example: "3.14 type",
        });

        // General
        m.insert("help", CommandHelp {
            name: "help",
            description: "Show general help or help for a specific command",
            syntax: "help  or  'command' help",
            args: &[("command", "optional, quoted symbol of command name")],
            example: "'sto' help",
        });
        m.insert("h", CommandHelp {
            name: "h",
            description: "Show help (alias for help)",
            syntax: "h  or  'command' h",
            args: &[("command", "optional, quoted symbol of command name")],
            example: "'sqrt' h",
        });
        m.insert("?", CommandHelp {
            name: "?",
            description: "Show help (alias for help)",
            syntax: "?  or  'command' ?",
            args: &[("command", "optional, quoted symbol of command name")],
            example: "'sin' ?",
        });
        m.insert("history", CommandHelp {
            name: "history",
            description: "Display command history",
            syntax: "history",
            args: &[],
            example: "history",
        });
        m.insert("version", CommandHelp {
            name: "version",
            description: "Push rpnx version string onto the stack",
            syntax: "version",
            args: &[],
            example: "version",
        });
        m.insert("uname", CommandHelp {
            name: "uname",
            description: "Push system identification string onto the stack",
            syntax: "uname",
            args: &[],
            example: "uname",
        });
        m.insert("error", CommandHelp {
            name: "error",
            description: "Push the last error code onto the stack",
            syntax: "error",
            args: &[],
            example: "error",
        });
        m.insert("strerror", CommandHelp {
            name: "strerror",
            description: "Push the last error message onto the stack",
            syntax: "strerror",
            args: &[],
            example: "strerror",
        });
        m.insert("test", CommandHelp {
            name: "test",
            description: "Run a test file",
            syntax: "'filename' test",
            args: &[("filename", "quoted symbol, path to test file")],
            example: "'test/all.md' test",
        });
        m.insert("quit", CommandHelp {
            name: "quit",
            description: "Exit rpnx",
            syntax: "quit",
            args: &[],
            example: "quit",
        });
        m.insert("q", CommandHelp {
            name: "q",
            description: "Exit rpnx (alias for quit)",
            syntax: "q",
            args: &[],
            example: "q",
        });
        m.insert("exit", CommandHelp {
            name: "exit",
            description: "Exit rpnx (alias for quit)",
            syntax: "exit",
            args: &[],
            example: "exit",
        });
        m.insert("edit", CommandHelp {
            name: "edit",
            description: "Edit top stack item or a variable in external editor",
            syntax: "edit  or  'name' edit",
            args: &[("name", "optional, quoted symbol of variable name")],
            example: "'myprogram' edit",
        });
        m.insert("time", CommandHelp {
            name: "time",
            description: "Push current local time in HH:MM:SS format",
            syntax: "time",
            args: &[],
            example: "time",
        });
        m.insert("date", CommandHelp {
            name: "date",
            description: "Push current local date in YYYY-MM-DD format",
            syntax: "date",
            args: &[],
            example: "date",
        });
        m.insert("ticks", CommandHelp {
            name: "ticks",
            description: "Push current timestamp in microseconds",
            syntax: "ticks",
            args: &[],
            example: "ticks",
        });

        m
    };
}

/// Get help for a specific command
pub fn get_command_help(command: &str) -> Option<&'static CommandHelp> {
    HELP_DB.get(command)
}

/// Display help for a specific command
pub fn display_command_help(cmd: &CommandHelp) {
    const R: &str = "\x1b[0m";    // Reset
    const K: &str = "\x1b[33m";   // Keyword: yellow
    const T: &str = "\x1b[1;37m"; // Title: bold white
    const C: &str = "\x1b[36m";   // Cyan for values

    println!("\n{K}{}{R}: {}", cmd.name, cmd.description);
    println!("{T}syntax:{R} {}", cmd.syntax);

    if !cmd.args.is_empty() {
        for (arg_name, arg_desc) in cmd.args {
            println!("  {C}{}{R}: {}", arg_name, arg_desc);
        }
    }

    // Only show example if it's different from syntax
    if cmd.example != cmd.syntax {
        println!("{T}example:{R} {}", cmd.example);
    }
    println!();
}
