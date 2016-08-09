use error::JsResult;
use error::error::SyntaxErrorType;
use lexer::enums::{TokenType};
use scope::parser::{Parser, Item};

impl Parser {
    pub fn parse_catch_parameter(&mut self) -> JsResult<Item> {
        match self.peek() {
            Some(TokenType::Identifier(_)) => {
                try!(self.bump());
            }
            Some(TokenType::LeftBrace) => return Ok(Item::Item),
            Some(TokenType::LeftBracket) => return Ok(Item::Item),
            Some(t) => {
                try!(self.fatal(SyntaxErrorType::Unexpected(t)));
            }
            None => {
                try!(self.fatal(SyntaxErrorType::UnexpectedEOF));
            }
        }
        Ok(Item::Item)
    }

    pub fn parse_try(&mut self) -> JsResult<Item> {
        try!(self.bump());
        try!(self.parse_block());
        try!(self.consume_all_lineterminates());
        if try!(self.consume(TokenType::Catch)) {
            try!(self.expect(TokenType::LeftParen));
            try!(self.parse_catch_parameter());
            try!(self.expect(TokenType::RightParen));
            try!(self.parse_block());
            try!(self.consume_all_lineterminates());
        }

        if try!(self.consume(TokenType::Finally)) {
            try!(self.parse_block());
            try!(self.consume_all_lineterminates());
        }
        Ok(Item::Item)
    }

    pub fn parse_throw(&mut self) -> JsResult<Item> {
        try!(self.bump());
        try!(self.parse_assign_expr());
        try!(self.consume(TokenType::Semicolon));
        Ok(Item::Item)
    }
}