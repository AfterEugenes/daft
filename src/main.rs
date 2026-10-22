mod commands;

use commands::Cli;
use clap::Parser;
use rustyline::{error::ReadlineError, DefaultEditor};

const TIPS: &'static str = "To display the usage type \"help\", to exit type \"exit\".";

fn main() -> rustyline::Result<()> {
    let mut editor = DefaultEditor::new()?;

    loop{
        let line = match editor.readline("> ") {
            Ok(line) => line,

            Err(ReadlineError::Interrupted) => {
                println!("{}", TIPS);
                continue;
            }

            Err(ReadlineError::Eof) => {
                break;
            }

            Err(error) => {
                eprintln!("Input error: {error}");
                break;
            }

        };
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        if line == "exit" {
            break;
        }

        if let Err(error) = editor.add_history_entry(line) {
            eprintln!("Could not save command history: {error}");
        }

        let words = match shlex::split(line) {
            Some(words) => words,
            None => {
                eprintln!("Invalid command: unmatched quote");
                continue;
            }
        };

        match Cli::try_parse_from(words) {
            Ok(cli) => cli.execute(),
            Err(error) => {
                eprintln!("{error}");
            }
        }

    }

    Ok(())
}

