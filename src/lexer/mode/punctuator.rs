use lexer::enums::{LexerMode, TokenType, Punctuator, CommentType, RegexState};
use lexer::state::{LexerState};

impl LexerState {
    fn punctuator(&mut self, t: Punctuator) {
        self.push(TokenType::Punctuator(t));
        self.update(LexerMode::None);
    }

    fn mode_punctuator(&mut self, t: Punctuator, i: i32) {
        self.update(LexerMode::Punctuator(t, i));
    }

    pub fn parse_punctuator(&mut self, c: Option<char>, t: Punctuator, i: i32) -> bool {
        match (c, t) {
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
                self.punctuator(Punctuator::Increment);
                true
            }
            (Some('>'), Punctuator::Equal) => {
                self.punctuator(Punctuator::Lamda);
                true
            }
            (Some('.'), Punctuator::Point) => {
                if i == 1 {
                    self.punctuator(Punctuator::ThreePoints)
                } else {
                    self.mode_punctuator(Punctuator::Point, 1);
                }
                true
            }
            (_, Punctuator::Point) => {
                if i == 1 {
                    self.punctuator(Punctuator::Point);
                }
                self.punctuator(Punctuator::Point);
                false
            }
            (Some('='), Punctuator::RightShiftUnsigned) => {
                self.punctuator(Punctuator::RightShiftUnsignedAssign);
                true
            }
            (Some('='), Punctuator::GreaterThan) => {
                self.punctuator(Punctuator::GreaterAndEqualThan);
                true
            }
            (Some('='), Punctuator::SmallThan) => {
                self.punctuator(Punctuator::SmallAndEqualThan);
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
                self.punctuator(Punctuator::IsSame);
                true
            }
            (Some('='), Punctuator::IsNotEqual) => {
                self.punctuator(Punctuator::IsNotSame);
                true
            }
            (Some('='), Punctuator::Divide) => {
                self.punctuator(Punctuator::DivideAssign);
                true
            }
            (Some('='), Punctuator::Mod) => {
                self.punctuator(Punctuator::ModAssign);
                true
            }
            (Some('='), Punctuator::Xor) => {
                self.punctuator(Punctuator::XorAssign);
                true
            }
            (Some('='), Punctuator::OrBitwise) => {
                self.punctuator(Punctuator::OrBitwiseAssign);
                true
            }
            (Some('='), Punctuator::Multiple) => {
                self.punctuator(Punctuator::MultipleAssign);
                true
            }
            (Some('='), Punctuator::AndBitwise) => {
                self.punctuator(Punctuator::AndBitwiseAssign);
                true
            }
            (Some('='), Punctuator::Exp) => {
                self.punctuator(Punctuator::ExpAssign);
                true
            }
            (Some('='), Punctuator::LeftShift) => {
                self.punctuator(Punctuator::LeftShiftAssign);
                true
            }
            (Some('='), Punctuator::RightShift) => {
                self.punctuator(Punctuator::RightShiftAssign);
                true
            }
            (Some('&'), Punctuator::AndBitwise) => {
                self.punctuator(Punctuator::And);
                true
            }
            (Some('*'), Punctuator::Multiple) => {
                self.mode_punctuator(Punctuator::Exp, 0);
                true
            }
            (Some('|'), Punctuator::OrBitwise) => {
                self.punctuator(Punctuator::Or);
                true
            }
            (Some('-'), Punctuator::Minus) => {
                self.punctuator(Punctuator::Decrement);
                true
            }
            (_, Punctuator::SmallThan) | (_, Punctuator::GreaterThan) => {
                self.punctuator(t);
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
                    Some(TokenType::Punctuator(Punctuator::DoublePoint)) |
                    Some(TokenType::Punctuator(Punctuator::Equal)) |
                    Some(TokenType::Punctuator(Punctuator::LeftParen)) |
                    Some(TokenType::Comma) => {
                        self.update(LexerMode::Regex(RegexState::Normal));
                        self.reset_tmp();
                        self.tmp_push(c);
                        true
                    }
                    _ => {
                        self.punctuator(t);
                        false
                    }
                }
            }
            (_, _) => {
                self.punctuator(t);
                false
            }
        }
    }
}