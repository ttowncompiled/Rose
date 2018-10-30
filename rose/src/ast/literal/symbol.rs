use std::any::Any;

use ast::NodeType;
use ast::Node;
use ast::Expression;
use token::Token;

#[derive(Debug, PartialEq, Eq)]
pub struct Symbol {
    pub token:      Token,
    pub value:      String,
}

impl Symbol {
    pub fn new(token: Token) -> Symbol {
        let value: String = token.literal.clone();
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
    fn node_type(&self) -> NodeType {
        return NodeType::SYMBOL;
    }

    fn token_literal(&self) -> Option<String> {
        return Some(self.token.literal.clone());
    }

    fn as_any(&self) -> &Any {
        return self;
    }
}

impl Expression for Symbol {}

#[cfg(test)]
mod test {
    use super::*;
    use token::TokenType;

    #[test]
    fn test_to_string() {
        let sym: Symbol = Symbol::new(Token::new(TokenType::LIT_SYMBOL, ":foo".to_string(), 1, 1));
        assert_eq!(":foo", sym.to_string());
    }
}
