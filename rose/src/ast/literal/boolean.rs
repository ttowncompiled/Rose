use ast::Node;
use ast::Expression;
use token::Token;

#[derive(Debug, PartialEq, Eq)]
pub struct BooleanLiteral {
    pub token:      Token,
    pub value:      bool,
}

impl BooleanLiteral {
    pub fn new(token: Token, value: bool) -> BooleanLiteral {
        return BooleanLiteral{
            token:      token,
            value:      value,
        };
    }
}

impl ToString for BooleanLiteral {
    fn to_string(&self) -> String {
        return self.token.literal.clone();
    }
}

impl Node for BooleanLiteral {
    fn token_literal(&self) -> Option<String> {
        return Some(self.token.literal.clone());
    }
}

impl Expression for BooleanLiteral {}

#[cfg(test)]
mod test {
    use super::*;
    use token::TokenType;

    #[test]
    fn test_to_string() {
        let bl: BooleanLiteral = BooleanLiteral::new(
            Token::new(TokenType::LIT_BOOL, "true".to_string(), 1, 1),
            true,
        );
        assert_eq!("true".to_string(), bl.to_string());
        let bl: BooleanLiteral = BooleanLiteral::new(
            Token::new(TokenType::LIT_BOOL, "false".to_string(), 1, 1),
            false,
        );
        assert_eq!("false".to_string(), bl.to_string());
    }
}
