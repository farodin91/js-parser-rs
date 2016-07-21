//pub mod error;
pub mod lexer;
pub mod scope;
pub mod error;

pub use lexer::enums::TokenType;
pub use error::error::Error;

pub struct JsContext {}

impl JsContext {
    pub fn new() -> JsContext {
        JsContext {}
    }
}

use lexer::state::{LexerState};

pub fn parse<T, I>(iter: T) -> Result<Vec<TokenType>, Error> where
    T: IntoIterator<Item = char, IntoIter = I> + Sized,
    I: Iterator<Item = char> + 'static {
    let state = &mut LexerState::new(Box::new(iter.into_iter()));
    state.parse()
}
