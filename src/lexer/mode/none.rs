use lexer::enums::{LexerMode, TokenType, Punctuator, NumberType, RegexState};
use lexer::state::{LexerState};

impl LexerState {
    fn start_punctuator(&mut self, t: Punctuator) {
        self.update(LexerMode::Punctuator(t, 0));
    }
}

pub fn exec(state: &mut LexerState, c: Option<char>) -> bool {
    match c {
        Some('a' ... 'z') | Some('A' ... 'Z') | Some('_') | Some('$') => {
            state.update(LexerMode::Raw);
            state.reset_tmp();
            state.tmp_push(c.unwrap());
        }
        Some('"') => {
            state.update(LexerMode::String);
            state.reset_tmp();
        }
        Some('0') => {
            state.update(LexerMode::Number(NumberType::None));
            state.reset_tmp();
            state.tmp_push(c.unwrap());
        }
        Some('1'...'9') => {
            state.update(LexerMode::Number(NumberType::NoneLiteral));
            state.reset_tmp();
            state.tmp_push(c.unwrap());
        }
        Some(' ') => (),
        Some('\n') => state.push(TokenType::LineTerminate),
        Some(';') => state.push(TokenType::Semicolon),
        Some(',') => state.push(TokenType::Comma),
        Some('{') => state.push(TokenType::Punctuator(Punctuator::LeftBrace)),
        Some('}') => state.push(TokenType::Punctuator(Punctuator::RightBrace)),
        Some('[') => state.push(TokenType::Punctuator(Punctuator::LeftBracket)),
        Some(']') => state.push(TokenType::Punctuator(Punctuator::RightBracket)),
        Some('(') => state.push(TokenType::Punctuator(Punctuator::LeftParen)),
        Some(')') => state.push(TokenType::Punctuator(Punctuator::RightParen)),
        Some('~') => state.push(TokenType::Punctuator(Punctuator::Tilde)),
        Some(':') => state.push(TokenType::Punctuator(Punctuator::DoublePoint)),
        Some('?') => state.push(TokenType::Punctuator(Punctuator::QuestionMark)),
        Some('.') => state.start_punctuator(Punctuator::Point),
        Some('|') => state.start_punctuator(Punctuator::OrBitwise),
        Some('*') => state.start_punctuator(Punctuator::Multiple),
        Some('&') => state.start_punctuator(Punctuator::AndBitwise),
        Some('^') => state.start_punctuator(Punctuator::Xor),
        Some('+') => state.start_punctuator(Punctuator::Plus),
        Some('-') => state.start_punctuator(Punctuator::Minus),
        Some('%') => state.start_punctuator(Punctuator::Mod),
        Some('=') => state.start_punctuator(Punctuator::Equal),
        Some('<') => state.start_punctuator(Punctuator::SmallThan),
        Some('/') => {
            let last_token = state.last_token();
            match last_token {
                Some(TokenType::Punctuator(Punctuator::DoublePoint)) |
                Some(TokenType::Punctuator(Punctuator::Equal)) |
                Some(TokenType::Comma) => {
                    state.update(LexerMode::Regex(RegexState::Normal));
                    state.reset_tmp()
                }
                _ => {
                    state.start_punctuator(Punctuator::Divide)
                }
            }
        },
        Some('!') => state.start_punctuator(Punctuator::Invert),
        Some('>') => state.start_punctuator(Punctuator::GreaterThan),
        None => {
            state.update(LexerMode::EOF)
        }
        _ => {
            state.update(LexerMode::EOF);
            println!("Unhandled Parser State Reached: {:?}, {:?}, {:?}", c, state.mode(), state.is_escaped());
        }
    }
    true
}