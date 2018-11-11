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
    line_num:   i32,
    char_pos:   i32,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer {
        let mut l = Lexer{
            input:      input,
            chars:      input.chars().peekable(),
            ch:         '\0',
            ch2:        '\0',
            ch3:        '\0',
            line_num:   1,
            char_pos:   0,
        };
        l.read_char();
        l.read_char();
        l.read_char();
        l
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        match self.ch {
            '\0'    => {
                let token = self.basic_token(TokenType::MetaEOF);
                self.read_char();
                token
            },
            '+'     => {
                let token = self.basic_token(TokenType::OpAdd);
                self.read_char();
                token
            },
            _       => {
                let token = self.basic_token(TokenType::MetaIllegal);
                self.read_char();
                token
            },
        }
    }

    fn read_char(&mut self) {
        if self.ch == '\n' || self.ch == '\r' {
            self.line_num += 1;
            self.char_pos = 0;
        }
        if self.ch != '\0' {
            self.char_pos += 1;
        }
        self.ch = self.ch2;
        self.ch2 = self.ch3;
        match self.chars.next() {
            Some(ch) => self.ch3 = ch,
            None => self.ch3 = '\0',
        }
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' {
            self.read_char();
        }
    }

    fn basic_token(&self, ttype: TokenType) -> Token {
        Token{
            ttype:      ttype,
            literal:    self.ch.to_string(),
            line_num:   self.line_num,
            char_pos:   self.char_pos,
        }
    }

    fn lit_token(&self, ttype: TokenType, literal: String) -> Token {
        Token{
            ttype:      ttype,
            literal:    literal,
            line_num:   self.line_num,
            char_pos:   self.char_pos,
        }
    }
}
