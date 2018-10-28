#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenType {
    META_ILLEGAL,
    META_EOF,
    RW_LET,
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
    pub char_position:  i64,
}

impl Token {
    pub fn new(ttype: TokenType, literal: String, file_name: String, line_number: i64, char_position: i64) -> Token {
        return Token{
            ttype:          ttype,
            literal:        literal,
            file_name:      file_name,
            line_number:    line_number,
            char_position:  char_position,
        };
    }
}

#[derive(Debug, PartialEq, Eq)]
struct TokenBuilder {
    pub ttype:          TokenType,
    pub literal:        String,
    pub line_number:    i64,
    pub char_position:  i64,
}

impl TokenBuilder {
    pub fn new(ttype: TokenType, literal: String, line_number: i64, char_position: i64) -> TokenBuilder {
        return TokenBuilder{
            ttype:          ttype,
            literal:        literal,
            line_number:    line_number,
            char_position:  char_position,
        };
    }

    pub fn push(&mut self, ch: char) {
        self.literal.push(ch);
    }

    pub fn build(&self, file_name: String) -> Option<Token> {
        if self.literal != "" && file_name != "" && self.line_number > 0 && self.char_position > 0 {
            return Some(Token::new(self.ttype.clone(), self.literal.clone(), file_name, self.line_number, self.char_position));
        }
        return None;
    }
}

pub trait TokenFactory {
    fn new(file_name: String) -> Self;
    fn manufacture(&mut self, ch: char, peek_ch: char, line_number: i64, char_position: i64) -> Option<Token>;
    fn close(&mut self, line_number: i64, char_position: i64) -> Option<Token>;
    fn is_closed(&self) -> bool;
    fn lookup_ident(literal: &str) -> TokenType;
    fn is_letter(ch: char) -> bool;
    fn is_digit(ch: char) -> bool;
    fn is_special_char(ch: char) -> bool;
    fn is_whitespace(ch: char) -> bool;
}

pub struct RoseTokenFactory {
    pub file_name:      String,
    builder:            Option<TokenBuilder>,
    closed:             bool,
}

impl TokenFactory for RoseTokenFactory {
    fn new(file_name: String) -> Self {
        return RoseTokenFactory{
            file_name:      file_name,
            builder:        None,
            closed:         false,
        };
    }

    fn manufacture(&mut self, ch: char, peek_ch: char, line_number: i64, char_position: i64) -> Option<Token> {
        if self.closed {
            return None
        }
        let mut token: Option<Token>;
        match ch {
            '+'     => token = TokenBuilder::new(TokenType::OP_ADD, '+'.to_string(), line_number, char_position).build(self.file_name.clone()),
            '-'     => token = TokenBuilder::new(TokenType::OP_SUB, '-'.to_string(), line_number, char_position).build(self.file_name.clone()),
            '*'     => token = TokenBuilder::new(TokenType::OP_MUL, '*'.to_string(), line_number, char_position).build(self.file_name.clone()),
            '/'     => token = TokenBuilder::new(TokenType::OP_DIV, '/'.to_string(), line_number, char_position).build(self.file_name.clone()),
            '\n'    => token = TokenBuilder::new(TokenType::DEL_END, '\n'.to_string(), line_number, char_position).build(self.file_name.clone()),
            '\r'    => token = TokenBuilder::new(TokenType::DEL_END, '\r'.to_string(), line_number, char_position).build(self.file_name.clone()),
            ';'     => token = TokenBuilder::new(TokenType::DEL_END, ';'.to_string(), line_number, char_position).build(self.file_name.clone()),
            '_'     => {
                match self.builder {
                    Some(ref mut builder) => {
                        if builder.ttype == TokenType::LIT_IDENT && (Self::is_letter(peek_ch) || Self::is_digit(peek_ch) || Self::is_special_char(peek_ch)) {
                            builder.push(ch);
                            token = None;
                        } else if builder.ttype == TokenType::LIT_IDENT {
                            builder.push(ch);
                            token = builder.build(self.file_name.clone());
                        } else {
                            panic!("cannot process character {} at {}:{}:{}, building {:?}", ch, self.file_name.clone(), line_number, char_position, builder);
                        }
                    },
                    None => {
                        if Self::is_letter(peek_ch) {
                            self.builder = Some(TokenBuilder::new(TokenType::LIT_IDENT, '_'.to_string(), line_number, char_position));
                            token = None;
                        } else {
                            token = TokenBuilder::new(TokenType::LIT_BLANK, '_'.to_string(), line_number, char_position).build(self.file_name.clone());
                        }
                    },
                }
            },
            _       => {
                match self.builder {
                    Some(ref mut builder) => {
                        if builder.ttype == TokenType::LIT_IDENT && (Self::is_letter(ch) || Self::is_digit(ch) || Self::is_special_char(ch)) {
                            if Self::is_special_char(ch) {
                                builder.push(ch);
                                token = builder.build(self.file_name.clone());
                            } else if Self::is_letter(peek_ch) || Self::is_digit(peek_ch) || Self::is_special_char(peek_ch) {
                                builder.push(ch);
                                token = None;
                            } else {
                                builder.push(ch);
                                token = builder.build(self.file_name.clone());
                            }
                        } else if builder.ttype == TokenType::LIT_INT && Self::is_digit(ch) {
                            if Self::is_digit(peek_ch) {
                                builder.push(ch);
                                token = None;
                            } else {
                                builder.push(ch);
                                token = builder.build(self.file_name.clone());
                            }
                        } else {
                            panic!("cannot process character {} at {}:{}:{}, building {:?}", ch, self.file_name.clone(), line_number, char_position, builder);
                        }
                    },
                    None => {
                        if Self::is_letter(ch) {
                            if Self::is_letter(peek_ch) || Self::is_digit(peek_ch) || Self::is_special_char(peek_ch) {
                                self.builder = Some(TokenBuilder::new(TokenType::LIT_IDENT, ch.to_string(), line_number, char_position));
                                token = None;
                            } else {
                                token = TokenBuilder::new(TokenType::LIT_IDENT, ch.to_string(), line_number, char_position).build(self.file_name.clone());
                            }
                        } else if Self::is_digit(ch) {
                            if Self::is_digit(peek_ch) {
                                self.builder = Some(TokenBuilder::new(TokenType::LIT_INT, ch.to_string(), line_number, char_position));
                                token = None
                            } else {
                                token = TokenBuilder::new(TokenType::LIT_INT, ch.to_string(), line_number, char_position).build(self.file_name.clone());
                            }
                        } else {
                            token = TokenBuilder::new(TokenType::META_ILLEGAL, ch.to_string(), line_number, char_position).build(self.file_name.clone());
                        }
                    },
                }
            },
        }
        match token {
            Some(ref mut tok) => {
                if tok.ttype == TokenType::LIT_IDENT {
                    tok.ttype = Self::lookup_ident(&tok.literal);
                }
                self.builder = None;
            },
            None => (),
        }
        return token;
    }

    fn close(&mut self, line_number: i64, char_position: i64) -> Option<Token> {
        if self.closed {
            return None
        }
        return TokenBuilder::new(TokenType::META_EOF, '\0'.to_string(), line_number, char_position).build(self.file_name.clone());
    }

    fn is_closed(&self) -> bool {
        return self.closed;
    }

    fn lookup_ident(literal: &str) -> TokenType {
        return match literal {
            "let"   => TokenType::RW_LET,
            _       => TokenType::LIT_IDENT,
        };
    }

    fn is_letter(ch: char) -> bool {
        return 'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_';
    }

    fn is_digit(ch: char) -> bool {
        return '0' <= ch && ch <= '9';
    }

    fn is_special_char(ch: char) -> bool {
        return ch == '!' || ch == '?';
    }

    fn is_whitespace(ch: char) -> bool {
        return ch == ' ' || ch == '\t';
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup_ident() {
        assert_eq!(RoseTokenFactory::lookup_ident(&("let".to_string())), TokenType::RW_LET);
    }
}

