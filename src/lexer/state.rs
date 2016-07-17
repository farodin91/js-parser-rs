use lexer::enums::{TokenType, LexerMode};

pub struct LexerState {
    pub input: Box<Iterator<Item = char>>,
    pub tokens: Vec<TokenType>,
    pub mode: LexerMode,
    pub tmp: String,
    pub escaped: bool,
    pub last: Option<char>
}