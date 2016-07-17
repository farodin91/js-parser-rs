use lexer::enums::{LexerMode, CommentType, TokenType};
use lexer::state::{LexerState};

pub fn exec(state: &mut LexerState, c: Option<char>, t: CommentType) -> bool {
    match (c, t) {
        (Some('\n'), CommentType::SingleLine) => {
            state.tokens.push(TokenType::CommentLiteral(state.tmp.clone()));
            state.tokens.push(TokenType::LineTerminate);
            state.mode = LexerMode::None
        }
        (Some('\n'), _) => {
            state.tokens.push(TokenType::CommentLiteral(state.tmp.clone()));
            state.tokens.push(TokenType::LineTerminate);
            state.mode = LexerMode::None
        }
        (Some('/'), CommentType::MultiLineEnd) => {
            state.mode = LexerMode::None;
            state.tokens.push(TokenType::CommentLiteral(state.tmp.clone()));
        }
        (Some('*'), CommentType::MultiLineStart) => {
            state.mode = LexerMode::Comment(CommentType::MultiLineEnd);
        }
        (Some('*'), CommentType::SingleLine) => {
            state.tmp.push(c.unwrap());
        }
        (Some('*'), CommentType::MultiLineEnd) => {
            state.tmp.push(c.unwrap());
            state.mode = LexerMode::Comment(CommentType::MultiLineEnd);
        }
        (Some('*'), _) => {
            state.mode = LexerMode::Comment(CommentType::MultiLineEnd);
        }
        (Some(c), CommentType::MultiLineEnd) => {
            state.tmp.push('*');
            state.tmp.push(c);
            state.mode = LexerMode::Comment(CommentType::MultiLineNormal);
        }
        (Some(c), CommentType::SingleLine) => {
            state.tmp.push(c);
        }
        (Some(c), _) => {
            state.tmp.push(c);
            state.mode = LexerMode::Comment(CommentType::MultiLineNormal);
        }
        (_, _) => {
            state.mode = LexerMode::EOF;
            state.tokens.push(TokenType::CommentLiteral(state.tmp.clone()));
        }
    }
    true
}

