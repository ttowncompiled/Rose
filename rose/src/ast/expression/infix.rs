use token::Token;
use ast::Node;
use ast::Expression;

#[derive(Debug)]
pub struct InfixExpression {
    pub token:          Token,
    pub left:           Option<Box<dyn Expression>>,
    pub operator:       String,
    pub right:          Option<Box<dyn Expression>>,
}

impl InfixExpression {
    pub fn new(token: Token, left: Option<Box<dyn Expression>>, operator: String, right: Option<Box<dyn Expression>>) -> InfixExpression {
        return InfixExpression{
            token:          token,
            left:           left,
            operator:       operator,
            right:          right,
        };
    }
}

impl ToString for InfixExpression {
    fn to_string(&self) -> String {
        let mut builder: String = String::new();
        builder.push('(');
        match self.left {
            Some(ref left) => {
                builder.push_str(&(*left).to_string());
                builder.push(' ');
            },
            None => (),
        }
        builder.push_str(&self.operator);
        builder.push(' ');
        match self.right {
            Some(ref right) => builder.push_str(&(*right).to_string()),
            None => (),
        }
        builder.push(')');
        return builder;
    }
}

impl Node for InfixExpression {
    fn token_literal(&self) -> Option<String> {
        return Some(self.token.literal.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use token::TokenType;
    use ast::literal::ident::Identifier;

    #[test]
    fn test_to_string() {
        let ie: InfixExpression = InfixExpression::new(
            Token::new(TokenType::LIT_IDENT, "foo".to_string(), 1, 1),
            Some(Box::new(Identifier::new(
                Token::new(TokenType::LIT_IDENT, "foo".to_string(), 1, 1),
                "foo".to_string()
            ))),
            "+".to_string(),
            Some(Box::new(Identifier::new(
                Token::new(TokenType::LIT_IDENT, "bar".to_string(), 1, 7),
                "bar".to_string()
            )))
        );
        assert_eq!("(foo + bar)", ie.to_string());
    }
}
