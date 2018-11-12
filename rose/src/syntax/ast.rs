use std::fmt::Debug;

use syntax::token::Token;
use syntax::emitter::Emission;
use syntax::emitter::Emitter;

pub trait Node: Debug + ToString {
    fn emit_with(&self, emitter: &Emitter) -> Option<Box<dyn Emission>>;
}

pub trait Expression: Node {}

#[derive(Debug)]
pub struct PrefixExpression {
    pub token:      Token,
    pub right:      Option<Box<dyn Expression>>,
}

impl ToString for PrefixExpression {
    fn to_string(&self) -> String {
        let mut buffer = String::new();
        buffer.push('(');
        buffer.push_str(&self.token.literal);
        match self.right {
            Some(ref exp) => {
                buffer.push(' ');
                buffer.push_str(&exp.to_string());
            },
            None => (),
        }
        buffer.push(')');
        buffer
    }
}

impl Node for PrefixExpression {
    fn emit_with(&self, emitter: &Emitter) -> Option<Box<dyn Emission>> {
        match self.right {
            Some(ref right_exp) => match right_exp.emit_with(emitter) {
                Some(ref right) => emitter.emit_prefix(&self.token.ttype, right),
                None => None,
            }
            None => None,
        }
    }
}

impl Expression for PrefixExpression {}

#[derive(Debug)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i32,
}

impl ToString for IntegerLiteral {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

impl Node for IntegerLiteral {
    fn emit_with(&self, emitter: &Emitter) -> Option<Box<dyn Emission>> {
        emitter.emit_i32(self.value)
    }
}

impl Expression for IntegerLiteral {}

#[derive(Debug)]
pub struct InfixExpression {
    pub left:       Option<Box<dyn Expression>>,
    pub token:      Token,
    pub right:      Option<Box<dyn Expression>>,
}

impl ToString for InfixExpression {
    fn to_string(&self) -> String {
        let mut buffer = String::new();
        buffer.push('(');
        match self.left {
            Some(ref exp) => {
                buffer.push_str(&exp.to_string());
                buffer.push(' ');
            }
            None => (),
        }
        buffer.push_str(&self.token.literal);
        match self.right {
            Some(ref exp) => {
                buffer.push(' ');
                buffer.push_str(&exp.to_string());
            },
            None => (),
        }
        buffer.push(')');
        buffer
    }
}

impl Node for InfixExpression {
    fn emit_with(&self, emitter: &Emitter) -> Option<Box<dyn Emission>> {
        match (&self.left, &self.right) {
            (Some(ref left_exp), Some(ref right_exp)) => {
                match (left_exp.emit_with(emitter), right_exp.emit_with(emitter)) {
                    (Some(ref left), Some(ref right)) => emitter.emit_infix(left,
                        &self.token.ttype,
                        right),
                    (_, _) => None,
                }
            },
            (_, _) => None,
        }
    }
}

impl Expression for InfixExpression {}

#[cfg(test)]
mod tests {
    use super::*;
    use syntax::token::TokenType;

    #[test]
    fn test_to_string_impls() {
        assert_eq!(IntegerLiteral{
            token: Token{
                ttype: TokenType::LitInt,
                literal: "5".to_string(),
                line_num: 1,
                col_num: 1,
            },
            value: 5,
        }.to_string(), "5");
        assert_eq!(PrefixExpression{
            token: Token{
                ttype: TokenType::OpAdd,
                literal: '+'.to_string(),
                line_num: 1,
                col_num: 1,
            },
            right: Some(Box::new(IntegerLiteral{
                token: Token{
                    ttype: TokenType::LitInt,
                    literal: "5".to_string(),
                    line_num: 1,
                    col_num: 2,
                },
                value: 5,
            })),
        }.to_string(), "(+ 5)");
        assert_eq!(InfixExpression{
            left: Some(Box::new(IntegerLiteral{
                token: Token{
                    ttype: TokenType::LitInt,
                    literal: "5".to_string(),
                    line_num: 1,
                    col_num: 1,
                },
                value: 5,
            })),
            token: Token{
                ttype: TokenType::OpAdd,
                literal: '+'.to_string(),
                line_num: 1,
                col_num: 3,
            },
            right: Some(Box::new(IntegerLiteral{
                token: Token{
                    ttype: TokenType::LitInt,
                    literal: "10".to_string(),
                    line_num: 1,
                    col_num: 5,
                },
                value: 10,
            }))
        }.to_string(), "(5 + 10)");
    }
}
