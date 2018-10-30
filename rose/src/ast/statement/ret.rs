use std::any::Any;

use token::Token;
use ast::NodeType;
use ast::Node;
use ast::Statement;
use ast::Expression;

#[derive(Debug)]
pub struct ReturnStatement {
    pub token:          Token,
    pub value:          Option<Box<dyn Expression>>,
}

impl ReturnStatement {
    pub fn new(token: Token, value: Option<Box<dyn Expression>>) -> ReturnStatement {
        return ReturnStatement{
            token:          token,
            value:          value,
        };
    }
}

impl ToString for ReturnStatement {
    fn to_string(&self) -> String {
        let mut builder: String = String::new();
        builder.push_str(&self.token.literal.clone());
        match self.value {
            Some(ref value) => {
                builder.push(' ');
                builder.push_str(&(*value).to_string());
            },
            None => (),
        }
        builder.push(';');
        return builder;
    }
}

impl Node for ReturnStatement {
    fn node_type(&self) -> NodeType {
        return NodeType::RETURN;
    }

    fn token_literal(&self) -> Option<String> {
        return Some(self.token.literal.clone());
    }

    fn as_any(&self) -> &Any {
        return self;
    }
}

impl Statement for ReturnStatement {}

#[cfg(test)]
mod tests {
    use super::*;
    use token::TokenType;
    use ast::literal::ident::Identifier;

    #[test]
    fn test_to_string() {
        let rs = ReturnStatement::new(
            Token::new(TokenType::RW_RETURN, "return".to_string(), 1, 1),
            Some(Box::new(Identifier::new(Token::new(TokenType::LIT_IDENT, "foo".to_string(), 1, 8))))
        );
        assert_eq!("return foo;", rs.to_string());
    }
}
