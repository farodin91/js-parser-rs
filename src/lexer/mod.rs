mod mode;
pub mod enums;
pub mod state;

use std::result::Result;
use std::vec::Vec;
use lexer::enums::{ TokenType, LexerMode };
use lexer::state::LexerState;

pub fn run(input: Box<Iterator<Item=char>>) -> Result<Vec<TokenType>,()> {
    let mut state = LexerState { input: input, tokens: Vec::new(), mode: LexerMode::None, tmp: String::new(), escaped: false, last: None };
    loop {
        let c = next(&mut state);
        let mut done = false; // mut done: bool
        while !done {
            let mode = state.mode;
            done = match mode {
                LexerMode::None => mode::none::exec(&mut state, c),
                LexerMode::String => mode::string::exec(&mut state, c),
                LexerMode::Punctuator(t) =>  mode::punctuator::exec(&mut state, c, t),
                LexerMode::Number(t) => mode::number::exec(&mut state, c, t),
                LexerMode::Comment(t) => mode::comment::exec(&mut state, c, t),
                LexerMode::Raw => mode::raw::exec(&mut state, c),
                LexerMode::EOF => true
            }
        }
        if state.mode == LexerMode::EOF {
            break;
        } else {
            state.last = c;
        }
    }
    Ok(state.tokens)
}


fn next(state: &mut LexerState) -> Option<char> {
    state.input.next()
}