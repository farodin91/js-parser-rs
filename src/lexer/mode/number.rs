use std::str::FromStr;
use lexer::enums::{LexerMode, NumberType, TokenType, LiteralType};
use lexer::state::{LexerState};

fn token(state: &mut LexerState, t: LiteralType) {
    state.tokens.push(TokenType::Literal(t));
    state.mode = LexerMode::None;
}

pub fn exec(state: &mut LexerState, c: Option<char>, t: NumberType) -> bool {
    match (c, t) {
        (Some('x'), NumberType::None) | (Some('X'), NumberType::None) => {
            state.mode = LexerMode::Number(NumberType::Hex);
            state.tmp = String::new();
            true
        }
        (Some('0' ... '9'), NumberType::None) => {
            state.tmp.push(c.unwrap());
            state.mode = LexerMode::Number(NumberType::NoneLiteral);
            true
        }
        (Some('0' ... '9'), _) => {
            state.tmp.push(c.unwrap());
            true
        }
        (Some('a' ... 'f'), NumberType::Hex) | (Some('A' ... 'F'), NumberType::Hex) => {
            state.tmp.push(c.unwrap());
            true
        }
        (Some('.'), NumberType::None) | (Some('.'), NumberType::NoneLiteral) => {
            state.mode = LexerMode::Number(NumberType::Float);
            state.tmp.push(c.unwrap());
            true
        }
        (_, NumberType::None) | (_, NumberType::NoneLiteral) => {
            let i = i64::from_str_radix(&state.tmp, 10).unwrap();
            token(state, LiteralType::Integer(i));
            false
        }
        (_, NumberType::Hex) => {
            let i = i64::from_str_radix(&state.tmp, 16).unwrap();
            token(state, LiteralType::Integer(i));
            false
        }
        (_, NumberType::Float) => {
            let i = f64::from_str(&state.tmp).unwrap();
            token(state, LiteralType::Float(i));
            false
        }
    }
}
