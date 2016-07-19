mod mode;
pub mod enums;
pub mod state;

use std::result::Result;
use std::vec::Vec;
use lexer::enums::{TokenType, LexerMode};
use lexer::state::{LexerState, LexerStateIterator};

pub fn run(input: LexerStateIterator) -> Result<Vec<TokenType>, ()> {
    let state = &mut LexerState::new(input);
    loop {
        state.next_char();
        let mut done = false; // mut done: bool
        while !done {
            let mode = state.mode();
            let c = state.current_char();
            done = match mode {
                LexerMode::None => state.parse_normal(c),
                LexerMode::String(_) => state.parse_string(),
                LexerMode::Punctuator(t, i) => state.parse_punctuator(c, t, i),
                LexerMode::Number(_) => state.parse_number(),
                LexerMode::Comment(t) => state.parse_comment(c, t),
                LexerMode::Raw => state.parse_raw(),
                LexerMode::Regex(_) => state.parse_regex(),
                LexerMode::EOF => true
            }
        }
        if state.mode() == LexerMode::EOF {
            break;
        }
    }
    let tokens = state.tokens();
    Ok(tokens)
}