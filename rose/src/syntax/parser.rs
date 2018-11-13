use syntax::token::*;
use syntax::lexer::*;
use syntax::ast::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Precedence {
    Lowest,
    Sum,
    Prefix,
}

pub struct Parser<'a> {
    lexer:          Lexer<'a>,
    curr_token:     Token,
    peek_token:     Token,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Parser {
        let mut lexer = Lexer::new(input);
        let curr_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Parser{
            lexer: lexer,
            curr_token: curr_token,
            peek_token: peek_token,
        }
    }

    pub fn parse_expression(&mut self) -> Option<Box<dyn Expression>> {
        self.parse_expression_with(Precedence::Lowest)
    }

    fn parse_expression_with(&mut self, p: Precedence) -> Option<Box<dyn Expression>> {
        let prefix = self.do_prefix_parse_fn();
        if prefix.is_none() {
            return None;
        }
        let mut left = prefix;
        while ! self.peek_token_is(TokenType::DelEnd) && p < self.peek_precedence() {
            if self.peek_has_infix_parse_fn() {
                self.next_token();
                left = self.parse_infix_expression(left);
            } else {
                return left;
            }
        }
        left
    }

    fn do_prefix_parse_fn(&mut self) -> Option<Box<dyn Expression>> {
        match &self.curr_token.ttype {
            TokenType::OpAdd => self.parse_prefix_expression(),
            TokenType::LitInt => self.parse_integer_literal(),
            _ => None,
        }
    }

    fn parse_prefix_expression(&mut self) -> Option<Box<dyn Expression>> {
        let token = self.curr_token.clone();
        self.next_token();
        let right = self.parse_expression_with(Precedence::Prefix);
        Some(Box::new(PrefixExpression{
            op: token,
            right: right,
        }))
    }

    fn parse_integer_literal(&mut self) -> Option<Box<dyn Expression>> {
        match self.curr_token.literal.parse::<i32>() {
            Ok(val) => Some(Box::new(IntegerLiteral{
                token: self.curr_token.clone(),
                value: val,
            })),
            Err(_) => None,
        }
    }

    fn peek_has_infix_parse_fn(&self) -> bool {
        match self.peek_token.ttype {
            TokenType::OpAdd => true,
            _ => false,
        }
    }

    fn parse_infix_expression(&mut self, left: Option<Box<dyn Expression>>) -> Option<Box<dyn Expression>> {
        let token = self.curr_token.clone();
        let p = self.curr_precedence();
        self.next_token();
        let right = self.parse_expression_with(p);
        Some(Box::new(InfixExpression{
            left: left,
            op: token,
            right: right
        }))
    }

    fn next_token(&mut self) {
        self.curr_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn precedence_of(ttype: &TokenType) -> Precedence {
        match ttype {
            TokenType::OpAdd => Precedence::Sum,
            _ => Precedence::Lowest,
        }
    }

    fn peek_token_is(&self, ttype: TokenType) -> bool {
        self.peek_token.ttype == ttype
    }

    fn curr_precedence(&self) -> Precedence {
        Self::precedence_of(&self.curr_token.ttype)
    }

    fn peek_precedence(&self) -> Precedence {
        Self::precedence_of(&self.peek_token.ttype)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_expression() {
        test_parse_expression_with("5", TokenType::LitInt);
        test_parse_expression_with("+5", TokenType::OpAdd);
        test_parse_expression_with("5 + 5", TokenType::OpAdd);
    }

    fn test_parse_expression_with<'a>(input: &'a str, ttype: TokenType) {
        let mut p = Parser::new(input);
        match p.parse_expression() {
            Some(ref exp) => assert_eq!(exp.ttype(), &ttype),
            None => assert!(false),
        }
    }
}