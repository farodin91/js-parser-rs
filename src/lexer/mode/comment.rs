use lexer::enums::{LexerMode, CommentType, TokenType};
use lexer::state::{LexerState};

impl LexerState {
    fn comment(&mut self) {
        let tmp = self.tmp();
        self.push(TokenType::CommentLiteral(tmp));
        self.update(LexerMode::None);
    }
}

pub fn exec(state: &mut LexerState, c: Option<char>, t: CommentType) -> bool {
    match (c, t) {
        (Some('\n'), CommentType::SingleLine) => {
            state.comment();
            state.push(TokenType::LineTerminate);
        }
        (Some('/'), CommentType::MultiLineEnd) => {
            state.comment();
        }
        (Some('*'), CommentType::MultiLineStart) => {
            state.update(LexerMode::Comment(CommentType::MultiLineEnd));
        }
        (Some('*'), CommentType::SingleLine) => {
            state.tmp_push(c.unwrap());
        }
        (Some('*'), CommentType::MultiLineEnd) => {
            state.tmp_push(c.unwrap());
            state.update(LexerMode::Comment(CommentType::MultiLineEnd));
        }
        (Some('*'), _) => {
            state.update(LexerMode::Comment(CommentType::MultiLineEnd));
        }
        (Some(c), CommentType::MultiLineEnd) => {
            state.tmp_push('*');
            state.tmp_push(c);
            state.update(LexerMode::Comment(CommentType::MultiLineNormal));
        }
        (Some(c), CommentType::SingleLine) => {
            state.tmp_push(c);
        }
        (Some(c), _) => {
            state.tmp_push(c);
            state.update(LexerMode::Comment(CommentType::MultiLineNormal));
        }
        (_, _) => {
            state.update(LexerMode::EOF);
            let tmp = state.tmp();
            state.push(TokenType::CommentLiteral(tmp));
        }
    }
    true
}

