use ast::Node;
use ast::Expression;
use token::Token;

#[derive(Debug, PartialEq, Eq)]
pub struct Nil {
    pub token:      Token,
}

impl Nil {
    pub fn new(token: Token) -> Nil {
        return Nil{
            token:      token,
        };
    }
}

impl ToString for Nil {
    fn to_string(&self) -> String {
        return self.token.literal.clone();
    }
}

impl Node for Nil {
    fn token_literal(&self) -> Option<String> {
        return Some(self.token.literal.clone());
    }
}

impl Expression for Nil {}

#[cfg(test)]
mod test {
    use super::*;
    use token::TokenType;

    #[test]
    fn test_to_string() {
        let nl: Nil = Nil::new(Token::new(TokenType::LIT_NIL, "nil".to_string(), 1, 1));
        assert_eq!("nil".to_string(), nl.to_string());
    }
}
