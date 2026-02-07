// SPDX-License-Identifier: GPL-3.0-only
// REPL (Read-Eval-Print Loop) for interactive mode
// Uses rustyline for line editing and history

use crate::context::Context;
use crate::operations::{BRANCHES, KEYWORDS};
use crate::program;
use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context as RustyContext, Editor, Helper, Result as RustylineResult};
use std::borrow::Cow;
use std::path::PathBuf;

// ANSI color codes
const COLOR_RESET: &str = "\x1b[0m";
const COLOR_NUMBER: &str = "\x1b[36m"; // Cyan for numbers
const COLOR_KEYWORD: &str = "\x1b[33m"; // Yellow for keywords
const COLOR_SYMBOL: &str = "\x1b[32m"; // Green for symbols
const COLOR_PROGRAM: &str = "\x1b[34m"; // Blue for programs
const COLOR_OPERATOR: &str = "\x1b[1m"; // Bold default for operators

/// Custom completer for RPN keywords
struct RpnCompleter;

impl RpnCompleter {
    fn new() -> Self {
        Self
    }

    fn get_candidates(&self, line: &str) -> Vec<String> {
        let mut candidates = Vec::new();

        // Get the last word being typed
        let last_word = line.split_whitespace().last().unwrap_or("");

        if last_word.is_empty() {
            return candidates;
        }

        // Add matching keywords
        for keyword in KEYWORDS.keys() {
            if keyword.starts_with(last_word) {
                candidates.push(keyword.to_string());
            }
        }

        // Add matching branch keywords
        for branch in BRANCHES.keys() {
            if branch.starts_with(last_word) {
                candidates.push(branch.to_string());
            }
        }

        candidates.sort();
        candidates.dedup();
        candidates
    }
}

impl Completer for RpnCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &RustyContext<'_>,
    ) -> RustylineResult<(usize, Vec<Pair>)> {
        let candidates = self.get_candidates(line);

        // Find start of current word
        let start = line[..pos]
            .rfind(|c: char| c.is_whitespace())
            .map(|i| i + 1)
            .unwrap_or(0);

        let pairs = candidates
            .into_iter()
            .map(|c| Pair {
                display: c.clone(),
                replacement: c,
            })
            .collect();

        Ok((start, pairs))
    }
}

impl Hinter for RpnCompleter {
    type Hint = String;
}

impl Highlighter for RpnCompleter {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        Cow::Owned(highlight_line(line))
    }

    fn highlight_char(&self, _line: &str, _pos: usize, _forced: bool) -> bool {
        // Always re-highlight when character changes
        true
    }
}

impl Validator for RpnCompleter {}

impl Helper for RpnCompleter {}

/// Classify a token and return its color
fn get_token_color(token: &str) -> &'static str {
    // Check for quoted symbol (starts with ')
    if token.starts_with('\'') {
        return COLOR_SYMBOL;
    }

    // Check for program delimiters
    if token == "<<" || token == ">>" || token == "«" || token == "»" {
        return COLOR_PROGRAM;
    }

    // Check for operators (single character math operators)
    if matches!(
        token,
        "+" | "-" | "*" | "/" | "^" | "&" | "|" | "~" | "%" | "=" | "<" | ">" | "!"
    ) {
        return COLOR_OPERATOR;
    }

    // Check for comparison operators
    if matches!(token, "==" | "!=" | "<=" | ">=" | "->") {
        return COLOR_OPERATOR;
    }

    // Check for number (starts with digit, or 0x, 0b, or negative number)
    if is_number(token) {
        return COLOR_NUMBER;
    }

    // Check for complex number (starts with '(')
    if token.starts_with('(') && token.contains(',') {
        return COLOR_NUMBER;
    }

    // Check for keyword
    if KEYWORDS.contains_key(token) || BRANCHES.contains_key(token) {
        return COLOR_KEYWORD;
    }

    // Check for common constants
    if matches!(token, "pi" | "e" | "i" | "inf" | "nan" | "true" | "false") {
        return COLOR_NUMBER;
    }

    // Default: treat as symbol (variable name)
    COLOR_SYMBOL
}

/// Check if a token looks like a number
fn is_number(token: &str) -> bool {
    if token.is_empty() {
        return false;
    }

    let token = if token.starts_with('-') || token.starts_with('+') {
        &token[1..]
    } else {
        token
    };

    if token.is_empty() {
        return false;
    }

    // Hex number
    if token.starts_with("0x") || token.starts_with("0X") {
        return token[2..].chars().all(|c| c.is_ascii_hexdigit());
    }

    // Binary number
    if token.starts_with("0b") || token.starts_with("0B") {
        return token[2..].chars().all(|c| c == '0' || c == '1');
    }

    // Base notation (e.g., 16#FF)
    if let Some(hash_pos) = token.find('#') {
        let base_part = &token[..hash_pos];
        return base_part.chars().all(|c| c.is_ascii_digit());
    }

    // Regular decimal number (with optional decimal point and exponent)
    let mut has_dot = false;
    let mut has_exp = false;
    let mut chars = token.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '0'..='9' => continue,
            '.' if !has_dot && !has_exp => {
                has_dot = true;
            }
            'e' | 'E' if !has_exp => {
                has_exp = true;
                // Allow optional sign after exponent
                if let Some(&next) = chars.peek() {
                    if next == '+' || next == '-' {
                        chars.next();
                    }
                }
            }
            _ => return false,
        }
    }

    true
}

/// Highlight a line of input with ANSI colors
fn highlight_line(line: &str) -> String {
    let mut result = String::with_capacity(line.len() * 2);
    let mut chars = line.chars().peekable();
    let mut in_program = 0; // Nesting level for programs
    let mut current_token = String::new();

    while let Some(c) = chars.next() {
        // Handle program delimiters
        if c == '<' && chars.peek() == Some(&'<') {
            // Flush current token
            if !current_token.is_empty() {
                let color = get_token_color(&current_token);
                result.push_str(color);
                result.push_str(&current_token);
                result.push_str(COLOR_RESET);
                current_token.clear();
            }
            chars.next(); // consume second <
            result.push_str(COLOR_PROGRAM);
            result.push_str("<<");
            result.push_str(COLOR_RESET);
            in_program += 1;
            continue;
        }

        if c == '>' && chars.peek() == Some(&'>') {
            // Flush current token
            if !current_token.is_empty() {
                let color = get_token_color(&current_token);
                result.push_str(color);
                result.push_str(&current_token);
                result.push_str(COLOR_RESET);
                current_token.clear();
            }
            chars.next(); // consume second >
            result.push_str(COLOR_PROGRAM);
            result.push_str(">>");
            result.push_str(COLOR_RESET);
            if in_program > 0 {
                in_program -= 1;
            }
            continue;
        }

        // Handle unicode program delimiters
        if c == '«' {
            if !current_token.is_empty() {
                let color = get_token_color(&current_token);
                result.push_str(color);
                result.push_str(&current_token);
                result.push_str(COLOR_RESET);
                current_token.clear();
            }
            result.push_str(COLOR_PROGRAM);
            result.push(c);
            result.push_str(COLOR_RESET);
            in_program += 1;
            continue;
        }

        if c == '»' {
            if !current_token.is_empty() {
                let color = get_token_color(&current_token);
                result.push_str(color);
                result.push_str(&current_token);
                result.push_str(COLOR_RESET);
                current_token.clear();
            }
            result.push_str(COLOR_PROGRAM);
            result.push(c);
            result.push_str(COLOR_RESET);
            if in_program > 0 {
                in_program -= 1;
            }
            continue;
        }

        // Handle whitespace - flush token and pass through
        if c.is_whitespace() {
            if !current_token.is_empty() {
                let color = get_token_color(&current_token);
                result.push_str(color);
                result.push_str(&current_token);
                result.push_str(COLOR_RESET);
                current_token.clear();
            }
            result.push(c);
            continue;
        }

        // Handle quoted symbols
        if c == '\'' {
            // Flush current token
            if !current_token.is_empty() {
                let color = get_token_color(&current_token);
                result.push_str(color);
                result.push_str(&current_token);
                result.push_str(COLOR_RESET);
                current_token.clear();
            }
            // Collect the quoted symbol
            current_token.push(c);
            while let Some(&next_c) = chars.peek() {
                if next_c == '\'' {
                    current_token.push(chars.next().unwrap());
                    break;
                } else if next_c.is_whitespace() {
                    break;
                } else {
                    current_token.push(chars.next().unwrap());
                }
            }
            result.push_str(COLOR_SYMBOL);
            result.push_str(&current_token);
            result.push_str(COLOR_RESET);
            current_token.clear();
            continue;
        }

        // Handle parentheses for complex numbers
        if c == '(' {
            if !current_token.is_empty() {
                let color = get_token_color(&current_token);
                result.push_str(color);
                result.push_str(&current_token);
                result.push_str(COLOR_RESET);
                current_token.clear();
            }
            // Collect until closing paren
            current_token.push(c);
            let mut paren_depth = 1;
            while let Some(next_c) = chars.next() {
                current_token.push(next_c);
                if next_c == '(' {
                    paren_depth += 1;
                } else if next_c == ')' {
                    paren_depth -= 1;
                    if paren_depth == 0 {
                        break;
                    }
                }
            }
            let color = get_token_color(&current_token);
            result.push_str(color);
            result.push_str(&current_token);
            result.push_str(COLOR_RESET);
            current_token.clear();
            continue;
        }

        // Accumulate token
        current_token.push(c);
    }

    // Flush any remaining token
    if !current_token.is_empty() {
        let color = get_token_color(&current_token);
        result.push_str(color);
        result.push_str(&current_token);
        result.push_str(COLOR_RESET);
    }

    result
}

/// Get the path to the history file ($XDG_DATA_HOME/rpnx/history)
pub fn history_path() -> PathBuf {
    let data_dir = std::env::var("XDG_DATA_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
            path.push(".local/share");
            path
        });
    data_dir.join("rpnx").join("history")
}

/// Run the interactive REPL
pub fn run_repl() -> RustylineResult<()> {
    let history_file = history_path();
    let helper = RpnCompleter::new();
    let mut rl = Editor::new()?;
    rl.set_helper(Some(helper));

    // Load history
    if history_file.exists() {
        let _ = rl.load_history(&history_file);
    }

    // Create context
    let mut ctx = Context::new();

    // Show welcome message
    println!(
        "\x1b[1mrpn\x1b[0m v{} - Reverse Polish Notation Calculator",
        env!("CARGO_PKG_VERSION")
    );
    println!("Type 'help' for help, 'quit' to exit");
    println!();

    // Track consecutive Ctrl+C for exit
    let mut consecutive_ctrl_c = false;

    loop {
        // Read line
        let readline = rl.readline("rpnx> ");
        match readline {
            Ok(line) => {
                // Reset Ctrl+C counter on any input
                consecutive_ctrl_c = false;

                // Handle empty lines - just show stack
                if line.trim().is_empty() {
                    show_stack(&ctx);
                    continue;
                }

                // Add to history
                let _ = rl.add_history_entry(&line);

                // Execute
                match program::execute(&line, &mut ctx) {
                    Ok(()) => {
                        // Success - clear last error unless preserve flag is set
                        if ctx.preserve_last_error {
                            ctx.preserve_last_error = false;
                        } else {
                            ctx.last_error = None;
                        }
                        show_stack(&ctx);
                    }
                    Err(crate::error::Error::Goodbye) => {
                        // Clean exit on quit/exit command
                        break;
                    }
                    Err(crate::error::Error::Edit(content)) => {
                        // Edit mode: present content for editing
                        let edit_result = rl.readline_with_initial("rpnx> ", (&content, ""));
                        match edit_result {
                            Ok(edited_line) => {
                                // Add edited line to history
                                let _ = rl.add_history_entry(&edited_line);

                                // Execute edited line
                                match program::execute(&edited_line, &mut ctx) {
                                    Ok(()) => {
                                        ctx.last_error = None;
                                        show_stack(&ctx);
                                    }
                                    Err(crate::error::Error::Goodbye) => {
                                        break;
                                    }
                                    Err(e) => {
                                        ctx.last_error = Some(e.clone());
                                        show_stack(&ctx);
                                        eprintln!("Error: {}", e);
                                    }
                                }
                            }
                            Err(ReadlineError::Interrupted) => {
                                // Ctrl+C during edit: cancel edit
                                consecutive_ctrl_c = true;
                                continue;
                            }
                            Err(ReadlineError::Eof) => {
                                // Ctrl+D during edit: exit
                                println!("quit");
                                break;
                            }
                            Err(err) => {
                                eprintln!("Error: {:?}", err);
                            }
                        }
                    }
                    Err(e) => {
                        // Save last error, show stack, then error
                        ctx.last_error = Some(e.clone());
                        show_stack(&ctx);
                        eprintln!("Error: {}", e);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                // Ctrl+C: if this is the second consecutive one, exit
                if consecutive_ctrl_c {
                    println!("\nExiting...");
                    break;
                }
                // First Ctrl+C: just clear line (rustyline does this automatically)
                consecutive_ctrl_c = true;
                continue;
            }
            Err(ReadlineError::Eof) => {
                // Ctrl+D
                println!("quit");
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }

    // Save history (create directory if needed)
    if let Some(parent) = history_file.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = rl.save_history(&history_file);

    Ok(())
}

/// Display the current stack
/// Top of stack (most recent) is at index 0, should display as ">" at bottom when single item
fn show_stack(ctx: &Context) {
    if ctx.stack.is_empty() {
        return;
    }

    if ctx.stack.len() == 1 {
        // Single item: display without level number or prefix
        if let Some(obj) = ctx.stack.get(0) {
            let display = obj.display(&ctx.config);
            println!("{}", highlight_line(&display));
        }
    } else {
        // Multiple items: iterate from bottom (oldest) to top (newest)
        // Index i corresponds to level (i+1), so higher levels print first
        for i in (0..ctx.stack.len()).rev() {
            if let Some(obj) = ctx.stack.get(i) {
                let stack_level = i + 1;
                let display = obj.display(&ctx.config);
                println!("{}> {}", stack_level, highlight_line(&display));
            }
        }
    }
}
