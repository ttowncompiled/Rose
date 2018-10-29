use ast::Node;
use ast::Expression;
use token::Token;

#[derive(Debug, PartialEq, Eq)]
pub struct NaN {
    pub token:      Token,
}

impl NaN {
    pub fn new(token: Token) -> NaN {
        return NaN{
            token:      token,
        };
    }
}

impl ToString for NaN {
    fn to_string(&self) -> String {
        return self.token.literal.clone();
    }
}

impl Node for NaN {
    fn token_literal(&self) -> Option<String> {
        return Some(self.token.literal.clone());
    }
}

impl Expression for NaN {}

#[cfg(test)]
mod test {
    use super::*;
    use token::TokenType;

    #[test]
    fn test_to_string() {
        let nan: NaN = NaN::new(Token::new(TokenType::LIT_NAN, "NaN".to_string(), 1, 1));
        assert_eq!("NaN".to_string(), nan.to_string());
    }
}
