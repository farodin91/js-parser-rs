use error::JsResult;
use lexer::enums::{LexerMode, TokenType, CommentType, RegexState};
use lexer::state::{LexerState};

impl LexerState {
    fn punctuator(&mut self, t: TokenType) -> JsResult<()> {
        self.update(LexerMode::None);
        self.push(t)
    }

    fn mode_punctuator(&mut self, t: TokenType, i: i32) {
        self.update(LexerMode::Punctuator(t, i));
    }

    pub fn parse_punctuator(&mut self, c: Option<char>, t: TokenType, i: i32) -> JsResult<bool> {
        let handled = match (c, t.clone()) {
            (Some('<'), TokenType::SmallThan) => {
                self.mode_punctuator(TokenType::LeftShift, 0);
                true
            }
            (Some('>'), TokenType::GreaterThan) => {
                self.mode_punctuator(TokenType::RightShift, 0);
                true
            }
            (Some('>'), TokenType::RightShift) => {
                self.mode_punctuator(TokenType::RightShiftUnsigned, 0);
                true
            }
            (Some('+'), TokenType::Plus) => {
                try!(self.punctuator(TokenType::Increment));
                true
            }
            (Some('>'), TokenType::Equal) => {
                try!(self.punctuator(TokenType::Lamda));
                true
            }
            (Some('.'), TokenType::Point) => {
                if i == 1 {
                    try!(self.punctuator(TokenType::ThreePoints))
                } else {
                    self.mode_punctuator(TokenType::Point, 1);
                }
                true
            }
            (_, TokenType::Point) => {
                if i == 1 {
                    try!(self.punctuator(TokenType::Point));
                }
                try!(self.punctuator(TokenType::Point));
                false
            }
            (Some('='), TokenType::RightShiftUnsigned) => {
                try!(self.punctuator(TokenType::RightShiftUnsignedAssign));
                true
            }
            (Some('='), TokenType::Minus) => {
                try!(self.punctuator(TokenType::MinusAssign));
                true
            }
            (Some('='), TokenType::Plus) => {
                try!(self.punctuator(TokenType::PlusAssign));
                true
            }
            (Some('='), TokenType::GreaterThan) => {
                try!(self.punctuator(TokenType::GreaterAndEqualThan));
                true
            }
            (Some('='), TokenType::SmallThan) => {
                try!(self.punctuator(TokenType::SmallAndEqualThan));
                true
            }
            (Some('='), TokenType::Equal) => {
                self.mode_punctuator(TokenType::IsEqual, 0);
                true
            }
            (Some('='), TokenType::Invert) => {
                self.mode_punctuator(TokenType::IsNotEqual, 0);
                true
            }
            (Some('='), TokenType::IsEqual) => {
                try!(self.punctuator(TokenType::IsSame));
                true
            }
            (Some('='), TokenType::IsNotEqual) => {
                try!(self.punctuator(TokenType::IsNotSame));
                true
            }
            (Some('='), TokenType::Divide) => {
                try!(self.punctuator(TokenType::DivideAssign));
                true
            }
            (Some('='), TokenType::Mod) => {
                try!(self.punctuator(TokenType::ModAssign));
                true
            }
            (Some('='), TokenType::Xor) => {
                try!(self.punctuator(TokenType::XorAssign));
                true
            }
            (Some('='), TokenType::OrBitwise) => {
                try!(self.punctuator(TokenType::OrBitwiseAssign));
                true
            }
            (Some('='), TokenType::Multiple) => {
                try!(self.punctuator(TokenType::MultipleAssign));
                true
            }
            (Some('='), TokenType::AndBitwise) => {
                try!(self.punctuator(TokenType::AndBitwiseAssign));
                true
            }
            (Some('='), TokenType::Exp) => {
                try!(self.punctuator(TokenType::ExpAssign));
                true
            }
            (Some('='), TokenType::LeftShift) => {
                try!(self.punctuator(TokenType::LeftShiftAssign));
                true
            }
            (Some('='), TokenType::RightShift) => {
                try!(self.punctuator(TokenType::RightShiftAssign));
                true
            }
            (Some('&'), TokenType::AndBitwise) => {
                try!(self.punctuator(TokenType::And));
                true
            }
            (Some('*'), TokenType::Multiple) => {
                self.mode_punctuator(TokenType::Exp, 0);
                true
            }
            (Some('|'), TokenType::OrBitwise) => {
                try!(self.punctuator(TokenType::Or));
                true
            }
            (Some('-'), TokenType::Minus) => {
                try!(self.punctuator(TokenType::Decrement));
                true
            }
            (_, TokenType::SmallThan) | (_, TokenType::GreaterThan) => {
                try!(self.punctuator(t));
                false
            }
            (Some('/'), TokenType::Divide) => {
                self.update(LexerMode::Comment(CommentType::SingleLine));
                self.reset_tmp();
                true
            }
            (Some('*'), TokenType::Divide) => {
                self.update(LexerMode::Comment(CommentType::MultiLineStart));
                self.reset_tmp();
                true
            }
            (Some(c), TokenType::Divide) => {
                let last_token = self.last_token();
                match last_token {
                    Some(TokenType::Colon) |
                    Some(TokenType::Equal) |
                    Some(TokenType::LeftParen) |
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