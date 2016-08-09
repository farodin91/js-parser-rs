use error::JsResult;
use error::error::{Error, ErrorType, SyntaxErrorType};
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
    pub fn expect_identifier(&mut self) -> JsResult<()> {
        println!("expect_identifier {:?}", self.peek());
        match self.peek() {
            Some(TokenType::Identifier(_)) => {
                try!(self.bump());
                Ok(())
            }
            _ => Err(Error::new(ErrorType::SyntaxError(SyntaxErrorType::UnexpectedEOF), 0, 0, None))
        }
    }

    pub fn expect_identifier_name(&mut self) -> JsResult<()> {
        println!("expect_identifier {:?}", self.peek());
        match self.peek() {
            Some(TokenType::Identifier(_)) => {
                try!(self.bump());
                Ok(())
            }
            Some(TokenType::Get) => {
                try!(self.bump());
                Ok(())
            }
            Some(TokenType::Set) => {
                try!(self.bump());
                Ok(())
            }
            _ => Err(Error::new(ErrorType::SyntaxError(SyntaxErrorType::UnexpectedEOF), 0, 0, None))
        }
    }

    pub fn consume_identifier(&mut self) -> JsResult<bool> {
        println!("consume_identifier {:?}", self.peek());
        match self.peek() {
            Some(TokenType::Identifier(_)) => {
                try!(self.bump());
                Ok(true)
            }
            _ => Ok(false)
        }
    }

    pub fn parse_expr_stmt(&mut self) -> JsResult<Item> {
        println!("parse_expr_stmt {:?}", self.peek());
        let result = match self.peek() {
            Some(TokenType::LeftBrace) => Ok(Item::None),
            Some(TokenType::Function) => Ok(Item::None),
            Some(TokenType::Class) => Ok(Item::None),
            Some(TokenType::Let) => {
                if self.peek_at(1) == Some(TokenType::LeftBracket) {
                    Ok(Item::None)
                } else {
                    Ok(Item::Item)
                }
            },
            Some(_) => Ok(Item::Item),
            None => Ok(Item::None),
        };
        none!(try!(result));
        try!(self.parse_expr());
        if try!(self.consume(TokenType::LineTerminate)) {
            return  Ok(Item::Item)
        }
        try!(self.expect(TokenType::Semicolon));
        Ok(Item::Item)
    }

    pub fn parse_yield_expr(&mut self) -> JsResult<Item> {
        println!("parse_yield_expr {:?}", self.peek());
        if !try!(self.consume(TokenType::Yield)) {
            return Ok(Item::None)
        }
        if try!(self.consume(TokenType::Semicolon)) {
            Ok(Item::Item)
        } else {
            try!(self.consume(TokenType::Multiple));
            try!(self.parse_assign_expr());
            try!(self.expect(TokenType::Semicolon));
            Ok(Item::Item)
        }
    }

    pub fn bump_and_return_item(&mut self) -> JsResult<Item> {
        try!(self.bump());
        Ok(Item::Item)
    }

    pub fn parse_primary_expr(&mut self) -> JsResult<Item> {
        println!("parse_primary_expr {:?}", self.peek());
        match self.peek() {
            Some(TokenType::This) => self.bump_and_return_item(),
            Some(TokenType::Identifier(_)) => self.bump_and_return_item(),
            Some(TokenType::Literal(_)) => self.bump_and_return_item(),
            Some(TokenType::LeftBracket) => self.parse_array_literal(),
            Some(TokenType::LeftBrace) => self.parse_object_literal(),
            Some(TokenType::Function) => self.parse_function_expr(),
            Some(TokenType::Yield) => self.parse_yield_expr(),
            Some(TokenType::LeftParen) => self.parse_cover_parenthesized_expression_and_arrow_parameter_list(),
            _ => Ok(Item::None)
        }
    }

    pub fn dump_and_parse_unary_expr(&mut self) -> JsResult<Item> {
        try!(self.bump());
        self.parse_unary_expr(Item::None)
    }

    pub fn parse_unary_expr(&mut self, first: Item) -> JsResult<Item> {
        println!("parse_unary_expr {:?}", self.peek());
        wait!(try!(self.parse_update_expr(first)));
        match self.peek() {
            Some(TokenType::Delete) => self.dump_and_parse_unary_expr(),
            Some(TokenType::Void) => self.dump_and_parse_unary_expr(),
            Some(TokenType::Typeof) => self.dump_and_parse_unary_expr(),
            Some(TokenType::Plus) => self.dump_and_parse_unary_expr(),
            Some(TokenType::Minus) => self.dump_and_parse_unary_expr(),
            Some(TokenType::Tilde) => self.dump_and_parse_unary_expr(),
            Some(TokenType::Invert) => self.dump_and_parse_unary_expr(),
            _ => Ok(Item::None)
        }
    }
    pub fn parse_update_expr(&mut self, first: Item) -> JsResult<Item> {
        println!("parse_update_expr {:?}", self.peek());
        let left = match first {
            Item::Item => Item::Item,
            Item::None => try!(self.parse_left_hand_side_expr())
        };
        match left {
            Item::None => {
                if try!(self.consume(TokenType::Increment)) {
                    return self.parse_unary_expr(Item::None);
                }
                if try!(self.consume(TokenType::Decrement)) {
                    return self.parse_unary_expr(Item::None);
                }
                return Ok(Item::None)
            },
            Item::Item => {
                if try!(self.consume(TokenType::Increment)) {
                    return Ok(Item::Item)
                }
                if try!(self.consume(TokenType::Decrement)) {
                    return Ok(Item::Item)
                }
                return Ok(Item::Item)
            }
        }
    }

    pub fn parse_conditional_expr(&mut self, first: Item) -> JsResult<Item> {
        println!("parse_conditional_expr {:?}", self.peek());
        none!(try!(self.parse_logical_expr(first)));
        if try!(self.consume(TokenType::QuestionMark)) {
            try!(self.parse_assign_expr());
            try!(self.expect(TokenType::Colon));
            try!(self.parse_assign_expr());
        }
        Ok(Item::Item)
    }

    pub fn parse_cover_parenthesized_expression_and_arrow_parameter_list(&mut self) -> JsResult<Item> {
        println!("parse_cover_parenthesized_expression_and_arrow_parameter_list {:?}", self.peek());
        try!(self.expect(TokenType::LeftParen));
        if !try!(self.consume(TokenType::ThreePoints)) {
            try!(self.parse_expr());
            try!(self.consume(TokenType::ThreePoints));
        }
        try!(self.consume_all_lineterminates());
        try!(self.expect(TokenType::RightParen));
        Ok(Item::Item)
    }

    pub fn parse_assign_expr(&mut self) -> JsResult<Item> {
        println!("parse_assign_expr {:?}", self.peek());
        match try!(self.parse_left_hand_side_expr()) {
            Item::Item => {
                println!(" parse_assign_expr {:?}", self.peek());
                match self.peek() {
                    Some(TokenType::DivideAssign) |
                    Some(TokenType::ExpAssign) |
                    Some(TokenType::LeftShiftAssign) |
                    Some(TokenType::ModAssign) |
                    Some(TokenType::PlusAssign) |
                    Some(TokenType::MinusAssign) |
                    Some(TokenType::MultipleAssign) |
                    Some(TokenType::OrBitwiseAssign) |
                    Some(TokenType::XorAssign) |
                    Some(TokenType::AndBitwiseAssign) |
                    Some(TokenType::Equal) => {
                        try!(self.bump());
                        self.parse_assign_expr()
                    }
                    _ => {
                        self.parse_conditional_expr(Item::Item)
                    }
                }
            }
            Item::None => {
                wait!(try!(self.parse_conditional_expr(Item::None)));
                self.parse_yield_expr()
            }
        }
    }

    pub fn parse_expr(&mut self) -> JsResult<Item> {
        try!(self.consume_all_lineterminates());
        println!("parse_expr {:?}", self.peek());
        try!(self.parse_assign_expr());
        while try!(self.consume(TokenType::Comma)) {
            try!(self.consume_all_lineterminates());
            try!(self.parse_assign_expr());
        }
        Ok(Item::Item)
    }
}
