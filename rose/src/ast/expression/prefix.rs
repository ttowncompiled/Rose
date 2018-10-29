use token::Token;
use ast::Node;
use ast::Expression;

#[derive(Debug)]
pub struct PrefixExpression {
    pub token:          Token,
    pub operator:       String,
    pub value:          Option<Box<dyn Expression>>,
}

impl PrefixExpression {
    pub fn new(token: Token, value: Option<Box<dyn Expression>>) -> PrefixExpression {
        let operator: String = token.literal.clone();
        return PrefixExpression{
            token:          token,
            operator:       operator,
            value:          value,
        };
    }
}

impl ToString for PrefixExpression {
    fn to_string(&self) -> String {
        let mut builder: String = String::new();
        builder.push('(');
        builder.push_str(&self.operator);
        match self.value {
            Some(ref value) => builder.push_str(&(*value).to_string()),
            None => (),
        }
        builder.push(')');
        return builder;
    }
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> Option<String> {
        return Some(self.token.literal.clone());
    }
}

impl Expression for PrefixExpression {}

#[cfg(test)]
mod tests {
    use super::*;
    use token::TokenType;
    use ast::literal::ident::Identifier;

    #[test]
    fn test_to_string() {
        let pe: PrefixExpression = PrefixExpression::new(
            Token::new(TokenType::OP_ADD, "!".to_string(), 1, 1),
            Some(Box::new(Identifier::new(Token::new(TokenType::LIT_IDENT, "foo".to_string(), 1, 2))))
        );
        assert_eq!("(!foo)", pe.to_string());
    }
}
