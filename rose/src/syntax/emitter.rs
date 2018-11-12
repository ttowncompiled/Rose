use syntax::token::TokenType;
use syntax::ast::Expression;

pub trait Emission {}

pub trait Emitter {
    fn emit_i32(&self, val: i32) -> Option<Box<dyn Emission>>;
    fn emit_prefix(&self, op: &TokenType, right: &Box<Expression>) -> Option<Box<dyn Emission>>;
    fn emit_infix(&self,
        left: &Box<dyn Expression>,
        op: &TokenType,
        right: &Box<dyn Expression>) -> Option<Box<dyn Emission>>;
}
