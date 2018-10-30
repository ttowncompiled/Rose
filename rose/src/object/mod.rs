use std::any::Any;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq)]
pub enum ObjectType {
    BLANK,
    INTEGER,
    FLOAT,
    BOOLEAN,
    INF,
    NAN,
    NIL,
    SYMBOL,
}

pub trait Object {
    fn object_type(&self) -> ObjectType;
    fn inspect(&self) -> String;
    fn as_any(&self) -> &Any;
}

#[derive(Debug, PartialEq, Eq)]
pub struct Blank{}

impl Blank {
    pub fn new() -> Blank {
        return Blank{};
    }
}

impl Object for Blank {
    fn object_type(&self) -> ObjectType {
        return ObjectType::BLANK;
    }

    fn inspect(&self) -> String {
        return '_'.to_string();
    }

    fn as_any(&self) -> &Any {
        return self;
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Integer {
    value:      i32,
}

impl Integer {
    pub fn new(value: i32) -> Integer {
        return Integer{
            value:      value,
        };
    }
}

impl Object for Integer {
    fn object_type(&self) -> ObjectType {
        return ObjectType::INTEGER;
    }

    fn inspect(&self) -> String {
        return self.value.to_string();
    }

    fn as_any(&self) -> &Any {
        return self;
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Float {
    value:      f64,
}

impl Float {
    pub fn new(value: f64) -> Float {
        return Float{
            value:      value,
        };
    }
}

impl Object for Float {
    fn object_type(&self) -> ObjectType {
        return ObjectType::FLOAT;
    }

    fn inspect(&self) -> String {
        return self.value.to_string();
    }

    fn as_any(&self) -> &Any {
        return self;
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Boolean {
    value:      bool,
}

impl Boolean {
    pub fn new(value: bool) -> Boolean {
        return Boolean{
            value:      value,
        };
    }
}

impl Object for Boolean {
    fn object_type(&self) -> ObjectType {
        return ObjectType::BOOLEAN;
    }

    fn inspect(&self) -> String {
        return self.value.to_string();
    }

    fn as_any(&self) -> &Any {
        return self;
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Inf{}

impl Inf {
    pub fn new() -> Inf {
        return Inf{};
    }
}

impl Object for Inf {
    fn object_type(&self) -> ObjectType {
        return ObjectType::INF;
    }

    fn inspect(&self) -> String {
        return "Inf".to_string();
    }

    fn as_any(&self) -> &Any {
        return self;
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct NaN{}

impl NaN {
    pub fn new() -> NaN {
        return NaN{};
    }
}

impl Object for NaN {
    fn object_type(&self) -> ObjectType {
        return ObjectType::NAN;
    }

    fn inspect(&self) -> String {
        return "NaN".to_string();
    }

    fn as_any(&self) -> &Any {
        return self;
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Nil{}

impl Nil {
    pub fn new() -> Nil {
        return Nil{};
    }
}

impl Object for Nil {
    fn object_type(&self) -> ObjectType {
        return ObjectType::NIL;
    }

    fn inspect(&self) -> String {
        return "Nil".to_string();
    }

    fn as_any(&self) -> &Any {
        return self;
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Symbol {
    value:      String,
}

impl Symbol {
    pub fn new(value: String) -> Symbol {
        return Symbol{
            value:      value,
        };
    }
}

impl Object for Symbol {
    fn object_type(&self) -> ObjectType {
        return ObjectType::SYMBOL;
    }

    fn inspect(&self) -> String {
        return self.value.clone();
    }

    fn as_any(&self) -> &Any {
        return self;
    }
}
