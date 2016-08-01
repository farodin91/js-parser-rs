use error::JsResult;
use error::error::{Error, ErrorType, SyntaxErrorType};
use lexer::enums::{TokenType, Keyword, Punctuator};
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
    pub fn parse_function_expr(&mut self) -> JsResult<Item> {
        Ok(Item::Item)
    }

    pub fn expect_identifier(&mut self) -> JsResult<()> {
        match self.peek() {
            Some(TokenType::SymbolLiteral(_)) => {
                try!(self.bump());
                Ok(())
            }
            _ => Err(Error::new(ErrorType::SyntaxError(SyntaxErrorType::UnexpectedEOF), 0, 0, None))
        }
    }

    pub fn parse_super_prop(&mut self) -> JsResult<Item> {
        try!(self.bump());
        if try!(self.consume(TokenType::Punctuator(Punctuator::LeftBracket))) {
            try!(self.expect(TokenType::Punctuator(Punctuator::RightBracket)));
        } else {
            try!(self.expect(TokenType::Punctuator(Punctuator::Point)));
            try!(self.expect_identifier());
        }
        Ok(Item::Item)
    }

    pub fn parse_meta_prop(&mut self) -> JsResult<Item> {
        try!(self.bump());
        try!(self.expect(TokenType::Punctuator(Punctuator::Point)));
        try!(self.expect(TokenType::Keyword(Keyword::Target)));
        Ok(Item::Item)
    }

    pub fn parse_new_expr(&mut self) -> JsResult<Item> {
        println!("parse_new_expr {:?}", self.peek());
        wait!(try!(self.parse_member_expr()));
        if try!(self.consume(TokenType::Keyword(Keyword::New))) {
            try!(self.parse_new_expr());
        }
        Ok(Item::None)
    }

    pub fn parse_member_expr(&mut self) -> JsResult<Item> {
        println!("parse_left_hand_side_expr {:?}", self.peek());
        wait!(try!(self.parse_primary_expr()));
        match self.peek() {
            Some(TokenType::Keyword(Keyword::Super)) => self.parse_super_prop(),
            Some(TokenType::Keyword(Keyword::New)) => Ok(Item::Item),
            Some(_) => Ok(Item::None),
            None => Ok(Item::None)
        }
    }

    pub fn parse_call_expr(&mut self) -> JsResult<Item> {
        println!("parse_call_expr {:?}", self.peek());
        wait!(try!(self.parse_member_expr()));
        if try!(self.consume(TokenType::Keyword(Keyword::New))) {
            try!(self.parse_new_expr());
        }
        Ok(Item::None)
    }

    pub fn parse_left_hand_side_expr(&mut self) -> JsResult<Item> {
        println!("parse_left_hand_side_expr {:?}", self.peek());
        wait!(try!(self.parse_new_expr()));
        self.parse_call_expr()
    }

    pub fn parse_expr_stmt(&mut self) -> JsResult<Item> {
        println!("parse_expr_stmt {:?}", self.peek());
        let result = match self.peek() {
            Some(TokenType::Punctuator(Punctuator::LeftBrace)) => Ok(Item::None),
            Some(TokenType::Keyword(Keyword::Function)) => Ok(Item::None),
            Some(TokenType::Keyword(Keyword::Class)) => Ok(Item::None),
            Some(TokenType::Keyword(Keyword::Let)) => {
                if self.peek_at(1) == Some(TokenType::Punctuator(Punctuator::LeftBracket)) {
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
        try!(self.expect(TokenType::Semicolon));
        Ok(Item::Item)
    }

    pub fn parse_yield_expr(&mut self) -> JsResult<Item> {
        println!("parse_yield_expr {:?}", self.peek());
        if !try!(self.consume(TokenType::Keyword(Keyword::Yield))) {
            return Ok(Item::None)
        }
        if try!(self.consume(TokenType::Semicolon)) {
            Ok(Item::Item)
        } else {
            try!(self.consume(TokenType::Punctuator(Punctuator::Multiple)));
            self.parse_assign_expr()
        }
    }

    pub fn bump_and_return_item(&mut self) -> JsResult<Item> {
        try!(self.bump());
        Ok(Item::Item)
    }

    pub fn parse_primary_expr(&mut self) -> JsResult<Item> {
        println!("parse_primary_expr {:?}", self.peek());
        match self.peek() {
            Some(TokenType::Keyword(Keyword::This)) => self.bump_and_return_item(),
            Some(TokenType::SymbolLiteral(_)) => self.bump_and_return_item(),
            Some(TokenType::Literal(_)) => self.bump_and_return_item(),
            Some(TokenType::Punctuator(Punctuator::LeftBracket)) => self.parse_array_literal(),
            Some(TokenType::Punctuator(Punctuator::LeftBrace)) => self.parse_object_literal(),
            Some(TokenType::Keyword(Keyword::Function)) => self.parse_function_expr(),
            Some(TokenType::Keyword(Keyword::Yield)) => self.parse_yield_expr(),
            Some(TokenType::Punctuator(Punctuator::LeftParen)) => self.parse_cover_parenthesized_expression_and_arrow_parameter_list(),
            _ => Ok(Item::None)
        }
    }

    pub fn dump_and_parse_unary_expr(&mut self) -> JsResult<Item> {
        try!(self.bump());
        self.parse_unary_expr()
    }

    pub fn parse_unary_expr(&mut self) -> JsResult<Item> {
        println!("parse_unary_expr {:?}", self.peek());
        wait!(try!(self.parse_update_expr()));
        match self.peek() {
            Some(TokenType::Keyword(Keyword::Delete)) => self.dump_and_parse_unary_expr(),
            Some(TokenType::Keyword(Keyword::Void)) => self.dump_and_parse_unary_expr(),
            Some(TokenType::Keyword(Keyword::Typeof)) => self.dump_and_parse_unary_expr(),
            Some(TokenType::Punctuator(Punctuator::Plus)) => self.dump_and_parse_unary_expr(),
            Some(TokenType::Punctuator(Punctuator::Minus)) => self.dump_and_parse_unary_expr(),
            Some(TokenType::Punctuator(Punctuator::Tilde)) => self.dump_and_parse_unary_expr(),
            Some(TokenType::Punctuator(Punctuator::Invert)) => self.dump_and_parse_unary_expr(),
            _ => Ok(Item::None)
        }
    }
    pub fn parse_update_expr(&mut self) -> JsResult<Item> {
        println!("parse_update_expr {:?}", self.peek());
        match try!(self.parse_left_hand_side_expr()) {
            Item::None => {
                if try!(self.consume(TokenType::Punctuator(Punctuator::Increment))) {
                    return self.parse_unary_expr();
                }
                if try!(self.consume(TokenType::Punctuator(Punctuator::Decrement))) {
                    return self.parse_unary_expr();
                }
                return Ok(Item::None)
            },
            Item::Item => {
                if try!(self.consume(TokenType::Punctuator(Punctuator::Increment))) {
                    return Ok(Item::Item)
                }
                if try!(self.consume(TokenType::Punctuator(Punctuator::Decrement))) {
                    return Ok(Item::Item)
                }
                return Ok(Item::Item)
            }
        }
    }

    pub fn dump_and_parse_logical_expr(&mut self) -> JsResult<Item> {
        try!(self.bump());
        self.parse_logical_expr()
    }

    pub fn parse_logical_expr(&mut self) -> JsResult<Item> {
        println!("parse_logical_expr {:?}", self.peek());
        none!(try!(self.parse_unary_expr()));
        match self.peek() {
            Some(TokenType::Punctuator(Punctuator::Plus)) => self.dump_and_parse_logical_expr(),
            Some(TokenType::Punctuator(Punctuator::Minus)) => self.dump_and_parse_logical_expr(),
            Some(TokenType::Punctuator(Punctuator::Exp)) => self.dump_and_parse_logical_expr(),
            Some(TokenType::Punctuator(Punctuator::LeftShift)) => self.dump_and_parse_logical_expr(),
            Some(TokenType::Punctuator(Punctuator::RightShift)) => self.dump_and_parse_logical_expr(),
            Some(TokenType::Punctuator(Punctuator::RightShiftUnsigned)) => self.dump_and_parse_logical_expr(),
            Some(TokenType::Punctuator(Punctuator::GreaterThan)) => self.dump_and_parse_logical_expr(),
            Some(TokenType::Punctuator(Punctuator::GreaterAndEqualThan)) => self.dump_and_parse_logical_expr(),
            Some(TokenType::Punctuator(Punctuator::SmallAndEqualThan)) => self.dump_and_parse_logical_expr(),
            Some(TokenType::Punctuator(Punctuator::SmallThan)) => self.dump_and_parse_logical_expr(),
            Some(TokenType::Keyword(Keyword::Instanceof)) => self.dump_and_parse_logical_expr(),
            Some(TokenType::Keyword(Keyword::In)) => self.dump_and_parse_logical_expr(),
            Some(TokenType::Punctuator(Punctuator::IsEqual)) => self.dump_and_parse_logical_expr(),
            Some(TokenType::Punctuator(Punctuator::IsNotEqual)) => self.dump_and_parse_logical_expr(),
            Some(TokenType::Punctuator(Punctuator::IsSame)) => self.dump_and_parse_logical_expr(),
            Some(TokenType::Punctuator(Punctuator::IsNotSame)) => self.dump_and_parse_logical_expr(),
            Some(TokenType::Punctuator(Punctuator::And)) => self.dump_and_parse_logical_expr(),
            Some(TokenType::Punctuator(Punctuator::AndBitwise)) => self.dump_and_parse_logical_expr(),
            Some(TokenType::Punctuator(Punctuator::Or)) => self.dump_and_parse_logical_expr(),
            Some(TokenType::Punctuator(Punctuator::OrBitwise)) => self.dump_and_parse_logical_expr(),
            Some(TokenType::Punctuator(Punctuator::Xor)) => self.dump_and_parse_logical_expr(),
            _=> Ok(Item::None)
        }
    }

    pub fn parse_conditional_expr(&mut self) -> JsResult<Item> {
        println!("parse_conditional_expr {:?}", self.peek());
        match try!(self.parse_logical_expr()) {
            Item::None => Ok(Item::None),
            Item::Item => {
                if try!(self.consume(TokenType::Punctuator(Punctuator::QuestionMark))) {
                    try!(self.parse_assign_expr());
                    try!(self.expect(TokenType::Punctuator(Punctuator::Colon)));
                    try!(self.parse_assign_expr());
                }
                Ok(Item::Item)
            }
        }
    }

    pub fn parse_assign_expr(&mut self) -> JsResult<Item> {
        println!("parse_assign_expr {:?}", self.peek());

        wait!(try!(self.parse_conditional_expr()));
        wait!(try!(self.parse_yield_expr()));
        wait!(try!(self.parse_left_hand_side_expr()));

        Ok(Item::None)
    }

    pub fn parse_expr(&mut self) -> JsResult<Item> {
        println!("parse_expr {:?}", self.peek());
        try!(self.parse_assign_expr());
        while try!(self.consume(TokenType::Comma)) {
            try!(self.parse_assign_expr());
        }
        Ok(Item::Item)
    }
}
