//! The Wright interactive REPL. 

use std::io::{self, BufRead, Write};
use crate::{WRIGHT_VERSION, filemap::{FileMap, FileName}, parser::lexer::Lexer};
use derive_more::Display;

const HELP_MESSAGE: &'static str = "
Wright REPL Help:

Built-in commands:
- :?/:h/:help -- Print this help menu. 
- :m/:mode -- Print the current mode.
- :e/:eval -- Switch to eval mode.
- :t/:token -- Switch to token mode.
- :a/:ast -- Switch to AST mode. 
- :c/:clear -- Clear the terminal window. 
- :v/:version -- Print the current Wright version information. 
- :q/:quit/:exit -- Quit/Exit the REPL. 

Modes:
- eval mode: Evaluate each line of input 
- token mode: Print the tokens generated for each line of input. 
- AST mode: Print the AST tree/node generated for each line of input.
";

#[derive(Clone, Copy, PartialEq, Debug, Default, Display)]
enum ReplMode {
    
    /// Default REPL mode -- evaluates and prints results of input. 
    #[default]
    Eval,

    /// Print the tokens passed to the repl. 
    Tokens, 

    /// Print the AST Tree passed to the repl. 
    Ast,
}

/// Start an interactive Wright repl. 
pub fn start() -> anyhow::Result<()> {
    // Print version info. 
    println!("Wright REPL interpreter (wright version {})", WRIGHT_VERSION);

    // Get a global lock on the standard input.
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let stdout = io::stdout();
    let mut output = stdout.lock();

    // Track the line number of the input.
    let mut input_number = 0usize;

    // Set the repl mode.
    let mut repl_mode = ReplMode::Tokens;

    // Make a file map to track input. 
    let mut code_map = FileMap::new();

    // Loop until this returns/exits. 
    loop {
        // Increment input number.
        input_number += 1;

        // Write prompt. 
        write!(&mut output, "[{}]: >> ", input_number)?;
        output.flush()?;

        // Read line of input. 
        let mut line = String::new();
        input.read_line(&mut line)?;
        
        // Handle certain builtin REPL commands. 
        match line.trim() {
            ":?" | ":h" | ":help" => {
                writeln!(&mut output, "{}", HELP_MESSAGE)?;
                continue;
            }

            ":v" | ":version" => {
                writeln!(&mut output, "Wright programming language version {}", WRIGHT_VERSION)?;
                continue;
            }

            ":m" | ":mode" => {
                writeln!(&mut output, "{}", repl_mode)?;
                continue;
            }

            ":q" | ":exit" | ":quit" => return Ok(()),

            ":c" | ":clear" => {
                // https://stackoverflow.com/questions/34837011/how-to-clear-the-terminal-screen-in-rust-after-a-new-line-is-printed
                writeln!(&mut output, "{esc}[2J{esc}[1;1H", esc = 27 as char)?;
                continue;
            }

            ":e" | ":eval" => unimplemented!("Eval mode is not yet implemented."),

            "t" | ":token" => {
                repl_mode = ReplMode::Tokens;
                writeln!(&mut output, "switched to token mode")?;
                continue;
            }

            ":a" | ":ast" => {
                repl_mode = ReplMode::Ast;
                writeln!(&mut output, "switched to AST mode")?;
                continue;
            }

            // Any other input is a no-op here and will get handled later. 
            _ => {}
        }

        // If the line was actual input and not a command -- Print the output prefix
        write!(&mut output, "[{}]: << ", input_number)?;
        output.flush()?;

        // Add line to the code map.
        let file_handle = code_map.add(FileName::Repl { line_number: input_number }, line);
        // Get a ref to the line we just added to the code map. 
        let line_ref: &str = code_map.get(file_handle).unwrap().source().as_str();

        match repl_mode {
            ReplMode::Ast => {
                unimplemented!("AST mode is unimplemented.");
            }

            ReplMode::Tokens => {
                // Make a new lexer and iterate through the tokens generated.
                let lexer = Lexer::new(line_ref);

                for token in lexer {
                    write!(&mut output, "[{}]", token)?;
                }

                // Write newline. 
                writeln!(&mut output, "")?;
            }

            ReplMode::Eval => unimplemented!("Eval mode is unimplemented."),
        }

    }
}
