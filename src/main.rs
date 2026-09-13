mod models;
mod cli;
mod tcp;

use crate::models::utils::CommandsAction;
use crate::cli::cli::{Cli, Commands};

use std::io;
use std::io::Write;
use clap::Parser;

fn main() {
    print_welcome();

    loop {
        print!("rmap> ");
        match io::stdout().flush() {
            Ok(_) => {}
            Err(e) => {
                println!("Failed to flush stdout: {}", e);
                break;
            }
        }

        let mut input: String = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(e) => {
                println!("Failed to read input: {}", e);
                break;
            }
        }

        let input: &str = input.trim();
        if input.is_empty() {
            continue
        }

        let string_split: Vec<String> = split_string(&input);

        let args = std::iter::once("rmap".to_string())
            .chain(string_split);

        let result: Cli = match Cli::try_parse_from(args) {
            Ok(val) => val,
            Err(err) => {
                match err.kind() {
                    clap::error::ErrorKind::DisplayHelp |
                    clap::error::ErrorKind::DisplayVersion => {
                        print!("{}", err);
                    }
                    _ => {
                        eprintln!("ERROR: Error found during parsing the arguments!\n{}\n", err);
                    }
                }
                continue
            },
        };

        match &result.command {
            Commands::Clear {} => {
                print!("{esc}c", esc = 27 as char);
            }

            Commands::Quit {} => {
                break
            }

            Commands::TCP(tcp) => {
                match tcp.run() {
                    Ok(_) => {},
                    Err(err) => {
                        println!("Error: Found error during tcp scan: {}\n", err);
                        continue
                    },
                }
            }
        }
    }
}

fn print_welcome() {
    println!("===============================================");
    println!("██████╗     ███╗   ███╗     █████╗     ██████╗\n\
              ██╔══██╗    ████╗ ████║    ██╔══██╗    ██╔══██╗\n\
              ██████╔╝    ██╔████╔██║    ███████║    ██████╔╝\n\
              ██╔══██╗    ██║╚██╔╝██║    ██╔══██║    ██╔═══╝\n\
              ██║  ██║    ██║ ╚═╝ ██║    ██║  ██║    ██║\n\
              ╚═╝  ╚═╝    ╚═╝     ╚═╝    ╚═╝  ╚═╝    ╚═╝");
    println!("Enter 'quit' to quit the program!");
    println!("Enter '--help' (or '-h') for options!");
    println!("===============================================");

}

fn split_string(input: &str) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut current: String = String::new();
    let mut in_quotes: bool = false;
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '"' => in_quotes = !in_quotes,
            ' ' if !in_quotes => {
                if !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(c),
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    tokens
}
