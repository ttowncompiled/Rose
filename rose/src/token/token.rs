pub type TokenType = &'static str;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub ttype:      TokenType,
    pub literal:    String,
}

impl Token {
    pub fn new(ttype: TokenType, literal: String) -> Token {
        return Token{ ttype: ttype, literal: literal };
    }
}

// Metacharacters
pub static ILLEGAL:     TokenType       = "ILLEGAL";    // unrecognized character
pub static EOF:         TokenType       = "EOF";        // end-of-file

// Identifiers + Literals
pub static IDENT:       TokenType       = "IDENT";      // add, foobar, x, y, ...
pub static INT_LIT:     TokenType       = "INT_LIT";    // 1343456
pub static INT:         TokenType       = "Int";
pub static LET:         TokenType       = "let";
pub static DO:          TokenType       = "do";
pub static END:         TokenType       = "end";
pub static RETURN:      TokenType       = "return";

// Operators
pub static ASSIGN:      TokenType       = "=";
pub static PLUS:        TokenType       = "+";
pub static MINUS:       TokenType       = "-";
pub static MORPH:       TokenType       = "->";

// Delimiters
pub static COMMA:       TokenType       = ",";
pub static SEMICOLON:   TokenType       = ";";
pub static COLON:       TokenType       = ":";

// Collections + Scopes
pub static LPAREN:      TokenType       = "(";
pub static RPAREN:      TokenType       = ")";

pub fn lookup_ident(ident: &String) -> TokenType {
    match ident.as_str() {
        "Int"       => INT,
        "let"       => LET,
        "do"        => DO,
        "end"       => END,
        "return"    => RETURN,
        _           => IDENT,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup_ident() {
        assert_eq!(lookup_ident(&String::from("Int")), INT);
        assert_eq!(lookup_ident(&String::from("let")), LET);
        assert_eq!(lookup_ident(&String::from("do")), DO);
        assert_eq!(lookup_ident(&String::from("end")), END);
        assert_eq!(lookup_ident(&String::from("return")), RETURN);
    }
}

