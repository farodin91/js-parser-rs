
use lexer::enums::{LexerMode, Operation, TokenType};
use lexer::state::{ LexerState };

pub fn exec(state: &mut LexerState, c: Option<char>, t: Operation) -> bool {
    match (c, t) {
        (Some('\n'), Operation::End) => {
            state.tmp.push('*');
            state.tokens.push(TokenType::CommentLiteral(state.tmp.clone()));
            state.mode = LexerMode::None
        }
        (Some('\n'), _) => {
            state.tokens.push(TokenType::CommentLiteral(state.tmp.clone()));
            state.mode = LexerMode::None
        }
        (Some('/'), Operation::End) => {
            state.mode = LexerMode::None;
            state.tokens.push(TokenType::CommentLiteral(state.tmp.clone()));
        }
        (Some('*'), Operation::Start) => {
            state.mode = LexerMode::Comment(Operation::End);
        }
        (Some('*'), Operation::End) => {
            state.tmp.push('*');
        }
        (Some('*'), _) => {
            state.mode = LexerMode::Comment(Operation::End);
        }
        (Some(c), Operation::End) => {
            state.tmp.push('*');
            state.tmp.push(c);
            state.mode = LexerMode::Comment(Operation::Normal);
        }
        (Some(c), _) => {
            state.tmp.push(c);
            state.mode = LexerMode::Comment(Operation::Normal);
        }
        (_,_)=> {
            state.mode = LexerMode::EOF;
            state.tokens.push(TokenType::CommentLiteral(state.tmp.clone()));
        }
    }
    true
}

