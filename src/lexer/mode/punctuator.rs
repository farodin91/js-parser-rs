use lexer::enums::{LexerMode, TokenType, Punctuator, CommentType};
use lexer::state::{LexerState};

impl LexerState {
    fn punctuator(&mut self, t: Punctuator) {
        self.push(TokenType::Punctuator(t));
        self.update(LexerMode::None);
    }

    fn mode_punctuator(&mut self, t: Punctuator, i: i32) {
        self.update(LexerMode::Punctuator(t, i));
    }
}
pub fn exec(state: &mut LexerState, c: Option<char>, t: Punctuator, i: i32) -> bool {
    match (c, t) {
        (Some('<'), Punctuator::SmallThan) => {
            state.mode_punctuator(Punctuator::LeftShift, 0);
            true
        }
        (Some('>'), Punctuator::GreaterThan) => {
            state.mode_punctuator(Punctuator::RightShift, 0);
            true
        }
        (Some('>'), Punctuator::RightShift) => {
            state.mode_punctuator(Punctuator::RightShiftUnsigned, 0);
            true
        }
        (Some('+'), Punctuator::Plus) => {
            state.punctuator(Punctuator::Increment);
            true
        }
        (Some('>'), Punctuator::Equal) => {
            state.punctuator(Punctuator::Lamda);
            true
        }
        (Some('.'), Punctuator::Point) => {
            if i == 1 {
                state.punctuator(Punctuator::ThreePoints)
            } else {
                state.mode_punctuator(Punctuator::Point, 1);
            }
            true
        }
        (_, Punctuator::Point) => {
            if i == 1 {
                state.punctuator(Punctuator::Point);
            }
            state.punctuator(Punctuator::Point);
            false
        }
        (Some('='), Punctuator::RightShiftUnsigned) => {
            state.punctuator(Punctuator::RightShiftUnsignedAssign);
            true
        }
        (Some('='), Punctuator::GreaterThan) => {
            state.punctuator(Punctuator::GreaterAndEqualThan);
            true
        }
        (Some('='), Punctuator::SmallThan) => {
            state.punctuator(Punctuator::SmallAndEqualThan);
            true
        }
        (Some('='), Punctuator::Equal) => {
            state.mode_punctuator(Punctuator::IsEqual, 0);
            true
        }
        (Some('='), Punctuator::Invert) => {
            state.mode_punctuator(Punctuator::IsNotEqual, 0);
            true
        }
        (Some('='), Punctuator::IsEqual) => {
            state.punctuator(Punctuator::IsSame);
            true
        }
        (Some('='), Punctuator::IsNotEqual) => {
            state.punctuator(Punctuator::IsNotSame);
            true
        }
        (Some('='), Punctuator::Divide) => {
            state.punctuator(Punctuator::DivideAssign);
            true
        }
        (Some('='), Punctuator::Mod) => {
            state.punctuator(Punctuator::ModAssign);
            true
        }
        (Some('='), Punctuator::Xor) => {
            state.punctuator(Punctuator::XorAssign);
            true
        }
        (Some('='), Punctuator::OrBitwise) => {
            state.punctuator(Punctuator::OrBitwiseAssign);
            true
        }
        (Some('='), Punctuator::Multiple) => {
            state.punctuator(Punctuator::MultipleAssign);
            true
        }
        (Some('='), Punctuator::AndBitwise) => {
            state.punctuator(Punctuator::AndBitwiseAssign);
            true
        }
        (Some('='), Punctuator::Exp) => {
            state.punctuator(Punctuator::ExpAssign);
            true
        }
        (Some('='), Punctuator::LeftShift) => {
            state.punctuator(Punctuator::LeftShiftAssign);
            true
        }
        (Some('='), Punctuator::RightShift) => {
            state.punctuator(Punctuator::RightShiftAssign);
            true
        }
        (Some('&'), Punctuator::AndBitwise) => {
            state.punctuator(Punctuator::And);
            true
        }
        (Some('*'), Punctuator::Multiple) => {
            state.mode_punctuator(Punctuator::Exp, 0);
            true
        }
        (Some('|'), Punctuator::OrBitwise) => {
            state.punctuator(Punctuator::Or);
            true
        }
        (Some('-'), Punctuator::Minus) => {
            state.punctuator(Punctuator::Decrement);
            true
        }
        (_, Punctuator::SmallThan) | (_, Punctuator::GreaterThan) => {
            state.punctuator(t);
            false
        }
        (Some('/'), Punctuator::Divide) => {
            state.update(LexerMode::Comment(CommentType::SingleLine));
            state.reset_tmp();
            true
        }
        (Some('*'), Punctuator::Divide) => {
            state.update(LexerMode::Comment(CommentType::MultiLineStart));
            state.reset_tmp();
            true
        }
        (_, _) => {
            state.punctuator(t);
            false
        }
    }
}
