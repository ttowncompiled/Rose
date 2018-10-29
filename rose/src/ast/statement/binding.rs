use token::Token;
use ast::literal::ident::Identifier;
use ast::Node;
use ast::Statement;
use ast::Expression;

#[derive(Debug)]
pub struct LetStatement {
    pub token:      Token,
    pub battr:      Option<Token>,
    pub name:       Identifier,
    pub btype:      Option<Identifier>,
    pub value:      Option<Box<dyn Expression>>,
}

impl LetStatement {
    pub fn new(token: Token, battr: Option<Token>, name: Identifier, btype: Option<Identifier>, value: Option<Box<dyn Expression>>) -> LetStatement {
        return LetStatement{
            token:      token,
            battr:      battr,
            name:       name,
            btype:      btype,
            value:      value,
        };
    }
}

impl ToString for LetStatement {
    fn to_string(&self) -> String {
        let mut builder: String = String::new();
        builder.push_str(&self.token.literal.clone());
        builder.push(' ');
        match self.battr {
            Some(ref token) => {
                builder.push_str(&token.literal.clone());
                builder.push(' ');
            },
            None => (),
        }
        builder.push_str(&self.name.to_string());
        match self.btype {
            Some(ref ident) => {
                builder.push(':'); builder.push(' ');
                builder.push_str(&ident.to_string());
            },
            None => (),
        }
        match self.value {
            Some(ref value) => {
                builder.push(' '); builder.push('='); builder.push(' ');
                builder.push_str(&(*value).to_string());
            },
            None => (),
        }
        builder.push(';');
        return builder;
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> Option<String> {
        return Some(self.token.literal.clone());
    }
}

impl Statement for LetStatement {}

#[cfg(test)]
mod tests {
    use super::*;
    use token::TokenType;
    use ast::literal::integer::IntegerLiteral;

    #[test]
    fn test_to_string() {
        let ls: LetStatement = LetStatement::new(
            Token::new(TokenType::RW_LET, "let".to_string(), 1, 1),
            Some(Token::new(TokenType::RW_MUT, "mut".to_string(), 1, 5)),
            Identifier::new(Token::new(TokenType::LIT_IDENT, "foo".to_string(), 1, 9)),
            Some(Identifier::new(Token::new(TokenType::LIT_IDENT, "Int".to_string(), 1, 14))),
            Some(Box::new(IntegerLiteral::new(
                Token::new(TokenType::LIT_INT, "5".to_string(), 1, 20),
                5
            )))
        );
        assert_eq!("let mut foo: Int = 5;", ls.to_string());
        let ls: LetStatement = LetStatement::new(
            Token::new(TokenType::RW_LET, "let".to_string(), 1, 1),
            None,
            Identifier::new(Token::new(TokenType::LIT_IDENT, "foo".to_string(), 1, 5)),
            Some(Identifier::new(Token::new(TokenType::LIT_IDENT, "Int".to_string(), 1, 10))),
            Some(Box::new(IntegerLiteral::new(
                Token::new(TokenType::LIT_INT, "5".to_string(), 1, 16),
                5
            )))
        );
        assert_eq!("let foo: Int = 5;", ls.to_string());
        let ls: LetStatement = LetStatement::new(
            Token::new(TokenType::RW_LET, "let".to_string(), 1, 1),
            None,
            Identifier::new(Token::new(TokenType::LIT_IDENT, "foo".to_string(), 1, 5)),
            None,
            Some(Box::new(IntegerLiteral::new(
                Token::new(TokenType::LIT_INT, "5".to_string(), 1, 11),
                5
            )))
        );
        assert_eq!("let foo = 5;", ls.to_string());
        let ls: LetStatement = LetStatement::new(
            Token::new(TokenType::RW_LET, "let".to_string(), 1, 1),
            None,
            Identifier::new(Token::new(TokenType::LIT_IDENT, "foo".to_string(), 1, 5)),
            None,
            None
        );
        assert_eq!("let foo;", ls.to_string());
    }
}
