#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    MetaIllegal,
    MetaEOF,
    OpAdd,
    LitInt,
    DelEnd,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub ttype:          TokenType,
    pub literal:        String,
    pub line_num:       i32,
    pub char_pos:       i32,
}
