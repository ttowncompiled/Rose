use ast::NodeType;
use ast::Node;
use ast::Statement;
use ast::Expression;
use ast::Program;
use ast::statement::expression::ExpressionStatement;
use ast::literal::ident::Identifier;
use ast::literal::blank::Blank;
use ast::literal::integer::IntegerLiteral;
use ast::literal::float::FloatLiteral;
use ast::literal::boolean::BooleanLiteral;
use ast::literal::inf::Inf;
use ast::literal::nan::NaN;
use ast::literal::nil::Nil;
use ast::literal::symbol::Symbol;
use object;

pub trait Evaluator {
    fn eval(&self, node: Box<dyn Node>) -> Option<Box<dyn object::Object>>;
}

pub struct RoseEvaluator {}

impl RoseEvaluator {
    pub fn new() -> RoseEvaluator {
        return RoseEvaluator{};
    }
}

impl RoseEvaluator {
    fn eval_statements(&self, statements: &Vec<Box<dyn Statement>>) -> Option<Box<dyn object::Object>> {
        return None;
    }

    fn eval_expression(&self, expression: &Box<dyn Expression>) -> Option<Box<dyn object::Object>> {
        return None;
    }
}

impl Evaluator for RoseEvaluator {
    fn eval(&self, node: Box<dyn Node>) -> Option<Box<dyn object::Object>> {
        match node.node_type() {
            NodeType::PROGRAM       => {
                if let Some(program) = node.as_any().downcast_ref::<Program>() {
                    return self.eval_statements(&program.statements);
                } else {
                    return None;
                }
            },
            NodeType::LET           => {
                return None;
            },
            NodeType::EXPRESSION    => {
                if let Some(expression) = node.as_any().downcast_ref::<ExpressionStatement>() {
                    match expression.value {
                        Some(ref value) => return self.eval_expression(&value),
                        None => return None,
                    }
                } else {
                    return None;
                }
            },
            NodeType::RETURN        => {
                return None;
            },
            NodeType::PREFIX        => {
                return None;
            },
            NodeType::INFIX         => {
                return None;
            },
            NodeType::IDENT         => {
                return None;
            },
            NodeType::BLANK         => {
                if let Some(_) = node.as_any().downcast_ref::<Blank>() {
                    return Some(Box::new(object::Blank::new()));
                } else {
                    return None;
                }
            },
            NodeType::INTEGER       => {
                if let Some(integer_literal) = node.as_any().downcast_ref::<IntegerLiteral>() {
                    return Some(Box::new(object::Integer::new(integer_literal.value)));
                } else {
                    return None;
                }
            },
            NodeType::FLOAT         => {
                if let Some(float_literal) = node.as_any().downcast_ref::<FloatLiteral>() {
                    return Some(Box::new(object::Float::new(float_literal.value)));
                } else {
                    return None;
                }
            },
            NodeType::BOOLEAN       => {
                if let Some(boolean_literal) = node.as_any().downcast_ref::<BooleanLiteral>() {
                    return Some(Box::new(object::Boolean::new(boolean_literal.value)));
                } else {
                    return None;
                }
            },
            NodeType::INF           => {
                if let Some(_) = node.as_any().downcast_ref::<Inf>() {
                    return Some(Box::new(object::Inf::new()));
                } else {
                    return None;
                }
            },
            NodeType::NAN           => {
                if let Some(_) = node.as_any().downcast_ref::<NaN>() {
                    return Some(Box::new(object::NaN::new()));
                } else {
                    return None;
                }
            },
            NodeType::NIL           => {
                if let Some(_) = node.as_any().downcast_ref::<Nil>() {
                    return Some(Box::new(object::Nil::new()));
                } else {
                    return None;
                }
            },
            NodeType::SYMBOL        => {
                return None;
            },
            _                       =>  return None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use token::TokenType;
    use token::Token;
    use object::ObjectType;

    #[test]
    fn test_eval_integer_expression() {
        let evaluator: RoseEvaluator = RoseEvaluator::new();
        let node: Box<dyn Node> = Box::new(IntegerLiteral::new(
            Token::new(TokenType::LIT_INT, "5".to_string(), 1, 1),
            5
        ));
        match evaluator.eval(node) {
            Some(obj) => {
                assert_eq!(ObjectType::INTEGER, obj.object_type());
                assert_eq!("5", obj.inspect());
            },
            None => assert!(false, "expect {}, got None", 5)
        }
        let node: Box<dyn Node> = Box::new(IntegerLiteral::new(
            Token::new(TokenType::LIT_INT, "10".to_string(), 1, 1),
            10
        ));
        match evaluator.eval(node) {
            Some(obj) => {
                assert_eq!(ObjectType::INTEGER, obj.object_type());
                assert_eq!("10", obj.inspect());
            },
            None => assert!(false, "expect {}, got None", 10)
        }
    }
}
