use std::any::Any;

use ast::NodeType;
use ast::Node;
use ast::Expression;
use token::Token;

#[derive(Debug, PartialEq, Eq)]
pub struct IntegerLiteral {
    pub token:      Token,
    pub value:      i32,
}

impl IntegerLiteral {
    pub fn new(token: Token, value: i32) -> IntegerLiteral {
        return IntegerLiteral{
            token:      token,
            value:      value,
        };
    }
}

impl ToString for IntegerLiteral {
    fn to_string(&self) -> String {
        return self.token.literal.clone();
    }
}

impl Node for IntegerLiteral {
    fn node_type(&self) -> NodeType {
        return NodeType::INTEGER;
    }

    fn token_literal(&self) -> Option<String> {
        return Some(self.token.literal.clone());
    }

    fn as_any(&self) -> &Any {
        return self;
    }
}

impl Expression for IntegerLiteral {}

#[cfg(test)]
mod test {
    use super::*;
    use token::TokenType;

    #[test]
    fn test_to_string() {
        let il: IntegerLiteral = IntegerLiteral::new(
            Token::new(TokenType::LIT_INT, "5".to_string(), 1, 1),
            5,
        );
        assert_eq!("5", il.to_string());
    }
}
