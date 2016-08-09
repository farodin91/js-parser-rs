use error::JsResult;
use error::error::SyntaxErrorType;
use lexer::enums::{TokenType};
use scope::parser::{Parser, Item};

impl Parser {
    pub fn parse_case_clause(&mut self) -> JsResult<Item> {
        try!(self.bump());
        try!(self.parse_expr());
        try!(self.expect(TokenType::Colon));
        try!(self.parse_stmt_list());
        Ok(Item::Item)
    }

    pub fn parse_default_clause(&mut self) -> JsResult<Item> {
        try!(self.bump());
        try!(self.expect(TokenType::Colon));
        try!(self.parse_stmt_list());
        Ok(Item::Item)
    }

    pub fn parse_case_block(&mut self) -> JsResult<Item> {
        try!(self.expect(TokenType::LeftBrace));
        let mut default = false;
        loop {
            match self.peek() {
                Some(TokenType::Case) => {
                    try!(self.parse_case_clause());
                },
                Some(TokenType::Default) => {
                    if default {
                        try!(self.fatal(SyntaxErrorType::Unexpected(TokenType::Default)));
                    }
                    default = true;
                    try!(self.parse_case_clause());
                },
                Some(TokenType::RightBrace) => break,
                Some(t) => {
                    try!(self.fatal(SyntaxErrorType::Unexpected(t)));
                }
                None => {
                    try!(self.fatal(SyntaxErrorType::UnexpectedEOF));
                }
            }
        }

        try!(self.expect(TokenType::RightBrace));
        Ok(Item::Item)
    }

    pub fn parse_switch(&mut self) -> JsResult<Item> {
        try!(self.bump());
        try!(self.expect(TokenType::LeftParen));
        try!(self.parse_expr());
        try!(self.expect(TokenType::RightParen));
        self.parse_case_block()
    }
}