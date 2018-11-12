pub enum TokenType {
    MetaIllegal,
    MetaEOF,
    OpAdd,
    LitInt,
}

pub struct Token {
    pub ttype:          TokenType,
    pub literal:        String,
    pub line_num:       i32,
    pub char_pos:       i32,
}
