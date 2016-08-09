use error::JsResult;
//use error::error::{Error, ErrorType, SyntaxErrorType};
use lexer::enums::{TokenType};
use scope::parser::{Parser, Item};

macro_rules! wait {
    ($expr:expr) => (match $expr {
        Item::None => (),
        Item::Item => return Ok(Item::Item),
    })
}

macro_rules! none {
    ($expr:expr) => (match $expr {
        Item::None => return Ok(Item::None),
        Item::Item => (),
    })
}

impl Parser {
    pub fn dump_and_parse_logical_expr(&mut self) -> JsResult<Item> {
        println!("dump_and_parse_logical_expr {:?}", self.peek());
        try!(self.bump());
        self.parse_logical_expr(Item::None)
    }

    pub fn parse_logical_expr(&mut self, first: Item) -> JsResult<Item> {
        println!("parse_logical_expr {:?} {:?}", self.peek(), first);
        none!(try!(self.parse_unary_expr(first)));
        println!("  parse_logical_expr {:?}", self.peek());
        loop {
            match self.peek() {
                Some(TokenType::Plus) => self.dump_and_parse_logical_expr(),
                Some(TokenType::Minus) => self.dump_and_parse_logical_expr(),
                Some(TokenType::Multiple) => self.dump_and_parse_logical_expr(),
                Some(TokenType::Divide) => self.dump_and_parse_logical_expr(),
                Some(TokenType::Mod) => self.dump_and_parse_logical_expr(),
                Some(TokenType::Exp) => self.dump_and_parse_logical_expr(),
                Some(TokenType::LeftShift) => self.dump_and_parse_logical_expr(),
                Some(TokenType::RightShift) => self.dump_and_parse_logical_expr(),
                Some(TokenType::RightShiftUnsigned) => self.dump_and_parse_logical_expr(),
                Some(TokenType::GreaterThan) => self.dump_and_parse_logical_expr(),
                Some(TokenType::GreaterAndEqualThan) => self.dump_and_parse_logical_expr(),
                Some(TokenType::SmallAndEqualThan) => self.dump_and_parse_logical_expr(),
                Some(TokenType::SmallThan) => self.dump_and_parse_logical_expr(),
                Some(TokenType::Instanceof) => self.dump_and_parse_logical_expr(),
                Some(TokenType::In) => self.dump_and_parse_logical_expr(),
                Some(TokenType::IsEqual) => self.dump_and_parse_logical_expr(),
                Some(TokenType::IsNotEqual) => self.dump_and_parse_logical_expr(),
                Some(TokenType::IsSame) => self.dump_and_parse_logical_expr(),
                Some(TokenType::IsNotSame) => self.dump_and_parse_logical_expr(),
                Some(TokenType::And) => self.dump_and_parse_logical_expr(),
                Some(TokenType::AndBitwise) => self.dump_and_parse_logical_expr(),
                Some(TokenType::Or) => self.dump_and_parse_logical_expr(),
                Some(TokenType::OrBitwise) => self.dump_and_parse_logical_expr(),
                Some(TokenType::Xor) => self.dump_and_parse_logical_expr(),
                _ => return Ok(Item::Item)
            }
        }
    }
}