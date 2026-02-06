// SPDX-License-Identifier: GPL-3.0-only
// Reverse Polish Notation Calculator

use rpnx::{program, repl, Context};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Command-line mode: execute arguments as RPN expression
    if args.len() > 1 {
        let mut ctx = Context::new();
        let expression = args[1..].join(" ");

        match program::execute(&expression, &mut ctx) {
            Ok(()) => {
                // Show stack (bottom to top, index 1 at bottom)
                if ctx.stack.len() == 1 {
                    // Single item: display without prefix
                    if let Some(obj) = ctx.stack.get(0) {
                        println!("{}", obj.display(&ctx.config));
                    }
                } else {
                    // Multiple items: display from bottom (oldest) to top (newest)
                    // Index i corresponds to level (i+1), so higher levels print first
                    for i in (0..ctx.stack.len()).rev() {
                        if let Some(obj) = ctx.stack.get(i) {
                            let stack_level = i + 1;
                            println!("{}> {}", stack_level, obj.display(&ctx.config));
                        }
                    }
                }
            }
            Err(rpnx::Error::Goodbye) => {
                // Clean exit on quit/exit command
                std::process::exit(0);
            }
            Err(rpnx::Error::Edit(_)) => {
                // Edit command not supported in command-line mode
                eprintln!("Error: 'edit' command only works in interactive mode");
                std::process::exit(1);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        return;
    }

    // Interactive mode: run REPL
    if let Err(e) = repl::run_repl() {
        eprintln!("REPL error: {}", e);
        std::process::exit(1);
    }
}
