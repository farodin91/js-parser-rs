use error::JsResult;
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
    pub fn parse_object_literal(&mut self) -> JsResult<Item> {
        println!("parse_object_literal {:?}", self.peek());
        try!(self.expect(TokenType::LeftBrace));
        loop {
            try!(self.consume_all_lineterminates());
            match try!(self.parse_property_definition()) {
                Item::None => {
                    println!("empty");
                    break
                },
                Item::Item => ()
            }
            if !try!(self.consume(TokenType::Comma)) {
                break
            }
        }
        try!(self.consume_all_lineterminates());
        try!(self.expect(TokenType::RightBrace));
        Ok(Item::Item)
    }

    pub fn parse_property_definition(&mut self) -> JsResult<Item> {
        println!("parse_property_definition {:?}", self.peek());
        match self.peek() {
            Some(TokenType::Get) => {
                try!(self.bump());
                if try!(self.consume(TokenType::Colon)) {
                    self.parse_assign_expr()
                } else {
                    try!(self.expect_identifier());
                    try!(self.expect(TokenType::LeftParen));
                    try!(self.expect(TokenType::RightParen));
                    self.parse_block()
                }
            }
            Some(TokenType::Set) => {
                try!(self.bump());
                if try!(self.consume(TokenType::Colon)) {
                    self.parse_assign_expr()
                } else {
                    try!(self.expect_identifier());
                    try!(self.expect(TokenType::LeftParen));
                    try!(self.expect_identifier());
                    try!(self.expect(TokenType::RightParen));
                    self.parse_block()
                }
            }
            Some(TokenType::LeftBracket) => Ok(Item::None),
            Some(TokenType::Multiple) => {
                try!(self.bump());
                try!(self.expect_identifier());
                try!(self.expect(TokenType::LeftParen));
                try!(self.parse_formal_parameters());
                try!(self.expect(TokenType::RightParen));
                self.parse_block()
            },
            Some(TokenType::Identifier(_)) => {
                try!(self.bump());
                if try!(self.consume(TokenType::LeftParen)) {
                    try!(self.parse_formal_parameters());
                    try!(self.expect(TokenType::RightParen));
                    self.parse_block()
                } else {
                    if try!(self.consume(TokenType::Colon)) {
                        return self.parse_assign_expr()
                    } else {
                        Ok(Item::None)
                    }
                }
            },
            Some(_) => Ok(Item::None),
            None => Ok(Item::None),
        }
    }
}