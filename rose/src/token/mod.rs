#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Meta-tokens
    ILLEGAL,            // unrecognized character
    EOF,                // end-of-file
    PRE_PROC,           // #[foo]
    COMMENT,            // single-line comment
    DOC_STRING,         // multi-line comment
    AS,                 // as performs cast
    // Identifiers + Literals
    IDENT,              // x, y, z
    INT,                // 5
    FLOAT,              // 5.0
    BOOL,               // true, false
    CHAR,               // 'a'
    STRING,             // "a"
    INF,                // Inf
    NAN,                // NaN
    NIL,                // nil
    SYMBOL,             // :foo
    MACRO,              // @foo
    // Assignment Operators
    ASSIGN,             // =
    COLON_ASSIGN,       // := assign and infer type
    PLUS_ASSIGN,        // +=
    MINUS_ASSIGN,       // -=
    ASTERISK_ASSIGN,    // *=
    SLASH_ASSIGN,       // /=
    MOD_ASSIGN,         // %=
    // Unary Operators
    PLUS,               // +    prefix
    MINUS,              // -    prefix
    IM,                 // im   postfix
    BANG,               // !    prefix
    // Arithmetic Operators
    ASTERISK,           // *
    SLASH,              // /
    MOD,                // %
    POW,                // ** exponentiation
    DOUBLE_SLASH,       // // rational division
    CMP,                // <=> comparitor
    // Boolean Operators
    EQ,                 // == equal
    NEQ,                // != not equal
    GT,                 // > greater than
    GTE,                // >= greater than or equal to
    LT,                 // < less than
    LTE,                // <= less than or equal to
    AND,                // and
    OR,                 // or
    XOR,                // xor
    NOT,                // not
    IS,                 // is
    // Bitwise Operators
    LSHIFT,             // << left-shift
    RSHIFT,             // >> right-shift
    LAND,               // && bitwise and
    LOR,                // || bitwise or
    LXOR,               // ^ bitwise xor
    LNOT,               // ~ bitwise not
    // Dispatch Operators
    DOT,                // .
    DOUBLE_COLON,       // ::
    // Function Operators
    PIPE,               // |
    ARROW,              // ->
    FAT_ARROW,          // =>
    // Broadcast Operators
    DOT_PLUS,           // .+
    DOT_MINUS,          // .-
    DOT_ASTERISK,       // .*
    DOT_SLASH,          // ./
    DOT_MOD,            // .%
    DOT_POW,            // .**
    DOT_DOUBLE_SLASH,   // .//
    DOT_CMP,            // .<=>
    // Range + Sequence Operators
    DOUBLE_DOT,         // ..
    TRIPLE_DOT,         // ...
    // Delimiters
    SEMICOLON,          // ;
    NEWLINE,            // \n \r
    COMMA,              // ,
    COLON,              // :
    // Collection + Scope Delimiters
    LPAREN,             // (
    RPAREN,             // )
    LBRACE,             // {
    RBRACE,             // }
    LBRACKET,           // [
    RBRACKET,           // ]
    // Statement Operators
    LET,                // let
    // Block Operators
    WITH,               // with
    BEGIN,              // begin
    DO,                 // do
    END,                // end
    // Branching Operators
    IF,                 // if
    ELSE,               // else
    LOOP,               // loop
    WHILE,              // while
    FOR,                // for
    IN,                 // in
    OF,                 // of
    MATCH,              // match
    // Module Operators
    MODULE,             // mod
    USE,                // use
    // Type Operators
    CLASS,              // class
    TRAIT,              // trait
    DEF,                // def
    FUNCTION,           // fn
    STRUCT,             // struct
    TUPLE,              // tuple
    TYPE,               // type
    EXT,                // ext
    IMPL,               // impl
    HAS,                // has
    USES,               // uses
    SELF,               // self
    SELF_TYPE,          // Self
    SUPER,              // super
    SUPER_TYPE,         // Super
    // Privacy Operators
    PUB,                // pub
    PRO,                // pro
    // Binding Operators
    MUT,                // mut
    ABS,                // abs
    CONST,              // const
    STATIC,             // static
    OVERRIDE,           // override
    OVERLOAD,           // overload
    // Allocation + Borrow + Ownership + Lifetime Operators
    NEW,                // new
    BORROW,             // &
    MOVE,               // move
    LIFETIME,           // '
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub ttype:          TokenType,
    pub literal:        String,
    pub file_name:      String,
    pub line_number:    i64,
    pub char_offset:    i64,
}

impl Token {
    pub fn new(ttype: TokenType, literal: String, file_name: String, line_number: i64, char_offset: i64) -> Token {
        return Token{
            ttype:          ttype,
            literal:        literal,
            file_name:      file_name,
            line_number:    line_number,
            char_offset:    char_offset,
        };
    }

    pub fn len(&self) -> usize {
        return self.literal.len();
    }
}

#[derive(Debug, PartialEq)]
pub struct TokenBuilder {
    pub ttype:          TokenType,
    pub literal:        String,
    pub file_name:      String,
    pub line_number:    i64,
    pub char_offset:    i64,
}

impl TokenBuilder {
    pub fn new(file_name: String) -> TokenBuilder {
        return TokenBuilder{
            ttype:          TokenType::ILLEGAL,
            literal:        String::new(),
            file_name:      file_name,
            line_number:    0,
            char_offset:    0,
        };
    }

    pub fn done(ttype: TokenType, literal: String, file_name: String, line_number: i64, char_offset: i64) -> Token {
        return Token{
            ttype:          ttype,
            literal:        literal,
            file_name:      file_name,
            line_number:    line_number,
            char_offset:    char_offset,
        };
    }

    pub fn build(&self) -> Option<Token> {
        if self.literal != "" && self.line_number > 0 && self.char_offset > 0 {
            return Some(Token::new(self.ttype, self.literal, self.file_name, self.line_number, self.char_offset));
        }
        return None;
    }
}

pub trait TokenFactory {
    fn new(file_name: String) -> Self;
    fn manufacture(&mut self, ch: char, peek_ch: char, line_number: i64, char_offset: i64) -> Option<Token>;
    fn is_letter(&self, ch: char) -> bool;
    fn is_digit(&self, ch: char) -> bool;
    fn is_special_char(&self, ch: char) -> bool;
    fn is_whitespace(&self, ch: char) -> bool;
}

pub struct RoseTokenFactory {
    pub file_name:      String,
    pub builder:        Option<TokenBuilder>,
}

impl TokenFactory for RoseTokenFactory {
    fn new(file_name: String) -> Self {
        return RoseTokenFactory{
            file_name:      file_name,
            builder:        None,
        };
    }

    fn manufacture(&mut self, ch: char, peek_ch: char, line_number: i64, char_offset: i64) -> Option<Token> {
        return match ch {
            '+'     => Some(TokenBuilder::done(TokenType::ADD, String::from("+"), self.file_name, line_number, char_offset)),
            '-'     => Some(TokenBuilder::done(TokenType::SUB, String::from("-"), self.file_name, line_number, char_offset)),
            '*'     => Some(TokenBuilder::done(TokenType::MUL, String::from("*"), self.file_name, line_number, char_offset)),
            '/'     => Some(TokenBuilder::done(TokenType::DIV, String::from("/"), self.file_name, line_number, char_offset)),
            _       => None,
        };
    }

    pub fn is_letter(&self, ch: char) -> bool {
        return 'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_';
    }

    pub fn is_digit(&self, ch: char) -> bool {
        return '0' <= ch && ch <= '9';
    }

    pub fn is_special_char(&self, ch: char) -> bool {
        return ch == '!' || ch == '?';
    }

    pub fn is_whitespace(&self, ch: char) -> bool {
        return ch == ' ' || ch == '\t';
    }
}

