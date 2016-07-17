use lexer::enums::{LexerMode, TokenType, Punctuator, CommentType};
use lexer::state::{LexerState};

fn token(state: &mut LexerState, t: Punctuator) {
    state.tokens.push(TokenType::Punctuator(t));
    state.mode = LexerMode::None;
}

fn mode(state: &mut LexerState, t: Punctuator, i: i32) {
    state.mode = LexerMode::Punctuator(t, i);
}

pub fn exec(state: &mut LexerState, c: Option<char>, t: Punctuator, i: i32) -> bool {
    match (c, t) {
        (Some('<'), Punctuator::SmallThan) => {
            mode(state, Punctuator::LeftShift, 0);
            true
        }
        (Some('>'), Punctuator::GreaterThan) => {
            mode(state, Punctuator::RightShift, 0);
            true
        }
        (Some('>'), Punctuator::RightShift) => {
            mode(state, Punctuator::RightShiftUnsigned, 0);
            true
        }
        (Some('+'), Punctuator::Plus) => {
            token(state, Punctuator::Increment);
            true
        }
        (Some('>'), Punctuator::Equal) => {
            token(state, Punctuator::Lamda);
            true
        }
        (Some('.'), Punctuator::Point) => {
            if i == 1 {
                token(state, Punctuator::ThreePoints)
            } else {
                mode(state, Punctuator::Point, 1);
            }
            true
        }
        (_, Punctuator::Point) => {
            if i == 1 {
                token(state, Punctuator::Point);
            }
            token(state, Punctuator::Point);
            false
        }
        (Some('='), Punctuator::RightShiftUnsigned) => {
            token(state, Punctuator::RightShiftUnsignedEq);
            true
        }
        (Some('='), Punctuator::GreaterThan) => {
            token(state, Punctuator::GreaterAndEqualThan);
            true
        }
        (Some('='), Punctuator::SmallThan) => {
            token(state, Punctuator::SmallAndEqualThan);
            true
        }
        (Some('='), Punctuator::Equal) => {
            mode(state, Punctuator::IsEqual, 0);
            true
        }
        (Some('='), Punctuator::Invert) => {
            mode(state, Punctuator::IsNotEqual, 0);
            true
        }
        (Some('='), Punctuator::IsEqual) => {
            token(state, Punctuator::IsSame);
            true
        }
        (Some('='), Punctuator::IsNotEqual) => {
            token(state, Punctuator::IsNotSame);
            true
        }
        (Some('='), Punctuator::Divide) => {
            token(state, Punctuator::DivideEq);
            true
        }
        (Some('='), Punctuator::Mod) => {
            token(state, Punctuator::ModEq);
            true
        }
        (Some('='), Punctuator::Xor) => {
            token(state, Punctuator::XorEq);
            true
        }
        (Some('='), Punctuator::OrBitwise) => {
            token(state, Punctuator::OrBitwiseEq);
            true
        }
        (Some('='), Punctuator::Multiple) => {
            token(state, Punctuator::MultipleEq);
            true
        }
        (Some('='), Punctuator::AndBitwise) => {
            token(state, Punctuator::AndBitwiseEq);
            true
        }
        (Some('='), Punctuator::Exp) => {
            token(state, Punctuator::ExpEq);
            true
        }
        (Some('='), Punctuator::LeftShift) => {
            token(state, Punctuator::LeftShiftEq);
            true
        }
        (Some('='), Punctuator::RightShift) => {
            token(state, Punctuator::RightShiftEq);
            true
        }
        (Some('&'), Punctuator::AndBitwise) => {
            token(state, Punctuator::And);
            true
        }
        (Some('*'), Punctuator::Multiple) => {
            mode(state, Punctuator::Exp, 0);
            true
        }
        (Some('|'), Punctuator::OrBitwise) => {
            token(state, Punctuator::Or);
            true
        }
        (Some('-'), Punctuator::Minus) => {
            token(state, Punctuator::Decrement);
            true
        }
        (_, Punctuator::SmallThan) | (_, Punctuator::GreaterThan) => {
            token(state, t);
            false
        }
        (Some('/'), Punctuator::Divide) => {
            state.mode = LexerMode::Comment(CommentType::SingleLine);
            state.tmp = String::new();
            true
        }
        (Some('*'), Punctuator::Divide) => {
            state.mode = LexerMode::Comment(CommentType::MultiLineStart);
            state.tmp = String::new();
            true
        }
        (_, _) => {
            token(state, t);
            false
        }
    }
}
