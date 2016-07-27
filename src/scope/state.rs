use lexer::token::Token;
use lexer::enums::{TokenType, LiteralType};
use error::error::{Error, ErrorType, SyntaxErrorType, CodePos};

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum CurrentState {
    String,
    None
}

pub struct JsScope {
    state: CurrentState
}
//pub type LexerStateIterator = Box<Iterator<Item = Token>>;

impl JsScope {
    fn new() -> JsScope {
        JsScope {
            state: CurrentState::None
        }
    }

    pub fn from_tokens(tokens: Vec<Token>) -> Result<(), Error> {
        let mut iter = Box::new(tokens.into_iter());
        let scope = &mut JsScope::new();
        loop {
            let result = match iter.next() {
                Some(token) => {
                    scope.handle_token(token)
                }
                None => break
            };

            match result {
                Ok(_) => (),
                Err(err) => { return Err(err) }
            }
        }
        Ok(())
    }

    fn handle_token(&mut self, token: Token) -> Result<(), Error> {
        let (line, col) = token.location();
        let token = token.token;
        match (token.clone(), self.state) {
            (TokenType::Literal(LiteralType::String(_)), CurrentState::String) => {
                return Err(Error::new(ErrorType::SyntaxError(SyntaxErrorType::Unexpected(token)),col, line, None))
            }
            (TokenType::Literal(LiteralType::String(_)), CurrentState::None) => {
                self.state = CurrentState::String
            }
            _ => {
                self.state = CurrentState::None
            }
        }
        Ok(())
    }
}