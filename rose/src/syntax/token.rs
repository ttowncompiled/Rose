#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    MetaIllegal,
    MetaEOF,
    RwClass,
    RwDef,
    RwEnd,
    RwPub,
    KwAs,
    OpAdd,
    OpDeref,
    LitInt,
    LitIdent,
    DelOpenParen,
    DelCloseParen,
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
