use token::Token;
use token::TokenType;
use ast::Node;
use ast::Statement;
use ast::Expression;
use ast::Program;
use ast::literal::ident::Identifier;
use ast::literal::blank::Blank;
use ast::literal::integer::IntegerLiteral;
use ast::literal::float::FloatLiteral;
use ast::literal::boolean::BooleanLiteral;
use ast::literal::inf::Inf;
use ast::literal::nan::NaN;
use ast::literal::nil::Nil;
use ast::literal::symbol::Symbol;
use ast::statement::binding::LetStatement;
use ast::statement::expression::ExpressionStatement;
use ast::statement::ret::ReturnStatement;
use ast::expression::prefix::PrefixExpression;
use ast::expression::infix::InfixExpression;
use lexer::Lexer;
use lexer::RoseLexer;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Precedence {
    LOWEST,
    OR,
    AND,
    EQUALS,
    LESSGREATER,
    SUM,
    PRODUCT,
    POW,
    PREFIX,
    CALL,
}

pub trait Parser<'a> {
    fn parse_program(&mut self) -> Option<Program>;
    fn parse_statement(&mut self) -> Option<Box<dyn Statement>>;
    fn parse_expression(&mut self) -> Option<Box<dyn Expression>>;
}

pub struct RoseParser<'a> {
    input:          &'a str,
    file_name:      String,
    lexer:          RoseLexer<'a>,
    pub errors:     Vec<String>,
    curr_token:     Token,
    peek_token:     Token,
}

impl<'a> RoseParser<'a> {
    pub fn new(input: &'a str, file_name: String) -> RoseParser<'a> {
        let mut lexer: RoseLexer = RoseLexer::new(input, file_name.clone());
        let peek_token: Token = lexer.next_token();
        let curr_token: Token = peek_token.clone();
        let peek_token: Token = lexer.next_token();
        let parser: RoseParser = RoseParser{
            input:          input,
            file_name:      file_name,
            lexer:          lexer,
            errors:         Vec::new(),
            curr_token:     curr_token,
            peek_token:     peek_token,
        };
        return parser;
    }

    fn next_token(&mut self) {
        self.curr_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn precedence(ttype: &TokenType) -> Precedence {
        return match ttype {
            TokenType::RW_OR            => Precedence::OR,
            TokenType::RW_AND           => Precedence::AND,
            TokenType::OP_EQ            => Precedence::EQUALS,
            TokenType::OP_NEQ           => Precedence::EQUALS,
            TokenType::OP_GT            => Precedence::LESSGREATER,
            TokenType::OP_GTE           => Precedence::LESSGREATER,
            TokenType::OP_LT            => Precedence::LESSGREATER,
            TokenType::OP_LTE           => Precedence::LESSGREATER,
            TokenType::OP_ADD           => Precedence::SUM,
            TokenType::OP_SUB           => Precedence::SUM,
            TokenType::OP_MUL           => Precedence::PRODUCT,
            TokenType::OP_DIV           => Precedence::PRODUCT,
            TokenType::OP_MOD           => Precedence::PRODUCT,
            TokenType::OP_POW           => Precedence::POW,
            TokenType::DEL_LPAREN       => Precedence::CALL,
            _                           => Precedence::LOWEST,
        };
    }

    fn curr_precedence(&self) -> Precedence {
        return RoseParser::precedence(&self.curr_token.ttype);
    }

    fn peek_precedence(&mut self) -> Precedence {
        return RoseParser::precedence(&self.peek_token.ttype);
    }

    fn do_prefix_parse_fn(&mut self, ttype: &TokenType) -> Option<Box<dyn Expression>> {
        return match ttype {
            TokenType::RW_NOT               => self.parse_prefix_expression(),
            TokenType::OP_NOT               => self.parse_prefix_expression(),
            TokenType::OP_ADD               => self.parse_prefix_expression(),
            TokenType::OP_SUB               => self.parse_prefix_expression(),
            TokenType::LIT_IDENT            => self.parse_identifier(),
            TokenType::LIT_BLANK            => self.parse_blank(),
            TokenType::LIT_INT              => self.parse_integer_literal(),
            TokenType::LIT_FLOAT            => self.parse_float_literal(),
            TokenType::LIT_BOOL             => self.parse_boolean_literal(),
            TokenType::LIT_INF              => self.parse_inf(),
            TokenType::LIT_NAN              => self.parse_nan(),
            TokenType::LIT_NIL              => self.parse_nil(),
            TokenType::LIT_SYMBOL           => self.parse_symbol(),
            _ => None,
        };
    }

    fn has_infix_parse_fn(ttype: &TokenType) -> bool {
        return match ttype {
            TokenType::RW_OR            => true,
            TokenType::RW_AND           => true,
            TokenType::OP_ADD           => true,
            TokenType::OP_SUB           => true,
            TokenType::OP_MUL           => true,
            TokenType::OP_DIV           => true,
            TokenType::OP_MOD           => true,
            TokenType::OP_POW           => true,
            TokenType::OP_EQ            => true,
            TokenType::OP_NEQ           => true,
            TokenType::OP_GT            => true,
            TokenType::OP_GTE           => true,
            TokenType::OP_LT            => true,
            TokenType::OP_LTE           => true,
            _                           => false,
        };
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn Statement>> {
        let token: Token = self.curr_token.clone();
        let battr: Option<Token>;
        if self.peek_token_is(&TokenType::RW_MUT) {
            self.next_token();
            battr = Some(self.curr_token.clone());
        } else {
            battr = None;
        }
        if ! self.expect_peek(&TokenType::LIT_IDENT) {
            return None;
        }
        let name: Identifier = Identifier::new(self.curr_token.clone());
        let btype: Option<Identifier>;
        if self.peek_token_is(&TokenType::DEL_COLON) {
            self.next_token();
            if ! self.expect_peek(&TokenType::LIT_IDENT) {
                return None;
            }
            btype = Some(Identifier::new(self.curr_token.clone()));
        } else {
            btype = None;
        }
        if self.peek_token_is(&TokenType::DEL_END) {
            self.next_token();
            return Some(Box::new(LetStatement::new(token, battr, name, btype, None)));
        }
        if ! self.expect_peek(&TokenType::OP_ASSIGN) {
            return None;
        }
        self.next_token();
        let value: Option<Box<dyn Expression>> = self.parse_expression();
        if self.peek_token_is(&TokenType::DEL_END) {
            self.next_token();
        }
        return Some(Box::new(LetStatement::new(token, battr, name, btype, value)));
    }

    fn parse_expression_statement(&mut self) -> Option<Box<dyn Statement>> {
        let token: Token = self.curr_token.clone();
        let value: Option<Box<dyn Expression>> = self.parse_expression();
        if self.peek_token_is(&TokenType::DEL_END) {
            self.next_token();
        }
        return Some(Box::new(ExpressionStatement::new(token, value)));
    }

    fn parse_return_statement(&mut self) -> Option<Box<dyn Statement>> {
        let token: Token = self.curr_token.clone();
        self.next_token();
        let value: Option<Box<dyn Expression>> = self.parse_expression();
        if self.peek_token_is(&TokenType::DEL_END) {
            self.next_token();
        }
        return Some(Box::new(ReturnStatement::new(token, value)));
    }

    fn parse_expression_with_precedence(&mut self, precedence: Precedence) -> Option<Box<dyn Expression>> {
        let ttype: TokenType = self.curr_token.ttype.clone();
        let prefix: Option<Box<dyn Expression>> = self.do_prefix_parse_fn(&ttype);
        if prefix.is_none() {
            self.no_prefix_parse_fn_error();
            return None;
        }
        let mut left: Option<Box<dyn Expression>> = prefix;
        while ! self.peek_token_is(&TokenType::DEL_END) && precedence < self.peek_precedence() {
            if Self::has_infix_parse_fn(&self.peek_token.ttype) {
                self.next_token();
                left = self.parse_infix_expression(left);
            } else {
                return left;
            }
        }
        return left;
    }

    fn parse_prefix_expression(&mut self) -> Option<Box<dyn Expression>> {
        let token: Token = self.curr_token.clone();
        self.next_token();
        let value: Option<Box<dyn Expression>> = self.parse_expression_with_precedence(Precedence::PREFIX);
        return Some(Box::new(PrefixExpression::new(token, value)));
    }

    fn parse_infix_expression(&mut self, left: Option<Box<dyn Expression>>) -> Option<Box<dyn Expression>> {
        let token: Token = self.curr_token.clone();
        let precedence: Precedence = self.curr_precedence();
        self.next_token();
        let right: Option<Box<dyn Expression>> = self.parse_expression_with_precedence(precedence);
        return Some(Box::new(InfixExpression::new(token, left, right)));
    }

    fn parse_identifier(&mut self) -> Option<Box<dyn Expression>> {
        return Some(Box::new(Identifier::new(self.curr_token.clone())));
    }

    fn parse_blank(&mut self) -> Option<Box<dyn Expression>> {
        return Some(Box::new(Blank::new(self.curr_token.clone())));
    }

    fn parse_integer_literal(&mut self) -> Option<Box<dyn Expression>> {
        return match self.curr_token.literal.parse::<i32>() {
            Ok(val) => Some(Box::new(IntegerLiteral::new(self.curr_token.clone(), val))),
            Err(err) => {
                self.errors.push(err.to_string());
                None
            },
        };
    }

    fn parse_float_literal(&mut self) -> Option<Box<dyn Expression>> {
        return match self.curr_token.literal.parse::<f64>() {
            Ok(val) => Some(Box::new(FloatLiteral::new(self.curr_token.clone(), val))),
            Err(err) => {
                self.errors.push(err.to_string());
                None
            },
        };
    }

    fn parse_boolean_literal(&mut self) -> Option<Box<dyn Expression>> {
        return Some(Box::new(BooleanLiteral::new(self.curr_token.clone(), self.curr_token.literal == "true")));
    }

    fn parse_inf(&mut self) -> Option<Box<dyn Expression>> {
        return Some(Box::new(Inf::new(self.curr_token.clone())));
    }

    fn parse_nan(&mut self) -> Option<Box<dyn Expression>> {
        return Some(Box::new(NaN::new(self.curr_token.clone())));
    }

    fn parse_nil(&mut self) -> Option<Box<dyn Expression>> {
        return Some(Box::new(Nil::new(self.curr_token.clone())));
    }

    fn parse_symbol(&mut self) -> Option<Box<dyn Expression>> {
        return Some(Box::new(Symbol::new(self.curr_token.clone())));
    }

    fn peek_token_is(&self, ttype: &TokenType) -> bool {
        return self.peek_token.ttype == *ttype;
    }

    pub fn expect_peek(&mut self, ttype: &TokenType) -> bool {
        if self.peek_token_is(ttype) {
            self.next_token();
            return true;
        } else {
            self.peek_error(ttype);
            return false;
        }
    }

    fn peek_error(&mut self, ttype: &TokenType) {
        self.errors.push(format!("expected next token to be {:?}, got {:?} instead", ttype, self.peek_token.ttype));
    }

    fn no_prefix_parse_fn_error(&mut self) {
        self.errors.push(format!("no prefix parse function for {:?} found", self.curr_token.ttype));
    }
}

impl<'a> Parser<'a> for RoseParser<'a> {
    fn parse_program(&mut self) -> Option<Program> {
        let mut program: Program = Program::new();
        while self.curr_token.ttype != TokenType::META_EOF {
            if self.curr_token.ttype == TokenType::DEL_END {
                self.next_token();
                continue;
            }
            match self.parse_statement() {
                Some(statement) => program.statements.push(statement),
                None => (),
            }
            self.next_token();
        }
        return Some(program);
    }

    fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        if self.curr_token.ttype == TokenType::RW_LET {
            self.parse_let_statement()
        } else if self.curr_token.ttype == TokenType::RW_RETURN {
            self.parse_return_statement()
        } else {
            self.parse_expression_statement()
        }
    }

    fn parse_expression(&mut self) -> Option<Box<dyn Expression>> {
        return self.parse_expression_with_precedence(Precedence::LOWEST);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_let_statement() {
        let input: &str = "
let x = 5;
let y = 10;
let foobar = 838383;
";
        let count: usize = 3;
        let mut parser: RoseParser = RoseParser::new(input, "test_let_statement".to_string());
        let program: Option<Program> = parser.parse_program();
        check_parser_errors(&parser);
        match program {
            Some(prog) => {
                if prog.statements.len() != count {
                    assert!(false, "program.statements does not contain {} statements, got={}", count, prog.statements.len());
                }
                let mut i = 1;
                for stmt in prog.statements.iter() {
                    assert_eq!(Some("let".to_string()), (**stmt).token_literal(), "tests[{}]", i);
                    i += 1;
                }
            },
            None => assert!(false, "parse_program() returns None"),
        }
    }

    #[test]
    fn test_return_statement() {
        let input: &str  = "
return 5;
return 10;
return 838383;
";
        let count: usize = 3;
        let mut parser: RoseParser = RoseParser::new(input, "test_return_statement".to_string());
        let program: Option<Program> = parser.parse_program();
        check_parser_errors(&parser);
        match program {
            Some(prog) => {
                if prog.statements.len() != count {
                    assert!(false, "program.statements does not contain {} statements, got={}", count, prog.statements.len());
                }
                let mut i = 1;
                for stmt in prog.statements.iter() {
                    assert_eq!(Some("return".to_string()), (**stmt).token_literal(), "tests[{}]", i);
                    i += 1;
                }
            },
            None => assert!(false, "parse_program() returns None"),
        }
    }

    #[test]
    fn test_identifier_expression() {
        let input: &str = "foo;";
        let count: usize = 1;
        let mut parser: RoseParser = RoseParser::new(input, "test_identifier_expression".to_string());
        let program: Option<Program> = parser.parse_program();
        check_parser_errors(&parser);
        match program {
            Some(prog) => {
                if prog.statements.len() != count {
                    assert!(false, "program.statements does not contain {} statements, got={}", count, prog.statements.len());
                }
                assert_eq!(Some("foo".to_string()), (*prog.statements[0]).token_literal(), "tests[{}]", 1);
            },
            None => assert!(false, "parse_program() returns None"),
        }
    }

    #[test]
    fn test_blank_expression() {
        let input: &str = "_;";
        let count: usize = 1;
        let mut parser: RoseParser = RoseParser::new(input, "test_blank_expression".to_string());
        let program: Option<Program> = parser.parse_program();
        check_parser_errors(&parser);
        match program {
            Some(prog) => {
                if prog.statements.len() != count {
                    assert!(false, "program.statements does not contain {} statements, got={}", count, prog.statements.len());
                }
                assert_eq!(Some("_".to_string()), (*prog.statements[0]).token_literal(), "tests[{}]", 1);
            },
            None => assert!(false, "parse_program() returns None"),
        }
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = "5;";
        let count: usize = 1;
        let mut parser: RoseParser = RoseParser::new(input, "test_integer_literal_expression".to_string());
        let program: Option<Program> = parser.parse_program();
        check_parser_errors(&parser);
        match program {
            Some(prog) => {
                if prog.statements.len() != count {
                    assert!(false, "program.statements does not contain {} statements, got={}", count, prog.statements.len());
                }
                assert_eq!(Some("5".to_string()), (*prog.statements[0]).token_literal(), "tests[{}]", 1);
            },
            None => assert!(false, "parse_program() returns None"),
        }
    }

    #[test]
    fn test_float_literal_expression() {
        let input = "5.5;";
        let count: usize = 1;
        let mut parser: RoseParser = RoseParser::new(input, "test_float_literal_expression".to_string());
        let program: Option<Program> = parser.parse_program();
        check_parser_errors(&parser);
        match program {
            Some(prog) => {
                if prog.statements.len() != count {
                    assert!(false, "program.statements does not contain {} statements, got={}", count, prog.statements.len());
                }
                assert_eq!(Some("5.5".to_string()), (*prog.statements[0]).token_literal(), "tests[{}]", 1);
            },
            None => assert!(false, "parse_program() returns None"),
        }
    }

    #[test]
    fn test_boolean_literal_expression() {
        let input = "
true;
false;";
        let count: usize = 2;
        let mut parser: RoseParser = RoseParser::new(input, "test_boolean_literal_expression".to_string());
        let program: Option<Program> = parser.parse_program();
        check_parser_errors(&parser);
        match program {
            Some(prog) => {
                if prog.statements.len() != count {
                    assert!(false, "program.statements does not contain {} statements, got={}", count, prog.statements.len());
                }
                assert_eq!(Some("true".to_string()), (*prog.statements[0]).token_literal(), "tests[{}]", 1);
                assert_eq!(Some("false".to_string()), (*prog.statements[1]).token_literal(), "tests[{}]", 2);
            },
            None => assert!(false, "parse_program() returns None"),
        }
    }

    #[test]
    fn test_inf_expression() {
        let input: &str = "Inf;";
        let count: usize = 1;
        let mut parser: RoseParser = RoseParser::new(input, "test_inf_expression".to_string());
        let program: Option<Program> = parser.parse_program();
        check_parser_errors(&parser);
        match program {
            Some(prog) => {
                if prog.statements.len() != count {
                    assert!(false, "program.statements does not contain {} statements, got={}", count, prog.statements.len());
                }
                assert_eq!(Some("Inf".to_string()), (*prog.statements[0]).token_literal(), "tests[{}]", 1);
            },
            None => assert!(false, "parse_program() returns None"),
        }
    }

    #[test]
    fn test_nan_expression() {
        let input: &str = "NaN;";
        let count: usize = 1;
        let mut parser: RoseParser = RoseParser::new(input, "test_nan_expression".to_string());
        let program: Option<Program> = parser.parse_program();
        check_parser_errors(&parser);
        match program {
            Some(prog) => {
                if prog.statements.len() != count {
                    assert!(false, "program.statements does not contain {} statements, got={}", count, prog.statements.len());
                }
                assert_eq!(Some("NaN".to_string()), (*prog.statements[0]).token_literal(), "tests[{}]", 1);
            },
            None => assert!(false, "parse_program() returns None"),
        }
    }

    #[test]
    fn test_nil_expression() {
        let input: &str = "nil;";
        let count: usize = 1;
        let mut parser: RoseParser = RoseParser::new(input, "test_nil_expression".to_string());
        let program: Option<Program> = parser.parse_program();
        check_parser_errors(&parser);
        match program {
            Some(prog) => {
                if prog.statements.len() != count {
                    assert!(false, "program.statements does not contain {} statements, got={}", count, prog.statements.len());
                }
                assert_eq!(Some("nil".to_string()), (*prog.statements[0]).token_literal(), "tests[{}]", 1);
            },
            None => assert!(false, "parse_program() returns None"),
        }
    }

    #[test]
    fn test_symbol_expression() {
        let input: &str = ":foo;";
        let count: usize = 1;
        let mut parser: RoseParser = RoseParser::new(input, "test_symbol_expression".to_string());
        let program: Option<Program> = parser.parse_program();
        check_parser_errors(&parser);
        match program {
            Some(prog) => {
                if prog.statements.len() != count {
                    assert!(false, "program.statements does not contain {} statements, got={}", count, prog.statements.len());
                }
                assert_eq!(Some(":foo".to_string()), (*prog.statements[0]).token_literal(), "tests[{}]", 1);
            },
            None => assert!(false, "parse_program() returns None"),
        }
    }

    #[test]
    fn test_parsing_prefix_expressions() {
        let input: &str = "
!5;
-5";
        let count: usize = 2;
        let mut parser: RoseParser = RoseParser::new(input, "test_parsing_prefix_expression".to_string());
        let program: Option<Program> = parser.parse_program();
        check_parser_errors(&parser);
        match program {
            Some(prog) => {
                if prog.statements.len() != count {
                    assert!(false, "program.statements does not contain {} statements, got={}", count, prog.statements.len());
                }
                assert_eq!("(!5);".to_string(), (*prog.statements[0]).to_string(), "tests[{}]", 1);
                assert_eq!("(-5);".to_string(), (*prog.statements[1]).to_string(), "tests[{}]", 2);
            },
            None => assert!(false, "parse_program() returns None"),
        }
    }

    #[test]
    fn test_parsing_infix_expressions() {
        let input: &str = "
5 + 5;
5 - 5;
5 * 5;
5 / 5;
5 > 5;
5 < 5;
5 == 5;
5 != 5;";
        let count: usize = 8;
        let mut parser: RoseParser = RoseParser::new(input, "test_parsing_infix_expression".to_string());
        let program: Option<Program> = parser.parse_program();
        check_parser_errors(&parser);
        match program {
            Some(prog) => {
                if prog.statements.len() != count {
                    assert!(false, "program.statements does not contain {} statements, got={}", count, prog.statements.len());
                }
                assert_eq!("(5 + 5);".to_string(), (*prog.statements[0]).to_string(), "tests[{}]", 1);
                assert_eq!("(5 - 5);".to_string(), (*prog.statements[1]).to_string(), "tests[{}]", 2);
                assert_eq!("(5 * 5);".to_string(), (*prog.statements[2]).to_string(), "tests[{}]", 3);
                assert_eq!("(5 / 5);".to_string(), (*prog.statements[3]).to_string(), "tests[{}]", 4);
                assert_eq!("(5 > 5);".to_string(), (*prog.statements[4]).to_string(), "tests[{}]", 5);
                assert_eq!("(5 < 5);".to_string(), (*prog.statements[5]).to_string(), "tests[{}]", 6);
                assert_eq!("(5 == 5);".to_string(), (*prog.statements[6]).to_string(), "tests[{}]", 7);
                assert_eq!("(5 != 5);".to_string(), (*prog.statements[7]).to_string(), "tests[{}]", 8);
            },
            None => assert!(false, "parse_program() returns None"),
        }
    }

    fn check_parser_errors(parser: &RoseParser) {
        if parser.errors.len() == 0 {
            return;
        }
        let mut output: String = String::new();
        output.push_str(&format!("\n\nparser has {} errors", parser.errors.len()));
        for err in parser.errors.iter() {
            output.push_str(&format!("\nparser error: {}", err));
        }
        output.push_str("\n\n");
        assert!(false, output);
    }
}
