use token::Token;
use ast::Node;
use ast::Statement;
use ast::Expression;

#[derive(Debug)]
pub struct ExpressionStatement {
    pub token:          Token,
    pub value:          Option<Box<dyn Expression>>,
}

impl ExpressionStatement {
    pub fn new(token: Token, value: Option<Box<dyn Expression>>) -> ExpressionStatement {
        return ExpressionStatement{
            token:          token,
            value:          value,
        };
    }
}

impl ToString for ExpressionStatement {
    fn to_string(&self) -> String {
        return match self.value {
            Some(ref value) => {
                let mut builder: String = (*value).to_string();
                builder.push(';');
                builder
            }
            None => "".to_string(),
        };
    }
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> Option<String> {
        return Some(self.token.literal.clone());
    }
}

impl Statement for ExpressionStatement {}

#[cfg(test)]
mod tests {
    use super::*;
    use token::TokenType;
    use ast::literal::ident::Identifier;

    #[test]
    fn test_to_string() {
        let es: ExpressionStatement = ExpressionStatement::new(
            Token::new(TokenType::LIT_IDENT, "foo".to_string(), 1, 1),
            Some(Box::new(Identifier::new(Token::new(TokenType::LIT_IDENT, "foo".to_string(), 1, 1))))
        );
        assert_eq!("foo;", es.to_string());
    }
}
