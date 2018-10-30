use std::any::Any;
use std::fmt::Debug;

pub mod literal;
pub mod statement;
pub mod expression;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq)]
pub enum NodeType {
    PROGRAM,
    LET,
    EXPRESSION,
    RETURN,
    BLANK,
    BOOLEAN,
    FLOAT,
    IDENT,
    INF,
    INTEGER,
    NAN,
    NIL,
    SYMBOL,
    PREFIX,
    INFIX,
}

pub trait Node: Debug + ToString {
    fn node_type(&self) -> NodeType;
    fn token_literal(&self) -> Option<String>;
    fn as_any(&self) -> &Any;
}

pub trait Statement: Node {}
pub trait Expression: Node {}

#[derive(Debug)]
pub struct Program {
    pub statements:     Vec<Box<dyn Statement>>,
}

impl Program {
    pub fn new() -> Program {
        return Program{
            statements: Vec::new(),
        };
    }
}

impl ToString for Program {
    fn to_string(&self) -> String {
        let mut builder: String = String::new();
        for stmt in self.statements.iter() {
            builder.push_str(&(**stmt).to_string());
        }
        return builder;
    }
}

impl Node for Program {
    fn node_type(&self) -> NodeType {
        return NodeType::PROGRAM;
    }

    fn token_literal(&self) -> Option<String> {
        if self.statements.len() > 0 {
            return self.statements[0].token_literal();
        } else {
            return None;
        }
    }

    fn as_any(&self) -> &Any {
        return self;
    }
}
