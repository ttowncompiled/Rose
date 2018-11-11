pub enum TokenType {
    MetaIllegal,
    MetaEOF,
    OpAdd
}

pub struct Token {
    pub ttype:      TokenType,
    pub literal:    String,
}
