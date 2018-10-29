use std::fmt::Debug;

pub mod literal;
pub mod statement;
pub mod expression;

pub trait Node: Debug + ToString {
    fn token_literal(&self) -> Option<String>;
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
    fn token_literal(&self) -> Option<String> {
        if self.statements.len() > 0 {
            return self.statements[0].token_literal();
        } else {
            return None;
        }
    }
}
