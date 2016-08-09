use error::JsResult;
use error::error::{Error, ErrorType, SyntaxErrorType};
use lexer::enums::{TokenType, LiteralType};
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

    pub fn consume_all_lineterminates(&mut self) -> JsResult<()> {
        println!("consume_all_lineterminates {:?}", self.peek());
        while try!(self.consume(TokenType::LineTerminate)) {}
        Ok(())
    }

    pub fn parse_stmt_list(&mut self) -> JsResult<()> {
        loop {
            println!("parse_stmt_list {:?}", self.peek());
            match self.peek() {
                None |
                Some(TokenType::RightBrace) |
                Some(TokenType::Case) |
                Some(TokenType::Default) => return Ok(()),
                _ => {}
            }

            match try!(self.parse_stmt()) {
                Item::Item => (),
                Item::None => {
                    try!(self.parse_declaration());
                },
            }
        }
    }

    pub fn parse_labelled(&mut self) -> JsResult<Item> {
        try!(self.bump());
        try!(self.expect(TokenType::Colon));
        match self.peek() {
            Some(TokenType::Function) => self.parse_function(),
            Some(_) => self.parse_stmt(),
            None => {
                return Err(Error::new(ErrorType::SyntaxError(SyntaxErrorType::UnexpectedEOF), 0, 0, None))
            }
        }
    }

    pub fn parse_block(&mut self) -> JsResult<Item> {
        println!("parse_block {:?}", self.peek());
        try!(self.expect(TokenType::LeftBrace));
        try!(self.parse_stmt_list());
        try!(self.expect(TokenType::RightBrace));
        Ok(Item::Item)
    }

    pub fn parse_empty(&mut self) -> JsResult<Item> {
        println!("parse_empty {:?}", self.peek());
        try!(self.bump());
        Ok(Item::Item)
    }

    pub fn parse_declaration(&mut self) -> JsResult<Item> {
        println!("parse_declaration {:?}", self.peek());
        match self.peek() {
            Some(TokenType::Function) => self.parse_function(),
            Some(TokenType::Class) => self.parse_class(),
            Some(TokenType::Let) => self.parse_let(),
            Some(TokenType::Const) => self.parse_const(),
            Some(TokenType::LineTerminate) => {
                try!(self.bump());
                println!("Warning for LineTerminate");
                Ok(Item::None)
            }
            Some(t) => Err(Error::new(ErrorType::SyntaxError(SyntaxErrorType::Unexpected(t)), 0, 0, None)),
            None => Ok(Item::None)
        }
    }

    pub fn parse_stmt(&mut self) -> JsResult<Item> {
        println!("parse_stmt {:?}", self.peek());
        match self.peek() {
            Some(TokenType::LeftBrace) => self.parse_block(),
            Some(TokenType::Var) => self.parse_variable(),
            Some(TokenType::Semicolon) => self.parse_empty(),
            Some(TokenType::If) => self.parse_if(),

            Some(TokenType::While) => self.parse_while(),// Breakable
            Some(TokenType::Do) => self.parse_do(),// Breakable
            Some(TokenType::For) => self.parse_for(),// Breakable

            Some(TokenType::Switch) => self.parse_switch(),// Breakable

            Some(TokenType::Continue) => self.parse_continue(),
            Some(TokenType::Return) => self.parse_return(),
            Some(TokenType::With) => self.parse_with(),
            Some(TokenType::Break) => self.parse_break(),
            Some(TokenType::Throw) => self.parse_throw(),
            Some(TokenType::Try) => self.parse_try(),
            Some(TokenType::Debugger) => self.parse_debugger(),

            Some(TokenType::This) => self.parse_expr_stmt(),
            Some(TokenType::Increment) => self.parse_expr_stmt(),
            Some(TokenType::Decrement) => self.parse_expr_stmt(),
            Some(TokenType::Delete) => self.parse_expr_stmt(),
            Some(TokenType::LeftParen) => self.parse_expr_stmt(),
            Some(TokenType::Minus) => self.parse_expr_stmt(),
            Some(TokenType::Invert) => self.parse_expr_stmt(),
            Some(TokenType::Plus) => self.parse_expr_stmt(),
            Some(TokenType::Literal(LiteralType::String(_))) => self.parse_expr_stmt(),
            Some(TokenType::Literal(LiteralType::Integer(_))) => self.parse_expr_stmt(),

            Some(TokenType::Identifier(_)) => {
                if Some(TokenType::Colon) == self.peek_at(1) {
                    self.parse_labelled()
                } else {
                    self.parse_expr_stmt()
                }
            }
            Some(TokenType::Yield) => self.parse_yield_expr(),
            Some(_) => Ok(Item::None),
            None => Ok(Item::None)
        }
    }
}