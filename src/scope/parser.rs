use lexer::token::Token;
use lexer::enums::{TokenType};
use error::JsResult;
use error::error::{Error, ErrorType, SyntaxErrorType};
use std::iter::Peekable;
use std::vec::IntoIter;

pub type TokenPeekable = Peekable<Box<IntoIter<Token>>>;

struct Scope {}

#[derive(PartialEq, Debug, Clone)]
pub enum Item {
    Item,
    None
}

pub struct Parser {
    tokens: Vec<Token>,
    len: usize,
    index: usize,
    scopes: Vec<Scope>,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens.clone(),
            len: tokens.len(),
            index: 0,
            scopes: Vec::new()
        }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(Scope {})
    }

    pub fn bump(&mut self) -> JsResult<()> {
        println!("bump {:?}", self.peek());
        self.index += 1;
        if self.index > self.len {
            Err(Error::new(ErrorType::SyntaxError(SyntaxErrorType::UnexpectedEOF), 0, 0, None))
        } else {
            Ok(())
        }
    }

    pub fn next(&mut self) -> JsResult<Token> {
        println!("next {:?}", self.peek());
        self.index += 1;
        if self.index > self.len {
            Err(Error::new(ErrorType::SyntaxError(SyntaxErrorType::UnexpectedEOF), 0, 0, None))
        } else {
            Ok(self.tokens[self.index - 1].clone())
        }
    }

    pub fn fatal(&mut self, error: SyntaxErrorType) -> JsResult<()> {
        return Err(Error::new(ErrorType::SyntaxError(error), 0, 0, None))
    }


    pub fn expect(&mut self, token: TokenType) -> JsResult<()> {
        let next = try!(self.next());
        println!("expected: {:?} == {:?}", token.clone(), next.clone());

        if next.token == token {
            return Ok(());
        }

        self.fatal(SyntaxErrorType::Unexpected(next.token))
    }

    pub fn consume(&mut self, token: TokenType) -> JsResult<bool> {
        println!("consume: {:?} == {:?}", token.clone(), self.peek());
        let matched = match self.peek() {
            None => false,
            Some(t) => t == token
        };

        if matched {
            try!(self.bump());
        }

        Ok(matched)
    }

    pub fn peek(&mut self) -> Option<TokenType> {
        self.peek_at(0)
    }

    pub fn peek_at(&mut self, index: usize) -> Option<TokenType> {
        if self.index + index >= self.len {
            None
        } else {
            let token = self.tokens[self.index + index].clone();
            Some(token.token)
        }
    }

    pub fn from_tokens(tokens: Vec<Token>) -> JsResult<()> {
        let parser = &mut Parser::new(tokens);
        parser.push_scope();
        parser.parse_stmt_list()
    }

    pub fn parse_variable(&mut self) -> JsResult<Item> {
        println!("parse_variable {:?}", self.peek());
        try!(self.bump());

        try!(self.parse_variable_declaration());
        while try!(self.consume(TokenType::Comma)) {
            try!(self.parse_variable_declaration());
        }
        if !try!(self.consume(TokenType::LineTerminate)) {
            try!(self.expect(TokenType::Semicolon));
        }
        Ok(Item::Item)
    }

    pub fn parse_initializer(&mut self) -> JsResult<Item> {
        println!("parse_initializer {:?}", self.peek());
        if !try!(self.consume(TokenType::Equal)) {
            Ok(Item::None)
        } else {
            self.parse_assign_expr()
        }
    }

    pub fn parse_variable_declaration(&mut self) -> JsResult<Item> {
        println!("parse_variable_declaration {:?}", self.peek());
        match self.peek() {
            Some(TokenType::Identifier(_)) => {
                try!(self.bump());

                return self.parse_initializer()
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
    pub fn parse_continue(&mut self) -> JsResult<Item> {
        try!(self.bump());
        if try!(self.consume(TokenType::Semicolon)) {
            Ok(Item::Item)
        } else {
            match self.peek() {
                Some(TokenType::Identifier(_)) => (),
                Some(t) => {
                    return Err(Error::new(ErrorType::SyntaxError(SyntaxErrorType::Unexpected(t)), 0, 0, None))
                }
                None => {
                    return Err(Error::new(ErrorType::SyntaxError(SyntaxErrorType::UnexpectedEOF), 0, 0, None))
                }
            }
            try!(self.expect(TokenType::Semicolon));
            Ok(Item::Item)
        }
    }

    pub fn parse_return(&mut self) -> JsResult<Item> {
        try!(self.bump());
        if try!(self.consume(TokenType::Semicolon)) {
            Ok(Item::Item)
        } else {
            let result = try!(self.parse_expr());
            match self.peek() {
                Some(TokenType::Semicolon) => try!(self.bump()),
                Some(TokenType::LineTerminate) => try!(self.bump()),
                Some(TokenType::RightBrace) => (),
                _ => ()
            }
            Ok(result)
        }
    }

    pub fn parse_with(&mut self) -> JsResult<Item> {
        try!(self.bump());
        try!(self.expect(TokenType::LeftParen));
        try!( self.parse_expr());
        try!( self.expect(TokenType::RightParen));
        self.parse_stmt()
    }

    pub fn parse_do(&mut self) -> JsResult<Item> {
        try!(self.bump());
        Ok(Item::Item)
    }

    pub fn parse_while(&mut self) -> JsResult<Item> {
        try!(self.bump());
        try!(self.expect(TokenType::LeftParen));
        try!(self.parse_expr());
        try!(self.expect(TokenType::RightParen));
        self.parse_stmt()
    }

    pub fn parse_for(&mut self) -> JsResult<Item> {
        try!(self.bump());
        Ok(Item::Item)
    }

    pub fn parse_debugger(&mut self) -> JsResult<Item> {
        try!(self.bump());
        try!(self.expect(TokenType::Semicolon));
        Ok(Item::Item)
    }

    pub fn parse_break(&mut self) -> JsResult<Item> {
        try!(self.bump());
        try!(self.consume_identifier());
        try!(self.expect(TokenType::Semicolon));
        Ok(Item::Item)
    }

    pub fn parse_class(&mut self) -> JsResult<Item> {
        try!(self.bump());
        Ok(Item::Item)
    }

    pub fn parse_binding_list(&mut self) -> JsResult<Item> {
        Ok(Item::Item)
    }

    pub fn parse_let(&mut self) -> JsResult<Item> {
        try!(self.bump());
        self.parse_binding_list()
    }

    pub fn parse_const(&mut self) -> JsResult<Item> {
        try!(self.bump());
        self.parse_binding_list()
    }

    pub fn parse_if(&mut self) -> JsResult<Item> {
        println!("parse_if {:?}", self.peek());
        try!(self.bump());
        try!(self.expect(TokenType::LeftParen));
        try!(self.parse_expr());
        try!(self.expect(TokenType::RightParen));

        let then = try!(self.parse_stmt());
        match then {
            Item::Item => (),
            Item::None => try!(self.fatal(SyntaxErrorType::UnexpectedEOF))
        }

        if try!(self.consume(TokenType::Else)) {
            try!(self.parse_stmt());
        }
        Ok(Item::Item)
    }

    pub fn parse_element_list(&mut self) -> JsResult<Item> {
        if !try!(self.consume(TokenType::ThreePoints)) {
            try!(self.parse_expr());
            try!(self.consume(TokenType::ThreePoints));
        }
        Ok(Item::Item)
    }

    pub fn parse_array_literal(&mut self) -> JsResult<Item> {
        println!("parse_array_literal {:?}", self.peek());
        try!(self.expect(TokenType::LeftBracket));
        try!(self.parse_element_list());
        try!(self.expect(TokenType::RightBracket));
        Ok(Item::Item)
    }
}