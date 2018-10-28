#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    META_ILLEGAL,
    META_EOF,
    OP_ADD,
    OP_SUB,
    OP_MUL,
    OP_DIV,
    DEL_END,
    LIT_IDENT,
    LIT_BLANK,
    LIT_INT,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    pub ttype:          TokenType,
    pub literal:        String,
    pub file_name:      String,
    pub line_number:    i64,
    pub char_offset:    i64,
}

impl Token {
    pub fn new(ttype: TokenType, literal: String, file_name: String, line_number: i64, char_offset: i64) -> Token {
        return Token{
            ttype:          ttype,
            literal:        literal,
            file_name:      file_name,
            line_number:    line_number,
            char_offset:    char_offset,
        };
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TokenBuilder {
    pub ttype:          TokenType,
    pub literal:        String,
    pub file_name:      String,
    pub line_number:    i64,
    pub char_offset:    i64,
}

impl TokenBuilder {
    pub fn new(ttype: TokenType, literal: String, file_name: String, line_number: i64, char_offset: i64) -> TokenBuilder {
        return TokenBuilder{
            ttype:          ttype,
            literal:        literal,
            file_name:      file_name,
            line_number:    line_number,
            char_offset:    char_offset,
        };
    }

    pub fn push(&mut self, ch: char) {
        self.literal.push(ch);
    }

    pub fn build(&self) -> Option<Token> {
        if self.literal != "" && self.file_name != "" && self.line_number > 0 && self.char_offset > 0 {
            return Some(Token::new(self.ttype, self.literal, self.file_name, self.line_number, self.char_offset));
        }
        return None;
    }
}

pub trait TokenFactory {
    fn new(file_name: String) -> Self;
    fn manufacture(&mut self, ch: char, peek_ch: char, line_number: i64, char_offset: i64) -> Option<Token>;
    fn complete(&mut self, line_number: i64, char_offset: i64) -> Option<Token>;
    fn is_letter(&self, ch: char) -> bool;
    fn is_digit(&self, ch: char) -> bool;
    fn is_special_char(&self, ch: char) -> bool;
    fn is_whitespace(&self, ch: char) -> bool;
}

pub struct RoseTokenFactory {
    pub file_name:      String,
    pub builder:        Option<TokenBuilder>,
}

impl TokenFactory for RoseTokenFactory {
    fn new(file_name: String) -> Self {
        return RoseTokenFactory{
            file_name:      file_name,
            builder:        None,
        };
    }

    pub fn manufacture(&mut self, ch: char, peek_ch: char, line_number: i64, char_offset: i64) -> Option<Token> {
        match ch {
            '+'     => return TokenBuilder::new(TokenType::OP_ADD, '+'.to_string(), self.file_name, line_number, char_offset).build(),
            '-'     => return TokenBuilder::new(TokenType::OP_SUB, '-'.to_string(), self.file_name, line_number, char_offset).build(),
            '*'     => return TokenBuilder::new(TokenType::OP_MUL, '*'.to_string(), self.file_name, line_number, char_offset).build(),
            '/'     => return TokenBuilder::new(TokenType::OP_DIV, '/'.to_string(), self.file_name, line_number, char_offset).build(),
            '\n'    => return TokenBuilder::new(TokenType::DEL_END, '\n'.to_string(), self.file_name, line_number, char_offset).build(),
            '\r'    => return TokenBuilder::new(TokenType::DEL_END, '\r'.to_string(), self.file_name, line_number, char_offset).build(),
            ';'     => return TokenBuilder::new(TokenType::DEL_END, ';'.to_string(), self.file_name, line_number, char_offset).build(),
            '_'     => {
                match self.builder {
                    Some(builder) => {
                        if builder.ttype == TokenType::LIT_IDENT && (self.is_letter(peek_ch) || self.is_digit(peek_ch) || self.is_special_char(peek_ch)) {
                            builder.push(ch);
                            return None;
                        } else if builder.ttype == TokenType::LIT_IDENT {
                            builder.push(ch);
                            self.builder = None;
                            return builder.build();
                        } else {
                            panic!("cannot process character {} at {}:{}:{}, building {:?}", ch, self.file_name, line_number, char_offset, builder);
                        }
                    },
                    None => {
                        if self.is_letter(peek_ch) {
                            self.builder = Some(TokenBuilder::new(TokenType::LIT_IDENT, '_'.to_string(), self.file_name, line_number, char_offset));
                            return None;
                        } else {
                            return TokenBuilder::new(TokenType::LIT_BLANK, '_'.to_string(), self.file_name, line_number, char_offset).build();
                        }
                    },
                }
            },
            _       => {
                match self.builder {
                    Some(builder) {
                        if builder.ttype == TokenType::LIT_IDENT && (self.is_letter(ch) || self.is_digit(ch) || self.is_special_char(ch)) {
                            if self.is_special_char(ch) {
                                builder.push(ch);
                                self.builder = None;
                                return builder.build();
                            }
                            if self.is_letter(peek_ch) || self.is_digit(peek_ch) || self.is_special_char(peek_ch) {
                                build.push(ch);
                                return None;
                            } else {
                                builder.push(ch);
                                self.builder = None;
                                return builder.build();
                            }
                        } else if builder.ttype == TokenType::LIT_INT && self.is_digit(ch) {
                            if self.is_digit(peek_ch) {
                                builder.push(ch);
                                return None;
                            } else {
                                builder.push(ch);
                                self.builder = None;
                                return builder.build();
                            }
                        } else {
                            panic!("cannot process character {} at {}:{}:{}, building {:?}", ch, self.file_name, line_number, char_offset, builder);
                        }
                    },
                    None => {
                        if self.is_letter(ch) {
                            if self.is_letter(peek_ch) || self.is_digit(peek_ch) || self.is_special_char(peek_ch) {
                                self.builder = Some(TokenBuilder::new(TokenType::LIT_IDENT, ch.to_string(), self.file_name, line_number, char_offset));
                                return None;
                            } else {
                                return TokenBuilder::new(TokenType::LIT_IDENT, ch.to_string(), self.file_name, line_number, char_offset).build();
                            }
                        } else if self.is_digit(ch) {
                            if self.is_digit(peek_ch) {
                                self.builder = Some(TokenBuilder::new(TokenType::LIT_INT, ch.to_string(), self.file_name, line_number, char_offset));
                                return None;
                            } else {
                                return TokenBuilder::new(TokenType::LIT_INT, ch.to_string(), self.file_name, line_number, char_offset).build();
                            }
                        } else {
                            return TokenBuilder::new(TokenType::META_ILLEGAL, ch.to_string(), self.file_name, line_number, char_offset).build();
                        }
                    },
                }
            },
        }
    }

    pub fn complete(&mut self, line_number: i64, char_offset: i64) -> Option<Token> {
        return TokenBuilder::new(TokenType::META_EOF, '\0'.to_string(), self.file_name, line_number, char_offset).build();
    }

    pub fn is_letter(&self, ch: char) -> bool {
        return 'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_';
    }

    pub fn is_digit(&self, ch: char) -> bool {
        return '0' <= ch && ch <= '9';
    }

    pub fn is_special_char(&self, ch: char) -> bool {
        return ch == '!' || ch == '?';
    }

    pub fn is_whitespace(&self, ch: char) -> bool {
        return ch == ' ' || ch == '\t';
    }
}

