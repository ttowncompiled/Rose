use std::str::Chars;
use std::iter::Peekable;
use token::Token;
use token::TokenType;
use token::TokenFactory;
use token::RoseTokenFactory;

pub trait Lexer<'a> {
    fn next_token(&mut self) -> Token;
}

pub struct RoseLexer<'a> {
    input:              &'a str,
    file_name:          String,
    chars:              Peekable<Chars<'a>>,
    line_number:        i64,
    char_position:      i64,
    ch:                 char,
    token_factory:      RoseTokenFactory,
}

impl<'a> RoseLexer<'a> {
    pub fn new(input: &'a str, file_name: String) -> RoseLexer<'a> {
        let mut l = RoseLexer{
            input:              input,
            file_name:          file_name.clone(),
            chars:              input.chars().peekable(),
            line_number:        1,
            char_position:      1,
            ch:                 '\0',
            token_factory:      RoseTokenFactory::new(file_name),
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

    fn skip_whitespace(&mut self) {
        while RoseTokenFactory::is_whitespace(self.ch) {
            self.read_char();
        }
    }
}

impl<'a> Lexer<'a> for RoseLexer<'a> {
    fn next_token(&mut self) -> Token {
        let mut token: Option<Token> = None;
        while token.is_none() {
            self.skip_whitespace();
            let peek_ch: char = self.peek_char();
            token = self.token_factory.manufacture(self.ch, peek_ch, self.line_number, self.char_position);
            if self.ch == '\n' || self.ch == '\r' {
                self.line_number += 1;
                self.char_position = 0;
            }
            self.read_char();
        }
        if let Some(tok) = token {
            return tok;
        } else {
            return Token::new(TokenType::META_EOF, '\0'.to_string(), self.line_number, self.char_position);
        }
    }
}

