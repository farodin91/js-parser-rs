use lexer::enums::{LexerMode, RegexState, TokenType, RegexIdentifier};
use lexer::state::{LexerState};

impl LexerState {
    fn regex(&mut self, t: RegexIdentifier) {
        let tmp = self.tmp();
        self.push(TokenType::Regex(tmp, t));
        self.update(LexerMode::None);
    }
}

pub fn exec(state: &mut LexerState, c: Option<char>, t: RegexState) -> bool {
    match (c,t) {
        (Some('/'), RegexState::Normal)=> {
            state.update(LexerMode::Regex(RegexState::Identifier));
            true
        }
        (Some('g'), RegexState::Identifier) => {
            state.regex(RegexIdentifier::Global);
            true
        }
        (Some(_), RegexState::Identifier) => {
            state.regex(RegexIdentifier::None);
            false
        }
        (Some(c), RegexState::Normal) => {
            state.tmp_push(c);
            true
        }
        (None,RegexState::Identifier) => {
            state.regex(RegexIdentifier::None);
            state.update(LexerMode::EOF);
            true
        }
        (_,_) => {
            state.update(LexerMode::EOF);
            true
        }
    }
}