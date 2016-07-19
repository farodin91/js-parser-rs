//pub mod error;
pub mod lexer;
pub mod scope;
pub use lexer::enums::TokenType;


pub struct JsContext {}

impl JsContext {
    pub fn new() -> JsContext {
        JsContext {}
    }
}

pub fn parse<T, I>(iter: T) -> Result<Vec<TokenType>,()> where
    T: IntoIterator<Item=char, IntoIter=I> + Sized,
    I: Iterator<Item=char> + 'static {
    lexer::run(Box::new(iter.into_iter()))
}
