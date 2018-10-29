use ast::Node;
use ast::Expression;
use token::Token;

#[derive(Debug, PartialEq, Eq)]
pub struct Symbol {
    pub token:      Token,
    pub value:      String,
}

impl Symbol {
    pub fn new(token: Token, value: String) -> Symbol {
        return Symbol{
            token:      token,
            value:      value,
        };
    }
}

impl ToString for Symbol {
    fn to_string(&self) -> String {
        return self.value.clone();
    }
}

impl Node for Symbol {
    fn token_literal(&self) -> Option<String> {
        return Some(self.token.literal.clone());
    }
}

impl Expression for Symbol {}

#[cfg(test)]
mod test {
    use super::*;
    use token::TokenType;

    #[test]
    fn test_to_string() {
        let sym: Symbol = Symbol::new(
            Token::new(TokenType::LIT_SYMBOL, ":foo".to_string(), 1, 1),
            ":foo".to_string(),
        );
        assert_eq!(":foo".to_string(), sym.to_string());
    }
}
