// SPDX-License-Identifier: GPL-3.0-only
// Test framework for parsing and running markdown test files
// Parses test/*.md files with commands and expectations

use crate::context::Context;
use crate::error::Error;
use crate::program;
use std::fs;
use std::path::Path;

/// A step in a test case (either a command or an expectation)
#[derive(Debug, Clone)]
pub enum TestStep {
    Command(String),
    Expectation(Expectation),
}

/// A test case with interleaved commands and expectations
#[derive(Debug, Clone)]
pub struct TestCase {
    pub name: String,
    pub steps: Vec<TestStep>,
    pub source_file: Option<String>,
}

/// A test file with preamble and test cases
#[derive(Debug, Clone)]
pub struct TestFile {
    pub description: String,
    pub preamble: Vec<String>,
    pub test_cases: Vec<TestCase>,
}

/// An expectation to verify after executing commands
#[derive(Debug, Clone, PartialEq)]
pub enum Expectation {
    StackSize(usize),
    StackValues(Vec<String>),
    ErrorCode(u32),
}

/// Result of running a test
#[derive(Debug)]
pub struct TestResult {
    pub name: String,
    pub passed: bool,
    pub message: String,
    pub steps_passed: usize,
    pub steps_failed: usize,
}

/// Parse a markdown test file
pub fn parse_test_file(path: &Path) -> Result<TestFile, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;

    let source_file = path.file_name().map(|s| s.to_string_lossy().to_string());
    parse_test_content(&content, path.parent().unwrap(), source_file)
}

/// Parse test content from a string
fn parse_test_content(content: &str, base_dir: &Path, source_file: Option<String>) -> Result<TestFile, String> {
    let mut description = String::new();
    let mut preamble_commands = Vec::new();
    let mut test_cases = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;

    // First pass: collect description, preamble commands and @include directives (before first ## header)
    while i < lines.len() {
        let line = lines[i].trim();

        // Capture first # heading as description
        if description.is_empty() && line.starts_with("# ") && !line.starts_with("## ") {
            description = line.strip_prefix("# ").unwrap().to_string();
            i += 1;
            continue;
        }

        // Stop at first test case header
        if line.starts_with("## ") {
            break;
        }

        // Handle @include directives in preamble
        if line.starts_with("@include ") {
            let include_file = line.strip_prefix("@include ").unwrap().trim();
            let include_path = base_dir.join(include_file);
            let included_file = parse_test_file(&include_path)?;
            // Inject included file's preamble at the start of its first test case
            let mut included_tests = included_file.test_cases;
            if !included_file.preamble.is_empty() && !included_tests.is_empty() {
                let preamble_steps: Vec<TestStep> = included_file
                    .preamble
                    .into_iter()
                    .map(TestStep::Command)
                    .collect();
                let first_test = &mut included_tests[0];
                first_test.steps.splice(0..0, preamble_steps);
            }
            test_cases.extend(included_tests);
            i += 1;
            continue;
        }

        // Collect preamble commands
        if line.starts_with('`') && line.ends_with('`') && line.len() > 2 {
            let cmd = line.trim_start_matches('`').trim_end_matches('`');
            preamble_commands.push(cmd.to_string());
        }

        i += 1;
    }

    // Second pass: parse test cases
    while i < lines.len() {
        let line = lines[i].trim();

        // Handle @include directives in test section
        if line.starts_with("@include ") {
            let include_file = line.strip_prefix("@include ").unwrap().trim();
            let include_path = base_dir.join(include_file);
            let included_file = parse_test_file(&include_path)?;
            // Inject included file's preamble at the start of its first test case
            let mut included_tests = included_file.test_cases;
            if !included_file.preamble.is_empty() && !included_tests.is_empty() {
                let preamble_steps: Vec<TestStep> = included_file
                    .preamble
                    .into_iter()
                    .map(TestStep::Command)
                    .collect();
                let first_test = &mut included_tests[0];
                first_test.steps.splice(0..0, preamble_steps);
            }
            test_cases.extend(included_tests);
            i += 1;
            continue;
        }

        // Look for test case headers (## heading)
        if line.starts_with("## ") {
            let test_name = line.strip_prefix("## ").unwrap().to_string();
            let (steps, next_i) = parse_test_case(&lines, i + 1)?;

            if !steps.is_empty() {
                test_cases.push(TestCase {
                    name: test_name,
                    steps,
                    source_file: source_file.clone(),
                });
            }

            i = next_i;
            continue;
        }

        i += 1;
    }

    Ok(TestFile {
        description,
        preamble: preamble_commands,
        test_cases,
    })
}

/// Parse a single test case (interleaved commands and expectations)
fn parse_test_case(lines: &[&str], start: usize) -> Result<(Vec<TestStep>, usize), String> {
    let mut steps = Vec::new();
    let mut i = start;

    while i < lines.len() {
        let line = lines[i].trim();

        // Stop at next test case header or @include
        if line.starts_with("## ") || line.starts_with("@include ") {
            break;
        }

        // Stop at top-level heading
        if line.starts_with("# ") && !line.starts_with("## ") {
            break;
        }

        // Handle code blocks (triple backticks) - CHECK THIS FIRST!
        if line.starts_with("```") {
            let mut block_lines = Vec::new();
            i += 1;
            while i < lines.len() {
                // Check for end marker (trimmed to allow indentation)
                if lines[i].trim().starts_with("```") {
                    break;
                }
                // Preserve original line content (don't trim!)
                block_lines.push(lines[i]);
                i += 1;
            }
            if !block_lines.is_empty() {
                let cmd = block_lines.join("\n");
                steps.push(TestStep::Command(cmd));
            }
            i += 1;
            continue;
        }

        // Handle inline commands in backticks (`...`)
        if line.starts_with('`') && line.ends_with('`') && line.len() > 2 {
            let cmd = line.trim_start_matches('`').trim_end_matches('`');
            steps.push(TestStep::Command(cmd.to_string()));
            i += 1;
            continue;
        }

        // Handle expectations (-> ...)
        if line.starts_with("-> ") {
            let expectation_str = line.strip_prefix("-> ").unwrap();
            if let Some(exp) = parse_expectation(expectation_str) {
                steps.push(TestStep::Expectation(exp));
            }
            i += 1;
            continue;
        }

        i += 1;
    }

    Ok((steps, i))
}

/// Split a string by commas, but respect parentheses and quotes
fn split_respecting_parens(s: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    let mut paren_depth = 0;
    let mut in_single_quote = false;
    let mut in_double_quote = false;

    for ch in s.chars() {
        match ch {
            '\'' if !in_double_quote => {
                in_single_quote = !in_single_quote;
                current.push(ch);
            }
            '"' if !in_single_quote => {
                in_double_quote = !in_double_quote;
                current.push(ch);
            }
            '(' if !in_single_quote && !in_double_quote => {
                paren_depth += 1;
                current.push(ch);
            }
            ')' if !in_single_quote && !in_double_quote => {
                paren_depth -= 1;
                current.push(ch);
            }
            ',' if paren_depth == 0 && !in_single_quote && !in_double_quote => {
                // Split here
                if !current.is_empty() {
                    result.push(current.trim().to_string());
                    current.clear();
                }
            }
            _ => {
                current.push(ch);
            }
        }
    }

    // Don't forget the last value
    if !current.is_empty() {
        result.push(current.trim().to_string());
    }

    result
}

/// Parse an expectation line
fn parse_expectation(line: &str) -> Option<Expectation> {
    // Stack size: "stack size should be N"
    if line.starts_with("stack size should be ") {
        let size_str = line.strip_prefix("stack size should be ")?;
        let size = size_str.trim().parse::<usize>().ok()?;
        return Some(Expectation::StackSize(size));
    }

    // Stack values: "stack should be value1, value2, value3" or "stack should be "
    if line.starts_with("stack should be ") {
        let values_str = line.strip_prefix("stack should be ")?;
        if values_str.trim().is_empty() {
            return Some(Expectation::StackValues(vec![]));
        }

        // Split by commas, but not commas inside parentheses (for complex numbers)
        let values: Vec<String> = split_respecting_parens(values_str)
            .into_iter()
            .map(|s| s.trim().to_string())
            .collect();
        return Some(Expectation::StackValues(values));
    }

    // Error code: "error should be N"
    if line.starts_with("error should be ") {
        let error_str = line.strip_prefix("error should be ")?;
        let code = error_str.trim().parse::<u32>().ok()?;
        return Some(Expectation::ErrorCode(code));
    }

    None
}

/// Run a test case
pub fn run_test_case(test: &TestCase, ctx: &mut Context) -> TestResult {
    let mut last_error: Option<Error> = None;
    let mut first_failure: Option<String> = None;
    let mut steps_passed = 0;
    let mut steps_failed = 0;

    // Process steps sequentially (interleaved commands and expectations)
    for step in &test.steps {
        match step {
            TestStep::Command(cmd) => {
                // Execute command
                match program::execute(cmd, ctx) {
                    Ok(()) => {
                        // Respect preserve_last_error flag (set by error/strerror commands)
                        if ctx.preserve_last_error {
                            ctx.preserve_last_error = false;
                        } else {
                            last_error = None;
                            ctx.last_error = None;
                        }
                    }
                    Err(e) => {
                        last_error = Some(e.clone());
                        ctx.last_error = Some(e);
                    }
                }
            }

            TestStep::Expectation(expectation) => {
                // Verify expectation (but continue executing even if it fails)
                let failure_msg = match expectation {
                    Expectation::StackSize(expected) => {
                        let actual = ctx.stack.len();
                        if actual != *expected {
                            Some(format!(
                                "Stack size mismatch: expected {}, got {}",
                                expected, actual
                            ))
                        } else {
                            None
                        }
                    }

                    Expectation::StackValues(expected) => {
                        // Special case: single value checks only stack level 1 (top of stack)
                        // Empty or multiple values check the entire stack
                        if expected.len() == 1 {
                            // Single value: check only stack level 1 (top of stack)
                            if ctx.stack.is_empty() {
                                Some(format!("Stack is empty, expected value: '{}'", expected[0]))
                            } else {
                                // Stack level 1 is at index 0 (top of stack)
                                let actual_value = ctx.stack.get(0).unwrap().display(&ctx.config);

                                if !values_match(&expected[0], &actual_value) {
                                    Some(format!(
                                        "Stack level 1 mismatch: expected '{}', got '{}'",
                                        expected[0], actual_value
                                    ))
                                } else {
                                    None
                                }
                            }
                        } else {
                            // Empty or multiple values: check entire stack
                            // Stack is stored top-to-bottom, but expectations are bottom-to-top
                            // So reverse the actual stack values for comparison
                            let mut actual: Vec<String> = ctx
                                .stack
                                .iter()
                                .map(|obj| obj.display(&ctx.config))
                                .collect();
                            actual.reverse();

                            if actual.len() != expected.len() {
                                Some(format!(
                                    "Stack size mismatch: expected {} values, got {}",
                                    expected.len(),
                                    actual.len()
                                ))
                            } else {
                                // Check each value
                                let mut mismatch = None;
                                for (i, (exp_val, act_val)) in
                                    expected.iter().zip(actual.iter()).enumerate()
                                {
                                    if !values_match(exp_val, act_val) {
                                        mismatch = Some(format!(
                                                "Stack value mismatch at position {}: expected '{}', got '{}'",
                                                i + 1,
                                                exp_val,
                                                act_val
                                            ));
                                        break;
                                    }
                                }
                                mismatch
                            }
                        }
                    }

                    Expectation::ErrorCode(expected_code) => {
                        let actual_code = match &last_error {
                            None => 0,
                            Some(e) => error_to_code(e),
                        };

                        if actual_code != *expected_code {
                            Some(format!(
                                "Error code mismatch: expected {}, got {} ({:?})",
                                expected_code, actual_code, last_error
                            ))
                        } else {
                            None
                        }
                    }
                };

                // Track step result
                if let Some(msg) = failure_msg {
                    steps_failed += 1;
                    if first_failure.is_none() {
                        first_failure = Some(msg);
                    }
                } else {
                    steps_passed += 1;
                }
            }
        }
    }

    // Return result based on whether we had any failures
    if let Some(msg) = first_failure {
        TestResult {
            name: test.name.clone(),
            passed: false,
            message: msg,
            steps_passed,
            steps_failed,
        }
    } else {
        TestResult {
            name: test.name.clone(),
            passed: true,
            message: "PASSED".to_string(),
            steps_passed,
            steps_failed,
        }
    }
}

/// Check if two values match (exact string comparison)
fn values_match(expected: &str, actual: &str) -> bool {
    // Exact string match required
    expected == actual
}

/// Map Error enum to error code
/// Uses the error_code() method defined on Error enum
fn error_to_code(error: &Error) -> u32 {
    error.error_code() as u32
}

/// Run all tests in a file
pub fn run_test_file(path: &Path) -> Result<(TestFile, Vec<TestResult>), String> {
    let test_file = parse_test_file(path)?;
    let mut results = Vec::new();
    let mut ctx = Context::new();

    // Execute preamble commands once
    for cmd in &test_file.preamble {
        let _ = program::execute(cmd, &mut ctx);
    }

    // Run each test case sequentially with the same context
    for test in &test_file.test_cases {
        let result = run_test_case(test, &mut ctx);
        results.push(result);
    }

    Ok((test_file, results))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_expectation_stack_size() {
        let exp = parse_expectation("stack size should be 5");
        assert_eq!(exp, Some(Expectation::StackSize(5)));
    }

    #[test]
    fn test_parse_expectation_stack_values() {
        let exp = parse_expectation("stack should be 1, 2, 3");
        assert_eq!(
            exp,
            Some(Expectation::StackValues(vec![
                "1".to_string(),
                "2".to_string(),
                "3".to_string()
            ]))
        );
    }

    #[test]
    fn test_parse_expectation_empty_stack() {
        let exp = parse_expectation("stack should be ");
        assert_eq!(exp, Some(Expectation::StackValues(vec![])));
    }

    #[test]
    fn test_parse_expectation_error() {
        let exp = parse_expectation("error should be 2");
        assert_eq!(exp, Some(Expectation::ErrorCode(2)));
    }
}
