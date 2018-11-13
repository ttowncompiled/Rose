#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    MetaIllegal,
    MetaEOF,
    RwPub,
    RwClass,
    RwDef,
    RwEnd,
    KwAs,
    OpAdd,
    OpDeref,
    LitInt,
    LitIdent,
    DelLParen,
    DelRParen,
    DelColon,
    DelComma,
    DelEnd,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub ttype:          TokenType,
    pub raw:            String,
    pub line_num:       i32,
    pub col_num:        i32,
}
