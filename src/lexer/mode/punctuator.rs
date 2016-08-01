use error::JsResult;
use lexer::enums::{LexerMode, TokenType, Punctuator, CommentType, RegexState};
use lexer::state::{LexerState};

impl LexerState {
    fn punctuator(&mut self, t: Punctuator) -> JsResult<()> {
        self.update(LexerMode::None);
        self.push(TokenType::Punctuator(t))
    }

    fn mode_punctuator(&mut self, t: Punctuator, i: i32) {
        self.update(LexerMode::Punctuator(t, i));
    }

    pub fn parse_punctuator(&mut self, c: Option<char>, t: Punctuator, i: i32) -> JsResult<bool> {
        let handled = match (c, t) {
            (Some('<'), Punctuator::SmallThan) => {
                self.mode_punctuator(Punctuator::LeftShift, 0);
                true
            }
            (Some('>'), Punctuator::GreaterThan) => {
                self.mode_punctuator(Punctuator::RightShift, 0);
                true
            }
            (Some('>'), Punctuator::RightShift) => {
                self.mode_punctuator(Punctuator::RightShiftUnsigned, 0);
                true
            }
            (Some('+'), Punctuator::Plus) => {
                try!(self.punctuator(Punctuator::Increment));
                true
            }
            (Some('>'), Punctuator::Equal) => {
                try!(self.punctuator(Punctuator::Lamda));
                true
            }
            (Some('.'), Punctuator::Point) => {
                if i == 1 {
                    try!(self.punctuator(Punctuator::ThreePoints))
                } else {
                    self.mode_punctuator(Punctuator::Point, 1);
                }
                true
            }
            (_, Punctuator::Point) => {
                if i == 1 {
                    try!(self.punctuator(Punctuator::Point));
                }
                try!(self.punctuator(Punctuator::Point));
                false
            }
            (Some('='), Punctuator::RightShiftUnsigned) => {
                try!(self.punctuator(Punctuator::RightShiftUnsignedAssign));
                true
            }
            (Some('='), Punctuator::GreaterThan) => {
                try!(self.punctuator(Punctuator::GreaterAndEqualThan));
                true
            }
            (Some('='), Punctuator::SmallThan) => {
                try!(self.punctuator(Punctuator::SmallAndEqualThan));
                true
            }
            (Some('='), Punctuator::Equal) => {
                self.mode_punctuator(Punctuator::IsEqual, 0);
                true
            }
            (Some('='), Punctuator::Invert) => {
                self.mode_punctuator(Punctuator::IsNotEqual, 0);
                true
            }
            (Some('='), Punctuator::IsEqual) => {
                try!(self.punctuator(Punctuator::IsSame));
                true
            }
            (Some('='), Punctuator::IsNotEqual) => {
                try!(self.punctuator(Punctuator::IsNotSame));
                true
            }
            (Some('='), Punctuator::Divide) => {
                try!(self.punctuator(Punctuator::DivideAssign));
                true
            }
            (Some('='), Punctuator::Mod) => {
                try!(self.punctuator(Punctuator::ModAssign));
                true
            }
            (Some('='), Punctuator::Xor) => {
                try!(self.punctuator(Punctuator::XorAssign));
                true
            }
            (Some('='), Punctuator::OrBitwise) => {
                try!(self.punctuator(Punctuator::OrBitwiseAssign));
                true
            }
            (Some('='), Punctuator::Multiple) => {
                try!(self.punctuator(Punctuator::MultipleAssign));
                true
            }
            (Some('='), Punctuator::AndBitwise) => {
                try!(self.punctuator(Punctuator::AndBitwiseAssign));
                true
            }
            (Some('='), Punctuator::Exp) => {
                try!(self.punctuator(Punctuator::ExpAssign));
                true
            }
            (Some('='), Punctuator::LeftShift) => {
                try!(self.punctuator(Punctuator::LeftShiftAssign));
                true
            }
            (Some('='), Punctuator::RightShift) => {
                try!(self.punctuator(Punctuator::RightShiftAssign));
                true
            }
            (Some('&'), Punctuator::AndBitwise) => {
                try!(self.punctuator(Punctuator::And));
                true
            }
            (Some('*'), Punctuator::Multiple) => {
                self.mode_punctuator(Punctuator::Exp, 0);
                true
            }
            (Some('|'), Punctuator::OrBitwise) => {
                try!(self.punctuator(Punctuator::Or));
                true
            }
            (Some('-'), Punctuator::Minus) => {
                try!(self.punctuator(Punctuator::Decrement));
                true
            }
            (_, Punctuator::SmallThan) | (_, Punctuator::GreaterThan) => {
                try!(self.punctuator(t));
                false
            }
            (Some('/'), Punctuator::Divide) => {
                self.update(LexerMode::Comment(CommentType::SingleLine));
                self.reset_tmp();
                true
            }
            (Some('*'), Punctuator::Divide) => {
                self.update(LexerMode::Comment(CommentType::MultiLineStart));
                self.reset_tmp();
                true
            }
            (Some(c), Punctuator::Divide) => {
                let last_token = self.last_token();
                match last_token {
                    Some(TokenType::Punctuator(Punctuator::Colon)) |
                    Some(TokenType::Punctuator(Punctuator::Equal)) |
                    Some(TokenType::Punctuator(Punctuator::LeftParen)) |
                    Some(TokenType::Comma) => {
                        self.update(LexerMode::Regex(RegexState::Normal));
                        self.reset_tmp();
                        self.tmp_push(c);
                        true
                    }
                    _ => {
                        try!(self.punctuator(t));
                        false
                    }
                }
            }
            (_, _) => {
                try!(self.punctuator(t));
                false
            }
        };
        Ok(handled)
    }
}