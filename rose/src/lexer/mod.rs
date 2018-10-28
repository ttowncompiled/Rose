use std::str::Chars;
use std::iter::Peekable;
use token::Token;
use token::TokenFactory;
use token::RoseTokenFactory;

pub trait Lexer<'a> {
    fn new(input: &'a str, file_name: String) -> Self;
    fn read_char(&mut self);
    fn peek_char(&mut self) -> char;
    fn next_token(&mut self) -> Option<Token>;
    fn skip_whitespace(&mut self);
}

pub struct RoseLexer<'a> {
    input:              &'a str,
    chars:              Peekable<Chars<'a>>,
    line_number:        i64,
    char_position:      i64,
    ch:                 char,
    token_factory:      Box<RoseTokenFactory>,
}

impl<'a> Lexer<'a> for RoseLexer<'a> {
    fn new(input: &'a str, file_name: String) -> RoseLexer<'a> {
        let mut l = RoseLexer{
            input:          input,
            chars:          input.chars().peekable(),
            line_number:    1,
            char_position:  1,
            ch:             '\0',
            token_factory:  Box::new(RoseTokenFactory::new(file_name)),
        };
        l.read_char();
        return l;
    }

    fn read_char(&mut self) {
        if self.ch == '\0' && self.peek_char() == '\0' {
            return;
        }
        match self.chars.next() {
            Some(ch) => self.ch = ch,
            None => self.ch = '\0',
        }
        self.char_position += 1;
    }

    fn peek_char(&mut self) -> char {
        return match self.chars.peek() {
            Some(ch) => *ch,
            None => '\0',
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        let peek_ch: char = self.peek_char();
        let token: Option<Token> = self.token_factory.manufacture(self.ch, peek_ch, self.line_number, self.char_position);
        if self.ch == '\n' || self.ch == '\r' {
            self.line_number += 1;
            self.char_position = 0;
        }
        self.read_char();
        return token;
    }

    fn skip_whitespace(&mut self) {
        while RoseTokenFactory::is_whitespace(self.ch) {
            self.read_char();
        }
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = "let five: Int = 5;
let ten: Int = 10;

let add: (Int, Int) -> Int = (x: Int, y: Int) -> Int do
    return x + y;
end;

let result: Int = add(five, ten);
";

        let tests = [
            token::Token{ ttype:    token::LET,         literal:    String::from("let")         },
            token::Token{ ttype:    token::IDENT,       literal:    String::from("five")        },
            token::Token{ ttype:    token::COLON,       literal:    String::from(":")           },
            token::Token{ ttype:    token::IDENT,       literal:    String::from("Int")         },
            token::Token{ ttype:    token::ASSIGN,      literal:    String::from("=")           },
            token::Token{ ttype:    token::INT,         literal:    String::from("5")           },
            token::Token{ ttype:    token::SEMICOLON,   literal:    String::from(";")           },
            token::Token{ ttype:    token::LET,         literal:    String::from("let")         },
            token::Token{ ttype:    token::IDENT,       literal:    String::from("ten")         },
            token::Token{ ttype:    token::COLON,       literal:    String::from(":")           },
            token::Token{ ttype:    token::IDENT,       literal:    String::from("Int")         },
            token::Token{ ttype:    token::ASSIGN,      literal:    String::from("=")           },
            token::Token{ ttype:    token::INT,         literal:    String::from("10")          },
            token::Token{ ttype:    token::SEMICOLON,   literal:    String::from(";")           },
            token::Token{ ttype:    token::LET,         literal:    String::from("let")         },
            token::Token{ ttype:    token::IDENT,       literal:    String::from("add")         },
            token::Token{ ttype:    token::COLON,       literal:    String::from(":")           },
            token::Token{ ttype:    token::LPAREN,      literal:    String::from("(")           },
            token::Token{ ttype:    token::IDENT,       literal:    String::from("Int")         },
            token::Token{ ttype:    token::COMMA,       literal:    String::from(",")           },
            token::Token{ ttype:    token::IDENT,       literal:    String::from("Int")         },
            token::Token{ ttype:    token::RPAREN,      literal:    String::from(")")           },
            token::Token{ ttype:    token::MORPH,       literal:    String::from("->")          },
            token::Token{ ttype:    token::IDENT,       literal:    String::from("Int")         },
            token::Token{ ttype:    token::ASSIGN,      literal:    String::from("=")           },
            token::Token{ ttype:    token::LPAREN,      literal:    String::from("(")           },
            token::Token{ ttype:    token::IDENT,       literal:    String::from("x")           },
            token::Token{ ttype:    token::COLON,       literal:    String::from(":")           },
            token::Token{ ttype:    token::IDENT,       literal:    String::from("Int")         },
            token::Token{ ttype:    token::COMMA,       literal:    String::from(",")           },
            token::Token{ ttype:    token::IDENT,       literal:    String::from("y")           },
            token::Token{ ttype:    token::COLON,       literal:    String::from(":")           },
            token::Token{ ttype:    token::IDENT,       literal:    String::from("Int")         },
            token::Token{ ttype:    token::RPAREN,      literal:    String::from(")")           },
            token::Token{ ttype:    token::MORPH,       literal:    String::from("->")          },
            token::Token{ ttype:    token::IDENT,       literal:    String::from("Int")         },
            token::Token{ ttype:    token::DO,          literal:    String::from("do")          },
            token::Token{ ttype:    token::RETURN,      literal:    String::from("return")      },
            token::Token{ ttype:    token::IDENT,       literal:    String::from("x")           },
            token::Token{ ttype:    token::PLUS,        literal:    String::from("+")           },
            token::Token{ ttype:    token::IDENT,       literal:    String::from("y")           },
            token::Token{ ttype:    token::SEMICOLON,   literal:    String::from(";")           },
            token::Token{ ttype:    token::END,         literal:    String::from("end")         },
            token::Token{ ttype:    token::SEMICOLON,   literal:    String::from(";")           },
            token::Token{ ttype:    token::LET,         literal:    String::from("let")         },
            token::Token{ ttype:    token::IDENT,       literal:    String::from("result")      },
            token::Token{ ttype:    token::COLON,       literal:    String::from(":")           },
            token::Token{ ttype:    token::IDENT,       literal:    String::from("Int")         },
            token::Token{ ttype:    token::ASSIGN,      literal:    String::from("=")           },
            token::Token{ ttype:    token::IDENT,       literal:    String::from("add")         },
            token::Token{ ttype:    token::LPAREN,      literal:    String::from("(")           },
            token::Token{ ttype:    token::IDENT,       literal:    String::from("five")        },
            token::Token{ ttype:    token::COMMA,       literal:    String::from(",")           },
            token::Token{ ttype:    token::IDENT,       literal:    String::from("ten")         },
            token::Token{ ttype:    token::RPAREN,      literal:    String::from(")")           },
            token::Token{ ttype:    token::SEMICOLON,   literal:    String::from(";")           },
        ];

        let mut l = new(&input);

        let mut i = 0;
        for test in tests.iter() {
            match l.next_token() {
                Some(tok) => assert_eq!(&tok, test, "tests[{}]", i),
                None => assert!(false),
            }
            i += 1;
        }
    }
}
*/

