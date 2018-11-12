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
            char_pos:   1,
        };
        l.read_char();
        l.read_char();
        l.read_char();
        l
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token: Token;
        match self.ch {
            '\0' => token = self.single_ch_token(TokenType::MetaEOF),
            '+' => token = self.single_ch_token(TokenType::OpAdd),
            '\n' | '\r' | ';' => token = self.single_ch_token(TokenType::DelEnd),
            '0' ... '9' => token = self.num_token(),
            _  => token = self.single_ch_token(TokenType::MetaIllegal),
        }
        self.read_char();
        token
    }

    fn read_char(&mut self) {
        if self.ch != '\0' {
            self.update_read_pos();
        }
        self.ch = self.ch2;
        self.ch2 = self.ch3;
        match self.chars.next() {
            Some(ch) => self.ch3 = ch,
            None => self.ch3 = '\0',
        }
    }

    fn update_read_pos(&mut self) {
        if self.ch == '\n' || self.ch == '\r' {
            self.line_num += 1;
            self.char_pos = 0;
        }
        self.char_pos += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' {
            self.read_char();
        }
    }

    fn single_ch_token(&self, ttype: TokenType) -> Token {
        Token{
            ttype:      ttype,
            literal:    self.ch.to_string(),
            line_num:   self.line_num,
            char_pos:   self.char_pos,
        }
    }

    fn num_token(&mut self) -> Token {
        let line_num = self.line_num;
        let char_pos = self.char_pos;
        let mut buffer = String::new();
        buffer.push(self.ch);
        while self.peeking_digit() {
            self.read_char();
            buffer.push(self.ch);
        }
        Token{
            ttype:      TokenType::LitInt,
            literal:    buffer,
            line_num:   line_num,
            char_pos:   char_pos,
        }
    }

    fn peeking_digit(&self) -> bool {
        '0' <= self.ch2 && self.ch2 <= '9'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_next_token() {
        test_with_input('\0'.to_string(), TokenType::MetaEOF);
        test_with_input('\\'.to_string(), TokenType::MetaIllegal);
        test_with_input('+'.to_string(), TokenType::OpAdd);
        test_with_input("5".to_string(), TokenType::LitInt);
        test_with_input("55".to_string(), TokenType::LitInt);
        test_with_input('\n'.to_string(), TokenType::DelEnd);
        test_with_input('\r'.to_string(), TokenType::DelEnd);
        test_with_input(';'.to_string(), TokenType::DelEnd);
    }

    fn test_with_input(test: String, ttype: TokenType) {
        let mut lexer = Lexer::new(&test);
        let token = Token{
            ttype: ttype,
            literal: test.clone(),
            line_num: 1,
            char_pos: 1,
        };
        assert_eq!(lexer.next_token(), token);
    }
}
