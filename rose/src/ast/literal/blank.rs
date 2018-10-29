use ast::Node;
use ast::Expression;
use token::Token;

#[derive(Debug, PartialEq, Eq)]
pub struct Blank {
    pub token:      Token,
}

impl Blank {
    pub fn new(token: Token) -> Blank {
        return Blank{
            token:      token,
        };
    }
}

impl ToString for Blank {
    fn to_string(&self) -> String {
        return self.token.literal.clone();
    }
}

impl Node for Blank {
    fn token_literal(&self) -> Option<String> {
        return Some(self.token.literal.clone());
    }
}

impl Expression for Blank {}

#[cfg(test)]
mod test {
    use super::*;
    use token::TokenType;

    #[test]
    fn test_to_string() {
        let blank: Blank = Blank::new(Token::new(TokenType::LIT_BLANK, "_".to_string(), 1, 1));
        assert_eq!("_", blank.to_string());
    }
}
