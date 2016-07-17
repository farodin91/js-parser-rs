use lexer::enums::{LexerMode, TokenType, Punctuator, NumberType};
use lexer::state::{LexerState};

fn start_punctuator(state: &mut LexerState, t: Punctuator) {
    state.mode = LexerMode::Punctuator(t, 0);
}

pub fn exec(state: &mut LexerState, c: Option<char>) -> bool {
    match c {
        Some('a' ... 'z') | Some('A' ... 'Z') | Some('_') | Some('$') => {
            state.mode = LexerMode::Raw;
            state.tmp = String::new();
            state.tmp.push(c.unwrap());
        }
        Some('"') => {
            state.mode = LexerMode::String;
            state.tmp = String::new();
        }
        Some('0') => {
            state.mode = LexerMode::Number(NumberType::None);
            state.tmp = String::new();
            state.tmp.push(c.unwrap());
        }
        Some('1'...'9') => {
            state.mode = LexerMode::Number(NumberType::NoneLiteral);
            state.tmp = String::new();
            state.tmp.push(c.unwrap());
        }
        Some(' ') => (),
        Some('\n') => state.tokens.push(TokenType::LineTerminate),
        Some(';') => state.tokens.push(TokenType::Semicolon),
        Some(',') => state.tokens.push(TokenType::Comma),
        Some('{') => state.tokens.push(TokenType::Punctuator(Punctuator::LeftBrace)),
        Some('}') => state.tokens.push(TokenType::Punctuator(Punctuator::RightBrace)),
        Some('[') => state.tokens.push(TokenType::Punctuator(Punctuator::LeftSquaredBrace)),
        Some(']') => state.tokens.push(TokenType::Punctuator(Punctuator::RightSquaredBrace)),
        Some('(') => state.tokens.push(TokenType::Punctuator(Punctuator::LeftRoundedBrace)),
        Some(')') => state.tokens.push(TokenType::Punctuator(Punctuator::RightRoundedBrace)),
        Some('.') => start_punctuator(state, Punctuator::Point),
        Some('~') => state.tokens.push(TokenType::Punctuator(Punctuator::Tilde)),
        Some(':') => state.tokens.push(TokenType::Punctuator(Punctuator::DoublePoint)),
        Some('?') => state.tokens.push(TokenType::Punctuator(Punctuator::If)),
        Some('|') => start_punctuator(state, Punctuator::OrBitwise),
        Some('*') => start_punctuator(state, Punctuator::Multiple),
        Some('&') => start_punctuator(state, Punctuator::AndBitwise),
        Some('^') => start_punctuator(state, Punctuator::Xor),
        Some('+') => start_punctuator(state, Punctuator::Plus),
        Some('-') => start_punctuator(state, Punctuator::Minus),
        Some('%') => start_punctuator(state, Punctuator::Mod),
        Some('=') => start_punctuator(state, Punctuator::Equal),
        Some('<') => start_punctuator(state, Punctuator::SmallThan),
        Some('/') => start_punctuator(state, Punctuator::Divide),
        Some('!') => start_punctuator(state, Punctuator::Invert),
        Some('>') => start_punctuator(state, Punctuator::GreaterThan),
        None => {
            state.mode = LexerMode::EOF
        }
        _ => {
            state.mode = LexerMode::EOF;
            println!("Unhandled Parser State Reached: {:?}, {:?}, {:?}", c, state.mode, state.escaped);
        }
    }
    true
}