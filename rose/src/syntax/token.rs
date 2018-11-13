#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    MetaIllegal,
    MetaEOF,
    OpAdd,
    LitInt,
    DelEnd,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub ttype:          TokenType,
    pub literal:        String,
    pub line_num:       i32,
    pub col_num:        i32,
}
