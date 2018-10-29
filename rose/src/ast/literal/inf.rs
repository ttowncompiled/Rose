use ast::Node;
use ast::Expression;
use token::Token;

#[derive(Debug, PartialEq, Eq)]
pub struct Inf {
    pub token:      Token,
}

impl Inf {
    pub fn new(token: Token) -> Inf {
        return Inf{
            token:      token,
        };
    }
}

impl ToString for Inf {
    fn to_string(&self) -> String {
        return self.token.literal.clone();
    }
}

impl Node for Inf {
    fn token_literal(&self) -> Option<String> {
        return Some(self.token.literal.clone());
    }
}

impl Expression for Inf {}

#[cfg(test)]
mod test {
    use super::*;
    use token::TokenType;

    #[test]
    fn test_to_string() {
        let inf: Inf = Inf::new(Token::new(TokenType::LIT_INF, "Inf".to_string(), 1, 1));
        assert_eq!("Inf", inf.to_string());
    }
}
