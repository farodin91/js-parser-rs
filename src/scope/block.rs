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
    pub fn parse_stmt_list(&mut self) -> JsResult<()> {
        loop {
            println!("parse_stmt_list {:?}", self.peek());
            match self.peek() {
                None |
                Some(TokenType::Punctuator(Punctuator::RightBrace)) |
                Some(TokenType::Keyword(Keyword::Case)) |
                Some(TokenType::Keyword(Keyword::Default)) => return Ok(()),
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
        try!(self.expect(TokenType::Punctuator(Punctuator::Colon)));
        match self.peek() {
            Some(TokenType::Keyword(Keyword::Function)) => self.parse_function(),
            Some(_) => self.parse_stmt(),
            None => {
                return Err(Error::new(ErrorType::SyntaxError(SyntaxErrorType::UnexpectedEOF), 0, 0, None))
            }
        }
    }

    pub fn parse_block(&mut self) -> JsResult<Item> {
        println!("parse_block {:?}", self.peek());
        try!(self.expect(TokenType::Punctuator(Punctuator::LeftBrace)));
        try!(self.parse_stmt_list());
        try!(self.expect(TokenType::Punctuator(Punctuator::RightBrace)));
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
            Some(TokenType::Keyword(Keyword::Function)) => self.parse_function(),
            Some(TokenType::Keyword(Keyword::Class)) => self.parse_class(),
            Some(TokenType::Keyword(Keyword::Let)) => self.parse_let(),
            Some(TokenType::Keyword(Keyword::Const)) => self.parse_const(),
            Some(t) => Err(Error::new(ErrorType::SyntaxError(SyntaxErrorType::Unexpected(t)), 0, 0, None)),
            None => Ok(Item::None)
        }
    }

    pub fn parse_stmt(&mut self) -> JsResult<Item> {
        println!("parse_stmt {:?}", self.peek());
        match self.peek() {
            Some(TokenType::Punctuator(Punctuator::LeftBrace)) => self.parse_block(),
            Some(TokenType::Keyword(Keyword::Var)) => self.parse_variable(),
            Some(TokenType::Semicolon) => self.parse_empty(),
            Some(TokenType::Keyword(Keyword::If)) => self.parse_if(),

            Some(TokenType::Keyword(Keyword::While)) => self.parse_while(),// Breakable
            Some(TokenType::Keyword(Keyword::Do)) => self.parse_do(),// Breakable
            Some(TokenType::Keyword(Keyword::For)) => self.parse_for(),// Breakable

            Some(TokenType::Keyword(Keyword::Switch)) => self.parse_switch(),// Breakable

            Some(TokenType::Keyword(Keyword::Continue)) => self.parse_continue(),
            Some(TokenType::Keyword(Keyword::Return)) => self.parse_return(),
            Some(TokenType::Keyword(Keyword::With)) => self.parse_with(),
            Some(TokenType::Keyword(Keyword::Throw)) => self.parse_throw(),
            Some(TokenType::Keyword(Keyword::Try)) => self.parse_try(),
            Some(TokenType::Keyword(Keyword::Debugger)) => self.parse_debugger(),
            Some(TokenType::SymbolLiteral(_)) => {
                if Some(TokenType::Punctuator(Punctuator::Colon)) == self.peek_at(1) {
                    self.parse_labelled()
                } else {
                    self.parse_expr_stmt()
                }
            }
            Some(TokenType::Keyword(Keyword::Yield)) => self.parse_yield_expr(),
            Some(_) => Ok(Item::None),
            None => Ok(Item::None)
        }
    }
}