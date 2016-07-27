use lexer::enums::{TokenType};
use error::error::{CodePos};


#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Token {
    pub token: TokenType,
    pub col: u32,
    pub line: u64
}

impl Token {
    pub fn new(token: TokenType, col: u32, line: u64) -> Token {
        Token {
            token: token,
            col: col,
            line: line,
        }
    }
}

impl CodePos for Token {
    fn location(&self) -> (u64, u32) {
        (self.line, self.col)
    }
}
