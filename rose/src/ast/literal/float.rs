use std::any::Any;

use ast::NodeType;
use ast::Node;
use ast::Expression;
use token::Token;

#[derive(Debug, PartialEq)]
pub struct FloatLiteral {
    pub token:      Token,
    pub value:      f64,
}

impl FloatLiteral {
    pub fn new(token: Token, value: f64) -> FloatLiteral {
        return FloatLiteral{
            token:      token,
            value:      value,
        };
    }
}

impl ToString for FloatLiteral {
    fn to_string(&self) -> String {
        return self.token.literal.clone();
    }
}

impl Node for FloatLiteral {
    fn node_type(&self) -> NodeType {
        return NodeType::FLOAT;
    }

    fn token_literal(&self) -> Option<String> {
        return Some(self.token.literal.clone());
    }

    fn as_any(&self) -> &Any {
        return self;
    }
}

impl Expression for FloatLiteral {}

#[cfg(test)]
mod test {
    use super::*;
    use token::TokenType;

    #[test]
    fn test_to_string() {
        let fl: FloatLiteral = FloatLiteral::new(
            Token::new(TokenType::LIT_FLOAT, "5.5".to_string(), 1, 1),
            5.5,
        );
        assert_eq!("5.5", fl.to_string());
    }
}
