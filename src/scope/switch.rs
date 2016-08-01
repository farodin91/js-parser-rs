use error::JsResult;
use error::error::SyntaxErrorType;
use lexer::enums::{TokenType, Keyword, Punctuator};
use scope::parser::{Parser, Item};

impl Parser {
    pub fn parse_case_clause(&mut self) -> JsResult<Item> {
        try!(self.bump());
        try!(self.parse_expr());
        try!(self.expect(TokenType::Punctuator(Punctuator::Colon)));
        try!(self.parse_stmt_list());
        Ok(Item::Item)
    }

    pub fn parse_default_clause(&mut self) -> JsResult<Item> {
        try!(self.bump());
        try!(self.expect(TokenType::Punctuator(Punctuator::Colon)));
        try!(self.parse_stmt_list());
        Ok(Item::Item)
    }

    pub fn parse_case_block(&mut self) -> JsResult<Item> {
        try!(self.expect(TokenType::Punctuator(Punctuator::LeftBrace)));
        let mut default = false;
        loop {
            match self.peek() {
                Some(TokenType::Keyword(Keyword::Case)) => {
                    try!(self.parse_case_clause());
                },
                Some(TokenType::Keyword(Keyword::Default)) => {
                    if default {
                        try!(self.fatal(SyntaxErrorType::Unexpected(TokenType::Keyword(Keyword::Default))));
                    }
                    default = true;
                    try!(self.parse_case_clause());
                },
                Some(TokenType::Punctuator(Punctuator::RightBrace)) => break,
                Some(t) => {
                    try!(self.fatal(SyntaxErrorType::Unexpected(t)));
                }
                None => {
                    try!(self.fatal(SyntaxErrorType::UnexpectedEOF));
                }
            }
        }

        try!(self.expect(TokenType::Punctuator(Punctuator::RightBrace)));
        Ok(Item::Item)
    }

    pub fn parse_switch(&mut self) -> JsResult<Item> {
        try!(self.bump());
        try!(self.expect(TokenType::Punctuator(Punctuator::LeftParen)));
        try!(self.parse_expr());
        try!(self.expect(TokenType::Punctuator(Punctuator::RightParen)));
        self.parse_case_block()
    }
}