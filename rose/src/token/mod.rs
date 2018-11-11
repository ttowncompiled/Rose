pub enum TokenType {
    MetaIllegal,
    MetaEOF,
    OpAdd
}

pub struct Token {
    pub ttype:          TokenType,
    pub literal:        String,
    pub line_num:       i32,
    pub char_pos:       i32,
}
