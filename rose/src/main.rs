use std::io::{self, Read, Write};

mod token;
mod lexer;
mod ast;

use token::*;
use lexer::*;

fn main() -> io::Result<()> {
    println!("Hello! Welcome to the Rose programming language!");
    'outer: loop {
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
                let mut lexer: RoseLexer = RoseLexer::new(&input, "REPL".to_string());
                'inner: loop {
                    match lexer.next_token() {
                        Some(token) => {
                            if token.ttype == TokenType::META_EOF {
                                break 'inner;
                            }
                            println!("{:?}", token);
                        },
                        None => (),
                    }
                }
            },
            Err(error) => return Err(error),
        }
    }
}

