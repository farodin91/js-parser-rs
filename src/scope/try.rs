use error::JsResult;
use error::error::SyntaxErrorType;
use lexer::enums::{TokenType, Keyword, Punctuator};
use scope::parser::{Parser, Item};

impl Parser {
    pub fn parse_catch_parameter(&mut self) -> JsResult<Item> {
        match self.peek() {
            Some(TokenType::SymbolLiteral(_)) => {
                try!(self.bump());
            }
            Some(TokenType::Punctuator(Punctuator::LeftBrace)) => return Ok(Item::Item),
            Some(TokenType::Punctuator(Punctuator::LeftBracket)) => return Ok(Item::Item),
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

        if try!(self.consume(TokenType::Keyword(Keyword::Catch))) {
            try!(self.expect(TokenType::Punctuator(Punctuator::LeftParen)));
            try!(self.parse_catch_parameter());
            try!(self.expect(TokenType::Punctuator(Punctuator::RightParen)));
            try!(self.parse_block());
        }

        if try!(self.consume(TokenType::Keyword(Keyword::Finally))) {
            try!(self.parse_block());
        }
        Ok(Item::Item)
    }

    pub fn parse_throw(&mut self) -> JsResult<Item> {
        try!(self.bump());
        if try!(self.consume(TokenType::Semicolon)) {
            Ok(Item::Item)
        } else {
            self.parse_assign_expr()
        }
    }
}