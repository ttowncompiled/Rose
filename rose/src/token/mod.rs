#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenType {
    META_ILLEGAL,
    META_EOF,
    RW_AND,
    RW_LET,
    RW_NOT,
    RW_OR,
    OP_ADD,
    OP_SUB,
    OP_MUL,
    OP_DIV,
    OP_MOD,
    OP_POW,
    OP_NOT,
    OP_EQ,
    OP_NEQ,
    OP_GT,
    OP_GTE,
    OP_LT,
    OP_LTE,
    OP_ASSIGN,
    DEL_END,
    DEL_COLON,
    DEL_LPAREN,
    DEL_RPAREN,
    LIT_IDENT,
    LIT_BLANK,
    LIT_INT,
    LIT_FLOAT,
    LIT_BOOL,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    pub ttype:          TokenType,
    pub literal:        String,
    pub line_number:    i64,
    pub char_position:  i64,
}

impl Token {
    pub fn new(ttype: TokenType, literal: String, line_number: i64, char_position: i64) -> Token {
        return Token{
            ttype:          ttype,
            literal:        literal,
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

    pub fn build(&self) -> Option<Token> {
        if self.literal != "" && self.line_number > 0 && self.char_position > 0 {
            return Some(Token::new(self.ttype.clone(), self.literal.clone(), self.line_number, self.char_position));
        }
        return None;
    }
}

pub trait TokenFactory {
    fn manufacture(&mut self, ch: char, peek_ch: char, line_number: i64, char_position: i64) -> Option<Token>;
}

pub struct RoseTokenFactory {
    pub file_name:      String,
    builder:            Option<TokenBuilder>,
}

impl RoseTokenFactory {
    pub fn new(file_name: String) -> Self {
        return RoseTokenFactory{
            file_name:      file_name,
            builder:        None,
        };
    }

    fn lookup_ident(literal: &str) -> TokenType {
        return match literal {
            "let"       => TokenType::RW_LET,
            "true"      => TokenType::LIT_BOOL,
            "false"     => TokenType::LIT_BOOL,
            "and"       => TokenType::RW_AND,
            "or"        => TokenType::RW_OR,
            "not"       => TokenType::RW_NOT,
            _           => TokenType::LIT_IDENT,
        };
    }

    fn is_letter(ch: char) -> bool {
        return 'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z';
    }

    fn is_digit(ch: char) -> bool {
        return '0' <= ch && ch <= '9';
    }

    fn is_special_char(ch: char) -> bool {
        return ch == '!' || ch == '?';
    }

    pub fn is_whitespace(ch: char) -> bool {
        return ch == ' ' || ch == '\t';
    }

    fn throw_manufacture_error(ch: char, file_name: String, line_number: i64, char_position: i64, builder: &TokenBuilder) {
        panic!("cannot process character {} at {}:{}:{}, building {:?}", ch, file_name, line_number, char_position, builder);
    }
}

impl TokenFactory for RoseTokenFactory {
    fn manufacture(&mut self, ch: char, peek_ch: char, line_number: i64, char_position: i64) -> Option<Token> {
        let mut token: Option<Token> = None;
        match ch {
            '\0'    => token = TokenBuilder::new(TokenType::META_EOF, '\0'.to_string(), line_number, char_position).build(),
            '+'     => token = TokenBuilder::new(TokenType::OP_ADD, '+'.to_string(), line_number, char_position).build(),
            '-'     => token = TokenBuilder::new(TokenType::OP_SUB, '-'.to_string(), line_number, char_position).build(),
            '*'     => {
                match self.builder {
                    Some(ref mut builder) => {
                        if builder.ttype == TokenType::OP_POW {
                            builder.push(ch);
                            token = builder.build();
                        } else {
                            Self::throw_manufacture_error(ch, self.file_name.clone(), line_number, char_position, builder);
                        }
                    },
                    None => {
                        if peek_ch == '*' {
                            self.builder = Some(TokenBuilder::new(TokenType::OP_POW, '*'.to_string(), line_number, char_position));
                            token = None;
                        } else {
                            token = TokenBuilder::new(TokenType::OP_MUL, '*'.to_string(), line_number, char_position).build();
                        }
                    },
                }
            },
            '/'     => token = TokenBuilder::new(TokenType::OP_DIV, '/'.to_string(), line_number, char_position).build(),
            '%'     => token = TokenBuilder::new(TokenType::OP_MOD, '%'.to_string(), line_number, char_position).build(),
            '='     => {
                match self.builder {
                    Some(ref mut builder) => {
                        if builder.ttype == TokenType::OP_EQ || builder.ttype == TokenType::OP_NEQ || builder.ttype == TokenType::OP_GTE || builder.ttype == TokenType::OP_LTE {
                            builder.push(ch);
                            token = builder.build();
                        } else {
                            Self::throw_manufacture_error(ch, self.file_name.clone(), line_number, char_position, builder);
                        }
                    },
                    None => {
                        if peek_ch == '=' {
                            self.builder = Some(TokenBuilder::new(TokenType::OP_EQ, '='.to_string(), line_number, char_position));
                            token = None;
                        } else {
                            token = TokenBuilder::new(TokenType::OP_ASSIGN, '='.to_string(), line_number, char_position).build();
                        }
                    },
                }
            },
            '\n'    => token = TokenBuilder::new(TokenType::DEL_END, '\n'.to_string(), line_number, char_position).build(),
            '\r'    => token = TokenBuilder::new(TokenType::DEL_END, '\r'.to_string(), line_number, char_position).build(),
            ';'     => token = TokenBuilder::new(TokenType::DEL_END, ';'.to_string(), line_number, char_position).build(),
            '('     => token = TokenBuilder::new(TokenType::DEL_LPAREN, '('.to_string(), line_number, char_position).build(),
            ')'     => token = TokenBuilder::new(TokenType::DEL_RPAREN, ')'.to_string(), line_number, char_position).build(),
            ':'     => token = TokenBuilder::new(TokenType::DEL_COLON, ':'.to_string(), line_number, char_position).build(),
            '!'     => {
                match self.builder {
                    Some(ref mut builder) => {
                        if builder.ttype == TokenType::LIT_IDENT {
                            builder.push(ch);
                            token = builder.build();
                        } else {
                            Self::throw_manufacture_error(ch, self.file_name.clone(), line_number, char_position, builder);
                        }
                    },
                    None => {
                        if peek_ch == '=' {
                            self.builder = Some(TokenBuilder::new(TokenType::OP_NEQ, '!'.to_string(), line_number, char_position));
                            token = None;
                        } else {
                            token = TokenBuilder::new(TokenType::OP_NOT, '!'.to_string(), line_number, char_position).build();
                        }
                    },
                }
            },
            '>'     => {
                if peek_ch == '=' {
                    self.builder = Some(TokenBuilder::new(TokenType::OP_GTE, '>'.to_string(), line_number, char_position));
                    token = None;
                } else {
                    token = TokenBuilder::new(TokenType::OP_GT, '>'.to_string(), line_number, char_position).build();
                }
            },
            '<'     => {
                if peek_ch == '=' {
                    self.builder = Some(TokenBuilder::new(TokenType::OP_LTE, '<'.to_string(), line_number, char_position));
                    token = None;
                } else {
                    token = TokenBuilder::new(TokenType::OP_LT, '<'.to_string(), line_number, char_position).build();
                }
            },
            _       => {
                match self.builder {
                    Some(ref mut builder) => {
                        if builder.ttype == TokenType::LIT_IDENT && (Self::is_letter(ch) || Self::is_digit(ch) || Self::is_special_char(ch) || ch == '_') {
                            builder.push(ch);
                            if Self::is_special_char(ch) {
                                token = builder.build();
                            } else if Self::is_letter(peek_ch) || Self::is_digit(peek_ch) || Self::is_special_char(peek_ch) || peek_ch == '_' {
                                token = None;
                            } else {
                                token = builder.build();
                            }
                        } else if builder.ttype == TokenType::LIT_INT && (Self::is_digit(ch) || ch == '.') {
                            builder.push(ch);
                            if ch == '.' {
                                builder.ttype = TokenType::LIT_FLOAT;
                                if Self::is_digit(peek_ch) {
                                    token = None;
                                } else {
                                    token = builder.build();
                                }
                            } else if Self::is_digit(peek_ch) || peek_ch == '.' {
                                token = None;
                            } else {
                                token = builder.build();
                            }
                        } else if builder.ttype == TokenType::LIT_FLOAT && Self::is_digit(ch) {
                            builder.push(ch);
                            if Self::is_digit(peek_ch) {
                                token = None;
                            } else {
                                token = builder.build();
                            }
                        } else {
                            Self::throw_manufacture_error(ch, self.file_name.clone(), line_number, char_position, builder);
                        }
                    },
                    None => {
                        if ch == '_' {
                            if Self::is_letter(peek_ch) {
                                self.builder = Some(TokenBuilder::new(TokenType::LIT_IDENT, '_'.to_string(), line_number, char_position));
                                token = None;
                            } else {
                                token = TokenBuilder::new(TokenType::LIT_BLANK, '_'.to_string(), line_number, char_position).build();
                            }
                        } else if Self::is_letter(ch) {
                            if Self::is_letter(peek_ch) || Self::is_digit(peek_ch) || Self::is_special_char(peek_ch) || peek_ch == '_' {
                                self.builder = Some(TokenBuilder::new(TokenType::LIT_IDENT, ch.to_string(), line_number, char_position));
                                token = None;
                            } else {
                                token = TokenBuilder::new(TokenType::LIT_IDENT, ch.to_string(), line_number, char_position).build();
                            }
                        } else if Self::is_digit(ch) {
                            if Self::is_digit(peek_ch) || peek_ch == '.' {
                                self.builder = Some(TokenBuilder::new(TokenType::LIT_INT, ch.to_string(), line_number, char_position));
                                token = None
                            } else {
                                token = TokenBuilder::new(TokenType::LIT_INT, ch.to_string(), line_number, char_position).build();
                            }
                        } else {
                            token = TokenBuilder::new(TokenType::META_ILLEGAL, ch.to_string(), line_number, char_position).build();
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::Chars;
    use std::iter::Peekable;

    #[test]
    fn test_manufacture() {
        test_factory_with("\\".to_string(), TokenType::META_ILLEGAL);
        test_factory_with("\0".to_string(), TokenType::META_EOF);
        test_factory_with("and".to_string(), TokenType::RW_AND);
        test_factory_with("let".to_string(), TokenType::RW_LET);
        test_factory_with("not".to_string(), TokenType::RW_NOT);
        test_factory_with("or".to_string(), TokenType::RW_OR);
        test_factory_with("+".to_string(), TokenType::OP_ADD);
        test_factory_with("-".to_string(), TokenType::OP_SUB);
        test_factory_with("*".to_string(), TokenType::OP_MUL);
        test_factory_with("/".to_string(), TokenType::OP_DIV);
        test_factory_with("%".to_string(), TokenType::OP_MOD);
        test_factory_with("**".to_string(), TokenType::OP_POW);
        test_factory_with("!".to_string(), TokenType::OP_NOT);
        test_factory_with("==".to_string(), TokenType::OP_EQ);
        test_factory_with("!=".to_string(), TokenType::OP_NEQ);
        test_factory_with(">".to_string(), TokenType::OP_GT);
        test_factory_with(">=".to_string(), TokenType::OP_GTE);
        test_factory_with("<".to_string(), TokenType::OP_LT);
        test_factory_with("<=".to_string(), TokenType::OP_LTE);
        test_factory_with("=".to_string(), TokenType::OP_ASSIGN);
        test_factory_with("\n".to_string(), TokenType::DEL_END);
        test_factory_with("\r".to_string(), TokenType::DEL_END);
        test_factory_with(";".to_string(), TokenType::DEL_END);
        test_factory_with("(".to_string(), TokenType::DEL_LPAREN);
        test_factory_with(")".to_string(), TokenType::DEL_RPAREN);
        test_factory_with(":".to_string(), TokenType::DEL_COLON);
        test_factory_with("x".to_string(), TokenType::LIT_IDENT);
        test_factory_with("foo".to_string(), TokenType::LIT_IDENT);
        test_factory_with("_F_o_O_1_!".to_string(), TokenType::LIT_IDENT);
        test_factory_with("_".to_string(), TokenType::LIT_BLANK);
        test_factory_with("5".to_string(), TokenType::LIT_INT);
        test_factory_with("55".to_string(), TokenType::LIT_INT);
        test_factory_with("5.5".to_string(), TokenType::LIT_FLOAT);
        test_factory_with("true".to_string(), TokenType::LIT_BOOL);
        test_factory_with("false".to_string(), TokenType::LIT_BOOL);
    }

    fn test_factory_with(input: String, exp_ttype: TokenType) {
        let mut factory: RoseTokenFactory = RoseTokenFactory::new(input.clone());
        let mut chars: Peekable<Chars> = input.chars().peekable();
        let mut cp: i64 = 1;
        while cp < input.len() as i64 {
            test_token(&mut factory, input.clone(), chars.next().unwrap_or('\0'), *chars.peek().unwrap_or(&'\0'), 1, cp, None);
            cp += 1;
        }
        test_token(&mut factory, input.clone(), chars.next().unwrap_or('\0'), '\0', 1, cp, Some(exp_ttype));
    }

    fn test_token(factory: &mut RoseTokenFactory, input: String, ch: char, peek_ch: char, ln: i64, cp: i64, exp_ttype: Option<TokenType>) {
        let expected: Option<Token> = match exp_ttype {
            Some(ttype) => Some(Token::new(ttype, input.clone(), 1, 1)),
            None => None,
        };
        let got: Option<Token> = factory.manufacture(ch, peek_ch, ln, cp);
        assert_eq!(expected, got, "test \"{}\"", input);
    }

    #[test]
    fn test_lookup_ident() {
        assert_eq!(RoseTokenFactory::lookup_ident(&("let".to_string())), TokenType::RW_LET);
    }
}

