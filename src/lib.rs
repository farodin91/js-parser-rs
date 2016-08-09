//pub mod error;
pub mod lexer;
pub mod scope;
pub mod error;

use lexer::enums::TokenType;
use error::error::{ErrorType};
use lexer::state::{LexerState};
use scope::parser::Parser;

pub struct JsContext {}

struct OwningChars {
    s: String,
    pos: usize
}

impl OwningChars {
    pub fn new(s: String) -> OwningChars {
        OwningChars { s: s, pos: 0 }
    }
}

impl Iterator for OwningChars {
    type Item = char;
    fn next(&mut self) -> Option<char> {
        if let Some(c) = self.s[self.pos..].chars().next() {
            self.pos += c.len_utf8();
            Some(c)
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.s.len() - self.pos;
        ((len + 3) / 4, Some(len)) // see the Chars impl for detail
    }
}

impl JsContext {
    pub fn new() -> JsContext {
        JsContext {}
    }

    pub fn parse(&mut self, str: String) -> Result<(), ErrorType> {
        let chars = OwningChars::new(str);
        let state = &mut LexerState::new(Box::new(chars.into_iter()));
        match state.parse() {
            Ok(_)=> (),
            Err(err) => return Err(err.error_type),
        }
        let tokens = state.tokens();
        match Parser::from_tokens(tokens) {
            Ok(_)=> Ok(()),
            Err(err) => Err(err.error_type)
        }
    }
}


pub fn parse<T, I>(iter: T) -> Result<Vec<TokenType>, ErrorType> where
    T: IntoIterator<Item = char, IntoIter = I> + Sized,
    I: Iterator<Item = char> + 'static {
    let state = &mut LexerState::new(Box::new(iter.into_iter()));
    match state.parse() {
        Ok(_)=> (),
        Err(err) => return Err(err.error_type),
    }
    Ok(state.tokens().iter().map(|token| { token.clone().token }).collect())
}
