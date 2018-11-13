use std::str::Chars;
use std::iter::Peekable;

use syntax::token::*;

pub struct Lexer<'a> {
    chars:      Peekable<Chars<'a>>,
    ch:         char,
    ch2:        char,
    ch3:        char,
    line_num:   i32,
    col_num:    i32,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer {
        let mut l = Lexer{
            chars:      input.chars().peekable(),
            ch:         '\0',
            ch2:        '\0',
            ch3:        '\0',
            line_num:   1,
            col_num:    1,
        };
        l.read_char();
        l.read_char();
        l.read_char();
        l
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.ch {
            '\0' => self.single_ch_token(TokenType::MetaEOF),
            '+' => self.single_ch_token(TokenType::OpAdd),
            '^' => self.single_ch_token(TokenType::OpDeref),
            '(' => self.single_ch_token(TokenType::DelOpenParen),
            ')' => self.single_ch_token(TokenType::DelCloseParen),
            ':' => self.single_ch_token(TokenType::DelColon),
            ',' => self.single_ch_token(TokenType::DelComma),
            '\n' | '\r' | ';' => self.single_ch_token(TokenType::DelEnd),
            '0' ... '9' => self.num_token(),
            '_' | 'a' ... 'z' | 'A' ... 'Z' => self.ident_token(),
            _  => self.single_ch_token(TokenType::MetaIllegal),
        };
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
            self.col_num = 0;
        }
        self.col_num += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' {
            self.read_char();
        }
    }

    fn single_ch_token(&self, ttype: TokenType) -> Token {
        Token{
            ttype:      ttype,
            raw:        self.ch.to_string(),
            line_num:   self.line_num,
            col_num:    self.col_num,
        }
    }

    fn num_token(&mut self) -> Token {
        let line_num = self.line_num;
        let col_num = self.col_num;
        let mut buffer = String::new();
        buffer.push(self.ch);
        while self.peeking_digit() {
            self.read_char();
            buffer.push(self.ch);
        }
        Token{
            ttype:      TokenType::LitInt,
            raw:        buffer,
            line_num:   line_num,
            col_num:    col_num,
        }
    }

    fn ident_token(&mut self) -> Token {
        let line_num = self.line_num;
        let col_num = self.col_num;
        let mut buffer = String::new();
        buffer.push(self.ch);
        while self.peeking_letter() || self.peeking_digit() {
            self.read_char();
            buffer.push(self.ch);
        }
        if self.peeking_special_char() {
            self.read_char();
            buffer.push(self.ch);
        }
        Token{
            ttype:      self.lookup_ident(&buffer),
            raw:        buffer,
            line_num:   line_num,
            col_num:    col_num,
        }
    }

    fn lookup_ident(&self, raw: &str) -> TokenType {
        match raw {
            "class" => TokenType::RwClass,
            "def" => TokenType::RwDef,
            "end" => TokenType::RwEnd,
            "pub" => TokenType::RwPub,
            "as" => TokenType::KwAs,
            _ => TokenType::LitIdent,
        }
    }

    fn peeking_digit(&self) -> bool {
        '0' <= self.ch2 && self.ch2 <= '9' || self.ch2 == '_'
    }

    fn peeking_letter(&self) -> bool {
        self.ch2 == '_' || ('a' <= self.ch2 && self.ch2 <= 'z')
            || ('A' <= self.ch2 && self.ch2 <= 'Z')
    }

    fn peeking_special_char(&self) -> bool {
        self.ch2 == '!' || self.ch2 == '?'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_next_token() {
        test_with_input("\\", TokenType::MetaIllegal);
        test_with_input("\0", TokenType::MetaEOF);
        test_with_input("class", TokenType::RwClass);
        test_with_input("def", TokenType::RwDef);
        test_with_input("end", TokenType::RwEnd);
        test_with_input("pub", TokenType::RwPub);
        test_with_input("as", TokenType::KwAs);
        test_with_input("+", TokenType::OpAdd);
        test_with_input("^", TokenType::OpDeref);
        test_with_input("5", TokenType::LitInt);
        test_with_input("55", TokenType::LitInt);
        test_with_input("1_000_000", TokenType::LitInt);
        test_with_input("_x", TokenType::LitIdent);
        test_with_input("foo", TokenType::LitIdent);
        test_with_input("Foo123", TokenType::LitIdent);
        test_with_input("_F_o_1_a_2", TokenType::LitIdent);
        test_with_input("foo!", TokenType::LitIdent);
        test_with_input("foo?", TokenType::LitIdent);
        test_with_input("(", TokenType::DelOpenParen);
        test_with_input(")", TokenType::DelCloseParen);
        test_with_input(":", TokenType::DelColon);
        test_with_input(",", TokenType::DelComma);
        test_with_input("\n", TokenType::DelEnd);
        test_with_input("\r", TokenType::DelEnd);
        test_with_input(";", TokenType::DelEnd);
    }

    fn test_with_input<'a>(test: &'a str, ttype: TokenType) {
        let mut lexer = Lexer::new(&test);
        let token = Token{
            ttype:      ttype,
            raw:        test.to_string(),
            line_num:   1,
            col_num:    1,
        };
        assert_eq!(lexer.next_token(), token);
    }
}
