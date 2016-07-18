use lexer::enums::{LexerMode, NumberType, TokenType, LiteralType};
use lexer::state::{LexerState};
use std::str::FromStr;

impl LexerState {
    fn number(&mut self, t: LiteralType) {
        self.push(TokenType::Literal(t));
        self.update(LexerMode::None);
    }
}

pub fn exec(state: &mut LexerState, c: Option<char>, t: NumberType) -> bool {
    match (c, t) {
        (Some('x'), NumberType::None) | (Some('X'), NumberType::None) => {
            state.update(LexerMode::Number(NumberType::Hex));
            state.reset_tmp();
            true
        }
        (Some('0' ... '9'), NumberType::None) => {
            state.update(LexerMode::Number(NumberType::NoneLiteral));
            state.tmp_push(c.unwrap());
            true
        }
        (Some('0' ... '9'), _) => {
            state.tmp_push(c.unwrap());
            true
        }
        (Some('a' ... 'f'), NumberType::Hex) | (Some('A' ... 'F'), NumberType::Hex) => {
            state.tmp_push(c.unwrap());
            true
        }
        (Some('.'), NumberType::None) | (Some('.'), NumberType::NoneLiteral) => {
            state.update(LexerMode::Number(NumberType::Float));
            state.tmp_push(c.unwrap());
            true
        }
        (_, NumberType::None) | (_, NumberType::NoneLiteral) => {
            let i = i64::from_str_radix(&state.tmp(), 10).unwrap();
            state.number(LiteralType::Integer(i));
            false
        }
        (_, NumberType::Hex) => {
            let i = i64::from_str_radix(&state.tmp(), 16).unwrap();
            state.number(LiteralType::Integer(i));
            false
        }
        (_, NumberType::Float) => {
            let i = f64::from_str(&state.tmp()).unwrap();
            state.number(LiteralType::Float(i));
            false
        }
    }
}
