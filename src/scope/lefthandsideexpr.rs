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
    pub fn parse_left_hand_side_expr(&mut self) -> JsResult<Item> {//done
        println!("parse_left_hand_side_expr {:?}", self.peek());
        wait!(try!(self.parse_call_expr()));
        self.parse_new_expr()
    }

    pub fn parse_new_expr(&mut self) -> JsResult<Item> {//done
        println!("parse_new_expr {:?}", self.peek());
        wait!(try!(self.parse_member_expr()));
        if try!(self.consume(TokenType::New)) {
            try!(self.parse_new_expr());
            Ok(Item::Item)
        } else {
            Ok(Item::None)
        }
    }

    pub fn parse_super_prop(&mut self) -> JsResult<Item> {
        if !try!(self.consume(TokenType::Super)) {
            return Ok(Item::None)
        }
        if try!(self.consume(TokenType::LeftBracket)) {
            try!(self.expect(TokenType::RightBracket));
        } else {
            try!(self.expect(TokenType::Point));
            try!(self.expect_identifier());
        }
        Ok(Item::Item)
    }

    pub fn parse_meta_prop(&mut self) -> JsResult<Item> {
        try!(self.expect(TokenType::New));
        try!(self.expect(TokenType::Point));
        try!(self.expect(TokenType::Target));
        Ok(Item::Item)
    }

    pub fn bump_new_and_member_expr_and_arguments(&mut self) -> JsResult<Item> {
        println!("bump_new_and_member_expr_and_arguments {:?}", self.peek());
        if !try!(self.consume(TokenType::New)) {
            return Ok(Item::None)
        }
        try!(self.parse_member_expr());
        try!(self.parse_arguments());
        Ok(Item::Item)
    }

    pub fn parse_arguments(&mut self) -> JsResult<Item> {
        println!("parse_arguments {:?}", self.peek());
        if !try!(self.consume(TokenType::LeftParen)) {
            return Ok(Item::None)
        }
        if !try!(self.consume(TokenType::ThreePoints)) {
            try!(self.parse_expr());
            try!(self.consume(TokenType::ThreePoints));
        }
        try!(self.consume_all_lineterminates());
        try!(self.expect(TokenType::RightParen));
        Ok(Item::Item)
    }

    pub fn parse_member(&mut self) -> JsResult<Item> {
        println!("parse_member {:?}", self.peek());
        if try!(self.consume(TokenType::LeftBracket)) {
            try!(self.parse_expr());
            try!(self.expect(TokenType::RightBracket));
            return Ok(Item::Item)
        }
        if try!(self.consume(TokenType::Point)) {
            try!(self.expect_identifier_name());
            return Ok(Item::Item)
        }
        Ok(Item::None)
    }

    pub fn member_expr_and_members(&mut self) -> JsResult<Item> {
        println!("member_expr_and_members {:?}", self.peek());
        none!(try!(self.parse_member_expr()));
        self.parse_member()
    }


    pub fn parse_member_expr(&mut self) -> JsResult<Item> {
        let mut first = false;
        loop {
            println!("parse_member_expr {:?} {:?}", self.peek(), first);
            match try!(self.parse_primary_expr()) {
                Item::Item => {
                    first = true;
                    continue
                },
                Item::None => (),
            }
            match try!(self.parse_super_prop()) {
                Item::Item => {
                    first = true;
                    continue
                },
                Item::None => (),
            }
            if !first {
                return Ok(Item::None)
            }
            match try!(self.parse_member()) {
                Item::Item => continue,
                Item::None => (),
            }
            break
        }
        Ok(Item::Item)
    }

    pub fn parse_super_call(&mut self) -> JsResult<Item> {
        println!("parse_super_call {:?}", self.peek());
        if !try!(self.consume(TokenType::Super)) {
            return Ok(Item::None)
        }
        try!(self.parse_arguments());
        Ok(Item::Item)
    }

    pub fn member_and_arguments(&mut self) -> JsResult<Item> {
        println!("member_and_arguments {:?}", self.peek());
        none!(try!(self.parse_member_expr()));
        self.parse_arguments()
    }

    pub fn parse_call_expr(&mut self) -> JsResult<Item> {
        let mut first = false;
        loop {
            println!("parse_call_expr {:?} {:?}", self.peek(), first);
            match try!(self.member_and_arguments()) {
                Item::Item => {
                    first = true;
                    continue
                },
                Item::None => (),
            }
            match try!(self.parse_super_call()) {
                Item::Item => {
                    first = true;
                    continue
                },
                Item::None => (),
            }
            if !first {
                return Ok(Item::None)
            }
            match try!(self.parse_member()) {
                Item::Item => continue,
                Item::None => (),
            }
            match try!(self.parse_arguments()) {
                Item::Item => continue,
                Item::None => (),
            }
            break
        }
        Ok(Item::Item)
    }
}