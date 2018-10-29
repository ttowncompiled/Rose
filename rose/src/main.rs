use std::io::{self, Read, Write};

mod token;
mod lexer;
mod ast;
mod parser;

use parser::*;

fn main() -> io::Result<()> {
    println!("Hello! Welcome to the Rose programming language!");
    loop {
        print!(">> ");
        match io::stdout().flush() {
            Ok(_) => (),
            Err(error) => return Err(error),
        }
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input == "exit()\n" || input == "exit()\r" {
                    return Ok(());
                }
                let mut parser: RoseParser = RoseParser::new(&input, "REPL".to_string());
                match parser.parse_program() {
                    Some(program) => println!("{}", program.to_string()),
                    None => (),
                }
            },
            Err(error) => return Err(error),
        }
    }
}

