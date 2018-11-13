use std::fmt::Debug;

use syntax::token::*;
use syntax::emitter::*;

pub trait Node: Debug + ToString {
    fn ttype(&self) -> &TokenType;
    fn emit_with(&self, emitter: &Emitter) -> Option<Box<dyn Emission>>;
}

pub trait Statement: Node {}

#[derive(Debug)]
pub struct ClassStatement {
    pub exposure:       Token,
    pub marker:         Token,
    pub ident:          Token,
    pub defs:           Vec<DefStatement>,
    pub depth:          i8,
    pub offset:         i8,
    pub end:            Token,
}

impl ToString for ClassStatement {
    fn to_string(&self) -> String {
        let mut buffer = String::new();
        buffer.push_str(&self.exposure.raw);
        buffer.push(' ');
        buffer.push_str(&self.marker.raw);
        buffer.push(' ');
        buffer.push_str(&self.ident.raw);
        if ! self.defs.is_empty() {
            buffer.push('\n');
            for df in self.defs.iter() {
                for _ in 0..((self.depth+1)*self.offset) {
                    buffer.push(' ');
                }
                buffer.push_str(&df.to_string());
                buffer.push('\n');
            }
            for _ in 0..(self.depth*self.offset) {
                buffer.push(' ');
            }
        } else {
            buffer.push(' ');
        }
        buffer.push_str(&self.end.raw);
        buffer
    }
}

impl Node for ClassStatement {
    fn ttype(&self) -> &TokenType {
        &self.marker.ttype
    }

    fn emit_with(&self, _emitter: &Emitter) -> Option<Box<dyn Emission>> {
        None
    }
}

impl Statement for ClassStatement {}

#[derive(Debug)]
pub struct DefStatement {
    pub exposure:       Token,
    pub marker:         Token,
    pub ident:          Token,
    pub params:         ParamsList,
    pub delim:          Token,
    pub dtype:          Token,
    pub body:           Vec<Box<dyn Statement>>,
    pub depth:          i8,
    pub offset:         i8,
    pub end:            Token,
}

impl ToString for DefStatement {
    fn to_string(&self) -> String {
        let mut buffer = String::new();
        buffer.push_str(&self.exposure.raw);
        buffer.push(' ');
        buffer.push_str(&self.marker.raw);
        buffer.push(' ');
        buffer.push_str(&self.ident.raw);
        buffer.push_str(&self.params.to_string());
        buffer.push(' ');
        buffer.push_str(&self.delim.raw);
        buffer.push(' ');
        buffer.push_str(&self.dtype.raw);
        if ! self.body.is_empty() {
            buffer.push('\n');
            for stmt in self.body.iter() {
                for _ in 0..((self.depth+1)*self.offset) {
                    buffer.push(' ');
                }
                buffer.push_str(&stmt.to_string());
                buffer.push('\n');
            }
            for _ in 0..(self.depth*self.offset) {
                buffer.push(' ');
            }
        } else {
            buffer.push(' ');
        }
        buffer.push_str(&self.end.raw);
        buffer
    }
}

impl Node for DefStatement {
    fn ttype(&self) -> &TokenType {
        &self.marker.ttype
    }

    fn emit_with(&self, _emitter: &Emitter) -> Option<Box<dyn Emission>> {
        None
    }
}

impl Statement for DefStatement {}

#[derive(Debug)]
pub struct ParamsList {
    pub open:       Token,
    pub params:     Vec<Param>,
    pub delims:     Vec<Token>,
    pub close:      Token,
}

impl ToString for ParamsList {
    fn to_string(&self) -> String {
        let mut buffer = String::new();
        buffer.push_str(&self.open.raw);
        if ! self.params.is_empty() {
            let mut iter = self.params.iter();
            buffer.push(' ');
            match iter.next() {
                Some(param) => buffer.push_str(&param.to_string()),
                None => (),
            }
            for delim in self.delims.iter() {
                buffer.push_str(&delim.raw);
                buffer.push(' ');
                match iter.next() {
                    Some(param) => buffer.push_str(&param.to_string()),
                    None => (),
                }
            }
            buffer.push(' ');
        }
        buffer.push_str(&self.close.raw);
        buffer
    }
}

impl Node for ParamsList {
    fn ttype(&self) -> &TokenType {
        &self.open.ttype
    }

    fn emit_with(&self, _emitter: &Emitter) -> Option<Box<dyn Emission>> {
        None
    }
}

#[derive(Debug)]
pub struct Param {
    pub ident:      Token,
    pub delim:      Token,
    pub ptype:      Token,
}

impl ToString for Param {
    fn to_string(&self) -> String {
        let mut buffer = String::new();
        buffer.push_str(&self.ident.raw);
        buffer.push(' ');
        buffer.push_str(&self.delim.raw);
        buffer.push(' ');
        buffer.push_str(&self.ptype.raw);
        buffer
    }
}

impl Node for Param {
    fn ttype(&self) -> &TokenType {
        &self.ident.ttype
    }

    fn emit_with(&self, _emitter: &Emitter) -> Option<Box<dyn Emission>> {
        None
    }
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub token:      Token,
    pub value:      Option<Box<dyn Expression>>,
}

impl ToString for ExpressionStatement {
    fn to_string(&self) -> String {
        match self.value {
            Some(ref exp) => exp.to_string(),
            None => "".to_string(),
        }
    }
}

impl Node for ExpressionStatement {
    fn ttype(&self) -> &TokenType {
        &self.token.ttype
    }

    fn emit_with(&self, _emitter: &Emitter) -> Option<Box<dyn Emission>> {
        None
    }
}

impl Statement for ExpressionStatement {}

pub trait Expression: Node {}

#[derive(Debug)]
pub struct PrefixExpression {
    pub op:         Token,
    pub right:      Option<Box<dyn Expression>>,
}

impl ToString for PrefixExpression {
    fn to_string(&self) -> String {
        let mut buffer = String::new();
        buffer.push('(');
        buffer.push_str(&self.op.raw);
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
    fn ttype(&self) -> &TokenType {
        &self.op.ttype
    }

    fn emit_with(&self, emitter: &Emitter) -> Option<Box<dyn Emission>> {
        match self.right {
            Some(ref right_exp) => match right_exp.emit_with(emitter) {
                Some(ref right) => emitter.emit_prefix(&self.op.ttype, right),
                None => None,
            }
            None => None,
        }
    }
}

impl Expression for PrefixExpression {}

#[derive(Debug)]
pub struct IntegerLiteral {
    pub token:  Token,
    pub value:  i32,
}

impl ToString for IntegerLiteral {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

impl Node for IntegerLiteral {
    fn ttype(&self) -> &TokenType {
        &self.token.ttype
    }

    fn emit_with(&self, emitter: &Emitter) -> Option<Box<dyn Emission>> {
        emitter.emit_i32(self.value)
    }
}

impl Expression for IntegerLiteral {}

#[derive(Debug)]
pub struct IdentifierLiteral {
    pub token:  Token,
    pub value:  String,
}

impl ToString for IdentifierLiteral {
    fn to_string(&self) -> String {
        self.value.clone()
    }
}

impl Node for IdentifierLiteral {
    fn ttype(&self) -> &TokenType {
        &self.token.ttype
    }

    fn emit_with(&self, _emitter: &Emitter) -> Option<Box<dyn Emission>> {
        None
    }
}

impl Expression for IdentifierLiteral {}

#[derive(Debug)]
pub struct InfixExpression {
    pub left:   Option<Box<dyn Expression>>,
    pub op:     Token,
    pub right:  Option<Box<dyn Expression>>,
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
        buffer.push_str(&self.op.raw);
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
    fn ttype(&self) -> &TokenType {
        &self.op.ttype
    }

    fn emit_with(&self, emitter: &Emitter) -> Option<Box<dyn Emission>> {
        match (&self.left, &self.right) {
            (Some(ref left_exp), Some(ref right_exp)) => {
                match (left_exp.emit_with(emitter), right_exp.emit_with(emitter)) {
                    (Some(ref left), Some(ref right)) => emitter.emit_infix(left,
                        &self.op.ttype,
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
    fn test_class_statement_to_string() {
        assert_eq!(ClassStatement{
            exposure:   Token{
                ttype:      TokenType::RwPub,
                raw:        "pub".to_string(),
                line_num:   1,
                col_num:    1,
            },
            marker:     Token{
                ttype:      TokenType::RwClass,
                raw:        "class".to_string(),
                line_num:   1,
                col_num:    5,
            },
            ident:      Token{
                ttype:      TokenType::LitIdent,
                raw:        "Foo".to_string(),
                line_num:   1,
                col_num:    11,
            },
            defs:       Vec::new(),
            depth:      0,
            offset:     4,
            end:        Token{
                ttype:      TokenType::RwEnd,
                raw:        "end".to_string(),
                line_num:   1,
                col_num:    15,
            },
        }.to_string(), "pub class Foo end");
        assert_eq!(ClassStatement{
            exposure:   Token{
                ttype:      TokenType::RwPub,
                raw:        "pub".to_string(),
                line_num:   1,
                col_num:    1,
            },
            marker:     Token{
                ttype:      TokenType::RwClass,
                raw:        "class".to_string(),
                line_num:   1,
                col_num:    5,
            },
            ident:      Token{
                ttype:      TokenType::LitIdent,
                raw:        "Foo".to_string(),
                line_num:   1,
                col_num:    11,
            },
            defs:       vec![
                DefStatement{
                    exposure:   Token{
                        ttype:      TokenType::RwPub,
                        raw:        "pub".to_string(),
                        line_num:   2,
                        col_num:    5,
                    },
                    marker:     Token{
                        ttype:      TokenType::RwDef,
                        raw:        "def".to_string(),
                        line_num:   2,
                        col_num:    9,
                    },
                    ident:      Token{
                        ttype:      TokenType::LitIdent,
                        raw:        "foo".to_string(),
                        line_num:   2,
                        col_num:    13,
                    },
                    params:     ParamsList{
                        open: Token{
                            ttype:      TokenType::DelOpenParen,
                            raw:        "(".to_string(),
                            line_num:   2,
                            col_num:    16,
                        },
                        params: vec![
                            Param{
                                ident: Token{
                                    ttype:      TokenType::LitIdent,
                                    raw:        "x".to_string(),
                                    line_num:   2,
                                    col_num:    18,
                                },
                                delim: Token{
                                    ttype:      TokenType::DelColon,
                                    raw:        ":".to_string(),
                                    line_num:   2,
                                    col_num:    20,
                                },
                                ptype: Token{
                                    ttype:      TokenType::LitIdent,
                                    raw:        "Foo".to_string(),
                                    line_num:   2,
                                    col_num:    22,
                                },
                            },
                            Param{
                                ident: Token{
                                    ttype:      TokenType::LitIdent,
                                    raw:        "y".to_string(),
                                    line_num:   2,
                                    col_num:    25,
                                },
                                delim: Token{
                                    ttype:      TokenType::DelColon,
                                    raw:        ":".to_string(),
                                    line_num:   2,
                                    col_num:    27,
                                },
                                ptype: Token{
                                    ttype:      TokenType::LitIdent,
                                    raw:        "Foo".to_string(),
                                    line_num:   2,
                                    col_num:    29,
                                },
                            },
                        ],
                        delims: vec![
                            Token{
                                ttype:      TokenType::DelComma,
                                raw:        ",".to_string(),
                                line_num:   2,
                                col_num:    27,
                            },
                        ],
                        close: Token{
                            ttype:      TokenType::DelCloseParen,
                            raw:        ")".to_string(),
                            line_num:   2,
                            col_num:    31,
                        },
                    },
                    delim:      Token{
                        ttype:      TokenType::DelColon,
                        raw:        ":".to_string(),
                        line_num:   2,
                        col_num:    33,
                    },
                    dtype:      Token{
                        ttype:      TokenType::LitIdent,
                        raw:        "Foo".to_string(),
                        line_num:   2,
                        col_num:    35,
                    },
                    body:       vec![
                        Box::new(ExpressionStatement{
                            token:  Token{
                                ttype:      TokenType::LitIdent,
                                raw:        "x".to_string(),
                                line_num:   3,
                                col_num:    9,
                            },
                            value: Some(Box::new(InfixExpression{
                                left: Some(Box::new(IdentifierLiteral{
                                    token: Token{
                                        ttype:      TokenType::LitIdent,
                                        raw:        "x".to_string(),
                                        line_num:   3,
                                        col_num:    9,
                                    },
                                    value: "x".to_string(),
                                })),
                                op: Token{
                                    ttype:      TokenType::OpAdd,
                                    raw:        '+'.to_string(),
                                    line_num:   3,
                                    col_num:    11,
                                },
                                right: Some(Box::new(IdentifierLiteral{
                                    token: Token{
                                        ttype:      TokenType::LitIdent,
                                        raw:        "y".to_string(),
                                        line_num:   3,
                                        col_num:    13,
                                    },
                                    value: "y".to_string(),
                                })),
                            })),
                        }),
                    ],
                    depth:      1,
                    offset:     4,
                    end:        Token{
                        ttype:      TokenType::RwEnd,
                        raw:        "end".to_string(),
                        line_num:   4,
                        col_num:    5,
                    },
                },
            ],
            depth:      0,
            offset:     4,
            end:        Token{
                ttype:      TokenType::RwEnd,
                raw:        "end".to_string(),
                line_num:   5,
                col_num:    1,
            },
        }.to_string(), "pub class Foo
    pub def foo( x : Foo, y : Foo ) : Foo
        (x + y)
    end
end");
    }

    #[test]
    fn test_def_statement_to_string() {
        assert_eq!(DefStatement{
            exposure:   Token{
                ttype:      TokenType::RwPub,
                raw:        "pub".to_string(),
                line_num:   1,
                col_num:    1,
            },
            marker:     Token{
                ttype:      TokenType::RwDef,
                raw:        "def".to_string(),
                line_num:   1,
                col_num:    5,
            },
            ident:      Token{
                ttype:      TokenType::LitIdent,
                raw:        "foo".to_string(),
                line_num:   1,
                col_num:    9,
            },
            params:     ParamsList{
                open: Token{
                    ttype:      TokenType::DelOpenParen,
                    raw:        "(".to_string(),
                    line_num:   1,
                    col_num:    12,
                },
                params: Vec::new(),
                delims: Vec::new(),
                close: Token{
                    ttype:      TokenType::DelCloseParen,
                    raw:        ")".to_string(),
                    line_num:   1,
                    col_num:    13,
                },
            },
            delim:      Token{
                ttype:      TokenType::DelColon,
                raw:        ":".to_string(),
                line_num:   1,
                col_num:    15,
            },
            dtype:      Token{
                ttype:      TokenType::LitIdent,
                raw:        "Foo".to_string(),
                line_num:   1,
                col_num:    17,
            },
            body:       Vec::new(),
            depth:      0,
            offset:     4,
            end:        Token{
                ttype:      TokenType::RwEnd,
                raw:        "end".to_string(),
                line_num:   1,
                col_num:    19,
            },
        }.to_string(), "pub def foo() : Foo end");
        assert_eq!(DefStatement{
            exposure:   Token{
                ttype:      TokenType::RwPub,
                raw:        "pub".to_string(),
                line_num:   1,
                col_num:    1,
            },
            marker:     Token{
                ttype:      TokenType::RwDef,
                raw:        "def".to_string(),
                line_num:   1,
                col_num:    5,
            },
            ident:      Token{
                ttype:      TokenType::LitIdent,
                raw:        "foo".to_string(),
                line_num:   1,
                col_num:    9,
            },
            params:     ParamsList{
                open: Token{
                    ttype:      TokenType::DelOpenParen,
                    raw:        "(".to_string(),
                    line_num:   1,
                    col_num:    12,
                },
                params: Vec::new(),
                delims: Vec::new(),
                close: Token{
                    ttype:      TokenType::DelCloseParen,
                    raw:        ")".to_string(),
                    line_num:   1,
                    col_num:    13,
                },
            },
            delim:      Token{
                ttype:      TokenType::DelColon,
                raw:        ":".to_string(),
                line_num:   1,
                col_num:    15,
            },
            dtype:      Token{
                ttype:      TokenType::LitIdent,
                raw:        "Foo".to_string(),
                line_num:   1,
                col_num:    17,
            },
            body:       vec![
                Box::new(ExpressionStatement{
                    token:  Token{
                        ttype:      TokenType::LitInt,
                        raw:        "5".to_string(),
                        line_num:   2,
                        col_num:    5,
                    },
                    value: Some(Box::new(InfixExpression{
                        left: Some(Box::new(IntegerLiteral{
                            token: Token{
                                ttype:      TokenType::LitInt,
                                raw:        "5".to_string(),
                                line_num:   2,
                                col_num:    5,
                            },
                            value: 5,
                        })),
                        op: Token{
                            ttype:      TokenType::OpAdd,
                            raw:        '+'.to_string(),
                            line_num:   2,
                            col_num:    7,
                        },
                        right: Some(Box::new(IntegerLiteral{
                            token: Token{
                                ttype:      TokenType::LitInt,
                                raw:        "10".to_string(),
                                line_num:   2,
                                col_num:    9,
                            },
                            value: 10,
                        })),
                    })),
                }),
            ],
            depth:      0,
            offset:     4,
            end:        Token{
                ttype:      TokenType::RwEnd,
                raw:        "end".to_string(),
                line_num:   3,
                col_num:    1,
            },
        }.to_string(), "pub def foo() : Foo
    (5 + 10)
end");
        assert_eq!(DefStatement{
            exposure:   Token{
                ttype:      TokenType::RwPub,
                raw:        "pub".to_string(),
                line_num:   1,
                col_num:    1,
            },
            marker:     Token{
                ttype:      TokenType::RwDef,
                raw:        "def".to_string(),
                line_num:   1,
                col_num:    5,
            },
            ident:      Token{
                ttype:      TokenType::LitIdent,
                raw:        "foo".to_string(),
                line_num:   1,
                col_num:    9,
            },
            params:     ParamsList{
                open: Token{
                    ttype:      TokenType::DelOpenParen,
                    raw:        "(".to_string(),
                    line_num:   1,
                    col_num:    12,
                },
                params: vec![
                    Param{
                        ident: Token{
                            ttype:      TokenType::LitIdent,
                            raw:        "x".to_string(),
                            line_num:   1,
                            col_num:    14,
                        },
                        delim: Token{
                            ttype:      TokenType::DelColon,
                            raw:        ":".to_string(),
                            line_num:   1,
                            col_num:    16,
                        },
                        ptype: Token{
                            ttype:      TokenType::LitIdent,
                            raw:        "Foo".to_string(),
                            line_num:   1,
                            col_num:    18,
                        },
                    },
                    Param{
                        ident: Token{
                            ttype:      TokenType::LitIdent,
                            raw:        "y".to_string(),
                            line_num:   1,
                            col_num:    21,
                        },
                        delim: Token{
                            ttype:      TokenType::DelColon,
                            raw:        ":".to_string(),
                            line_num:   1,
                            col_num:    23,
                        },
                        ptype: Token{
                            ttype:      TokenType::LitIdent,
                            raw:        "Foo".to_string(),
                            line_num:   1,
                            col_num:    25,
                        },
                    },
                ],
                delims: vec![
                    Token{
                        ttype:      TokenType::DelComma,
                        raw:        ",".to_string(),
                        line_num:   1,
                        col_num:    19,
                    },
                ],
                close: Token{
                    ttype:      TokenType::DelCloseParen,
                    raw:        ")".to_string(),
                    line_num:   1,
                    col_num:    27,
                },
            },
            delim:      Token{
                ttype:      TokenType::DelColon,
                raw:        ":".to_string(),
                line_num:   1,
                col_num:    29,
            },
            dtype:      Token{
                ttype:      TokenType::LitIdent,
                raw:        "Foo".to_string(),
                line_num:   1,
                col_num:    31,
            },
            body:       vec![
                Box::new(ExpressionStatement{
                    token:  Token{
                        ttype:      TokenType::LitIdent,
                        raw:        "x".to_string(),
                        line_num:   2,
                        col_num:    5,
                    },
                    value: Some(Box::new(InfixExpression{
                        left: Some(Box::new(IdentifierLiteral{
                            token: Token{
                                ttype:      TokenType::LitIdent,
                                raw:        "x".to_string(),
                                line_num:   2,
                                col_num:    5,
                            },
                            value: "x".to_string(),
                        })),
                        op: Token{
                            ttype:      TokenType::OpAdd,
                            raw:        '+'.to_string(),
                            line_num:   2,
                            col_num:    7,
                        },
                        right: Some(Box::new(IdentifierLiteral{
                            token: Token{
                                ttype:      TokenType::LitIdent,
                                raw:        "y".to_string(),
                                line_num:   2,
                                col_num:    9,
                            },
                            value: "y".to_string(),
                        })),
                    })),
                }),
            ],
            depth:      0,
            offset:     4,
            end:        Token{
                ttype:      TokenType::RwEnd,
                raw:        "end".to_string(),
                line_num:   3,
                col_num:    1,
            },
        }.to_string(), "pub def foo( x : Foo, y : Foo ) : Foo
    (x + y)
end");
    }

    #[test]
    fn test_params_list_to_string() {
        assert_eq!(ParamsList{
            open: Token{
                ttype:      TokenType::DelOpenParen,
                raw:        "(".to_string(),
                line_num:   1,
                col_num:    1,
            },
            params: Vec::new(),
            delims: Vec::new(),
            close: Token{
                ttype:      TokenType::DelCloseParen,
                raw:        ")".to_string(),
                line_num:   1,
                col_num:    2,
            },
        }.to_string(), "()");
        assert_eq!(ParamsList{
            open: Token{
                ttype:      TokenType::DelOpenParen,
                raw:        "(".to_string(),
                line_num:   1,
                col_num:    1,
            },
            params: vec![
                Param{
                    ident: Token{
                        ttype:      TokenType::LitIdent,
                        raw:        "foo".to_string(),
                        line_num:   1,
                        col_num:    3,
                    },
                    delim: Token{
                        ttype:      TokenType::DelColon,
                        raw:        ":".to_string(),
                        line_num:   1,
                        col_num:    7,
                    },
                    ptype: Token{
                        ttype:      TokenType::LitIdent,
                        raw:        "Foo".to_string(),
                        line_num:   1,
                        col_num:    9,
                    },
                },
            ],
            delims: Vec::new(),
            close: Token{
                ttype:      TokenType::DelCloseParen,
                raw:        ")".to_string(),
                line_num:   1,
                col_num:    11,
            },
        }.to_string(), "( foo : Foo )");
        assert_eq!(ParamsList{
            open: Token{
                ttype:      TokenType::DelOpenParen,
                raw:        "(".to_string(),
                line_num:   1,
                col_num:    1,
            },
            params: vec![
                Param{
                    ident: Token{
                        ttype:      TokenType::LitIdent,
                        raw:        "foo".to_string(),
                        line_num:   1,
                        col_num:    3,
                    },
                    delim: Token{
                        ttype:      TokenType::DelColon,
                        raw:        ":".to_string(),
                        line_num:   1,
                        col_num:    7,
                    },
                    ptype: Token{
                        ttype:      TokenType::LitIdent,
                        raw:        "Foo".to_string(),
                        line_num:   1,
                        col_num:    9,
                    },
                },
                Param{
                    ident: Token{
                        ttype:      TokenType::LitIdent,
                        raw:        "bar".to_string(),
                        line_num:   1,
                        col_num:    14,
                    },
                    delim: Token{
                        ttype:      TokenType::DelColon,
                        raw:        ":".to_string(),
                        line_num:   1,
                        col_num:    18,
                    },
                    ptype: Token{
                        ttype:      TokenType::LitIdent,
                        raw:        "Bar".to_string(),
                        line_num:   1,
                        col_num:    20,
                    },
                },
            ],
            delims: vec![
                Token{
                    ttype:      TokenType::DelComma,
                    raw:        ",".to_string(),
                    line_num:   1,
                    col_num:    12,
                },
            ],
            close: Token{
                ttype:      TokenType::DelCloseParen,
                raw:        ")".to_string(),
                line_num:   1,
                col_num:    24,
            },
        }.to_string(), "( foo : Foo, bar : Bar )");
    }

    #[test]
    fn test_param_to_string() {
        assert_eq!(Param{
            ident: Token{
                ttype:      TokenType::LitIdent,
                raw:        "foo".to_string(),
                line_num:   1,
                col_num:    1,
            },
            delim: Token{
                ttype:      TokenType::DelColon,
                raw:        ":".to_string(),
                line_num:   1,
                col_num:    3,
            },
            ptype: Token {
                ttype:      TokenType::LitIdent,
                raw:        "Foo".to_string(),
                line_num:   1,
                col_num:    5,
            },
        }.to_string(), "foo : Foo");
    }

    #[test]
    fn test_expression_statement_to_string() {
        assert_eq!(ExpressionStatement{
            token:  Token{
                ttype:      TokenType::LitInt,
                raw:        "5".to_string(),
                line_num:   1,
                col_num:    1,
            },
            value: Some(Box::new(InfixExpression{
                left: Some(Box::new(IntegerLiteral{
                    token: Token{
                        ttype:      TokenType::LitInt,
                        raw:        "5".to_string(),
                        line_num:   1,
                        col_num:    1,
                    },
                    value: 5,
                })),
                op: Token{
                    ttype:      TokenType::OpAdd,
                    raw:        '+'.to_string(),
                    line_num:   1,
                    col_num:    3,
                },
                right: Some(Box::new(IntegerLiteral{
                    token: Token{
                        ttype:      TokenType::LitInt,
                        raw:        "10".to_string(),
                        line_num:   1,
                        col_num:    5,
                    },
                    value: 10,
                })),
            })),
        }.to_string(), "(5 + 10)");
    }

    #[test]
    fn test_prefix_expression_to_string() {
        assert_eq!(PrefixExpression{
            op: Token{
                ttype:      TokenType::OpAdd,
                raw:        '+'.to_string(),
                line_num:   1,
                col_num:    1,
            },
            right: Some(Box::new(IntegerLiteral{
                token: Token{
                    ttype:      TokenType::LitInt,
                    raw:        "5".to_string(),
                    line_num:   1,
                    col_num:    2,
                },
                value: 5,
            })),
        }.to_string(), "(+ 5)");
    }

    #[test]
    fn test_integer_literal_to_string() {
        assert_eq!(IntegerLiteral{
            token: Token{
                ttype:      TokenType::LitInt,
                raw:        "5".to_string(),
                line_num:   1,
                col_num:    1,
            },
            value: 5,
        }.to_string(), "5");
    }

    #[test]
    fn test_identifier_literal_to_string() {
        assert_eq!(IdentifierLiteral{
            token:  Token{
                ttype:      TokenType::LitIdent,
                raw:        "foo".to_string(),
                line_num:   1,
                col_num:    1,
            },
            value:  "foo".to_string(),
        }.to_string(), "foo");
    }

    #[test]
    fn test_infix_expression_to_string() {
        assert_eq!(InfixExpression{
            left: Some(Box::new(IntegerLiteral{
                token: Token{
                    ttype:      TokenType::LitInt,
                    raw:        "5".to_string(),
                    line_num:   1,
                    col_num:    1,
                },
                value: 5,
            })),
            op: Token{
                ttype:      TokenType::OpAdd,
                raw:        '+'.to_string(),
                line_num:   1,
                col_num:    3,
            },
            right: Some(Box::new(IntegerLiteral{
                token: Token{
                    ttype:      TokenType::LitInt,
                    raw:        "10".to_string(),
                    line_num:   1,
                    col_num:    5,
                },
                value: 10,
            })),
        }.to_string(), "(5 + 10)");
    }
}
