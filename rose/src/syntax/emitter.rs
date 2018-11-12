use syntax::token::TokenType;

pub trait Emission {}

pub trait Emitter {
    fn emit_i32(&self, val: i32) -> Option<Box<dyn Emission>>;
    fn emit_prefix(&self, op: &TokenType, right: &Box<dyn Emission>) -> Option<Box<dyn Emission>>;
    fn emit_infix(&self,
        left: &Box<dyn Emission>,
        op: &TokenType,
        right: &Box<dyn Emission>) -> Option<Box<dyn Emission>>;
}
