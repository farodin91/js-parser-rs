use error::JsResult;
//use error::error::{Error, ErrorType, SyntaxErrorType};
use lexer::enums::{TokenType};
use scope::parser::{Parser, Item};

macro_rules! wait {
    ($expr:expr) => (match $expr {
        Item::Item => return Ok(Item::Item),
        Item::None => (),
    })
}


macro_rules! none {
    ($expr:expr) => (match $expr {
        Item::None => return Ok(Item::None),
        Item::Item => (),
    })
}

impl Parser {
    pub fn parse_function(&mut self) -> JsResult<Item> {
        println!("parse_function {:?}", self.peek());
        if !try!(self.consume(TokenType::Function)) {
            return Ok(Item::None)
        }
        try!(self.consume(TokenType::Multiple));
        try!(self.consume_identifier());
        try!(self.parse_formal_parameters());
        self.parse_block()
    }

    pub fn parse_formals_list(&mut self) -> JsResult<Item> {
        try!(self.consume_identifier());
        Ok(Item::Item)
    }

    pub fn parse_formal_parameters(&mut self) -> JsResult<Item> {
        try!(self.expect(TokenType::LeftParen));
        try!(self.parse_formals_list());
        while try!(self.consume(TokenType::Comma)) {
            try!(self.parse_formals_list());
        }
        try!(self.expect(TokenType::RightParen));
        Ok(Item::Item)
    }


    pub fn parse_function_expr(&mut self) -> JsResult<Item> {
        println!("parse_function_expr {:?}", self.peek());
        if !try!(self.consume(TokenType::Function)) {
            return Ok(Item::None)
        }
        try!(self.consume(TokenType::Multiple));
        try!(self.consume_identifier());
        try!(self.parse_formal_parameters());
        self.parse_block()
    }
}