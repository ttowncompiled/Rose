use std::str::Chars;
use std::iter::Peekable;

use token::TokenType;
use token::Token;

pub struct Lexer<'a> {
    input:      &'a str,
    chars:      Peekable<Chars<'a>>,
    ch:         char,
    ch2:        char,
    ch3:        char,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer {
        let mut l = Lexer{
            input:      input,
            chars:      input.chars().peekable(),
            ch:         '\0',
            ch2:        '\0',
            ch3:        '\0',
        };
        l.read_char();
        l.read_char();
        l.read_char();
        l
    }

    pub fn next_token(&mut self) -> Token {
        Token{
            ttype: TokenType::MetaEOF,
            literal: '\0'.to_string(),
        }
    }

    fn read_char(&mut self) {
        self.ch = self.ch2;
        self.ch2 = self.ch3;
        match self.chars.next() {
            Some(ch) => self.ch3 = ch,
            None => self.ch3 = '\0',
        }
    }
}
