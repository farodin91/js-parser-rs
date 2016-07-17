
use lexer::enums::{ LexerMode, TokenType, LiteralType };
use lexer::state::{ LexerState };

pub fn exec(state: &mut LexerState, c: Option<char>) -> bool {
    let escaped = state.escaped;
    match (c, escaped) {
        (Some('"'), true) => {
            state.escaped = false;
            state.tmp.push('"');
        }
        (Some('"'), false) => {
            state.mode = LexerMode::None;
            state.tokens.push(TokenType::Literal(LiteralType::String(state.tmp.clone())));
        }
        (Some('\\'), false) => {
            state.escaped = true;
        }
        (Some('\\'), true) => {
            state.escaped = false;
            state.tmp.push('\\');
        }
        (Some(x), _) => {
            state.tmp.push(x)
        }
        (None, _) => {
            println!("Unhandled Parser State Reached: {:?}, {:?}, {:?}", c, state.mode, state.escaped);
            state.mode = LexerMode::EOF
        }
    }
    true
}
