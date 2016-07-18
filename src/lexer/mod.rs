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
        let c = state.next_char();
        let mut done = false; // mut done: bool
        while !done {
            let mode = state.mode();
            done = match mode {
                LexerMode::None => mode::none::exec(state, c),
                LexerMode::String => mode::string::exec(state, c),
                LexerMode::Punctuator(t, i) => mode::punctuator::exec(state, c, t, i),
                LexerMode::Number(t) => mode::number::exec(state, c, t),
                LexerMode::Comment(t) => mode::comment::exec(state, c, t),
                LexerMode::Raw => mode::raw::exec(state, c),
                LexerMode::Regex(t) => mode::regex::exec(state, c, t),
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