pub enum TokenType {
    MetaIllegal,
    MetaEOF,
}

pub struct Token {
    pub ttype:      TokenType,
    pub literal:    String,
}
