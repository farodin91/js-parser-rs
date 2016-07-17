
use lexer::enums::{ LexerMode, TokenType, Punctuator, Operation };
use lexer::state::{ LexerState };

fn token(state: &mut LexerState, t: Punctuator) {
    state.tokens.push(TokenType::Punctuator(t));
    state.mode = LexerMode::None;
}

pub fn exec(state: &mut LexerState, c: Option<char>, t: Punctuator) -> bool {
    match (c, t) {
        (Some('<'), Punctuator::SmallThan) => {
            state.mode = LexerMode::Punctuator(Punctuator::LeftShift);
            true
        }
        (Some('>'), Punctuator::GreaterThan) => {
            state.mode = LexerMode::Punctuator(Punctuator::RightShift);
            true
        }
        (Some('>'), Punctuator::RightShift) => {
            token(state, Punctuator::RightShiftUnsigned);
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
        (Some('='), Punctuator::GreaterThan) => {
            token(state, Punctuator::GreaterAndEqualThan);
            true
        }
        (Some('='), Punctuator::SmallThan) => {
            token(state, Punctuator::SmallAndEqualThan);
            true
        }
        (Some('='), Punctuator::Equal) => {
            state.mode = LexerMode::Punctuator(Punctuator::IsEqual);
            true
        }
        (Some('='), Punctuator::Invert) => {
            state.mode = LexerMode::Punctuator(Punctuator::IsNotEqual);
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
            state.mode = LexerMode::Punctuator(Punctuator::Exp);
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
        (Some('/'), Punctuator::Divide) | (Some('*'), Punctuator::Divide) => {
            state.mode = LexerMode::Comment(Operation::Start);
            state.tmp = String::new();
            true
        }
        (_, _) => {
            token(state, t);
            false
        }
        //(_,_) => {
        //    println!("Unhandled Parser State Reached: {:?}, {:?}, {:?}", c, state.mode, state.escaped);
        //    state.mode = LexerMode::EOF;
        //    true
        //}
    }
}
