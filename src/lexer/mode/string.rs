use lexer::enums::{LexerMode, TokenType, LiteralType};
use lexer::state::{LexerState};

pub fn exec(state: &mut LexerState, c: Option<char>) -> bool {
    let escaped = state.is_escaped();
    match (c, escaped) {
        (Some('"'), true) => {
            state.escaped(false);
            state.tmp_push('"');
        }
        (Some('"'), false) => {
            state.update(LexerMode::None);
            let tmp = state.tmp();
            state.push(TokenType::Literal(LiteralType::String(tmp)));
        }
        (Some('\\'), false) => {
            state.escaped(true);
        }
        (Some('\\'), true) => {
            state.escaped(false);
            state.tmp_push('\\');
        }
        (Some(x), _) => {
            state.tmp_push(x)
        }
        (None, _) => {
            println!("Unhandled Parser State Reached: {:?}, {:?}, {:?}", c, state.mode(), state.is_escaped());
            state.update(LexerMode::EOF)
        }
    }
    true
}
