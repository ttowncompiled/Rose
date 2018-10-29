use ast::Node;
use ast::Expression;
use token::Token;

#[derive(Debug, PartialEq, Eq)]
pub struct Identifier {
    pub token:      Token,
    pub value:      String,
}

impl Identifier {
    pub fn new(token: Token) -> Identifier {
        let value: String = token.literal.clone();
        return Identifier{
            token:      token,
            value:      value,
        };
    }
}

impl ToString for Identifier {
    fn to_string(&self) -> String {
        return self.value.clone();
    }
}

impl Node for Identifier {
    fn token_literal(&self) -> Option<String> {
        return Some(self.token.literal.clone());
    }
}

impl Expression for Identifier {}

#[cfg(test)]
mod test {
    use super::*;
    use token::TokenType;

    #[test]
    fn test_to_string() {
        let ident: Identifier = Identifier::new(Token::new(TokenType::LIT_IDENT, "foo".to_string(), 1, 1));
        assert_eq!("foo", ident.to_string());
    }
}
