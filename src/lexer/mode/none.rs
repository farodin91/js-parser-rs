use error::JsResult;
use error::error::{ErrorType, SyntaxErrorType};
use lexer::enums::{LexerMode, TokenType, NumberType, StringType};
use lexer::state::{LexerState};

impl LexerState {
    fn start_punctuator(&mut self, t: TokenType) {
        self.update(LexerMode::Punctuator(t, 0));
    }

    pub fn parse_normal(&mut self, c: Option<char>) -> JsResult<bool> {
        let mut handled = true;
        match c {
            Some('a' ... 'z') | Some('A' ... 'Z') | Some('_') | Some('$') => {
                self.update(LexerMode::Raw);
                self.reset_tmp();
                self.tmp_push(c.unwrap());
            }
            Some('"') => {
                self.update(LexerMode::String(StringType::DoubleQuote));
                self.reset_tmp();
            }
            Some('\'') => {
                self.update(LexerMode::String(StringType::SingleQuote));
                self.reset_tmp();
            }
            Some('0') => {
                self.update(LexerMode::Number(NumberType::None));
                self.reset_tmp();
                self.tmp_push(c.unwrap());
            }
            Some('1'...'9') => {
                self.update(LexerMode::Number(NumberType::NoneLiteral));
                self.reset_tmp();
                self.tmp_push(c.unwrap());
            }
            Some('\n') |
            Some('\r') => try!(self.push(TokenType::LineTerminate)),
            Some(' ') |
            Some('\t') |
            Some('\u{c}') |
            Some('\u{b}') |
            Some('\u{a0}') => {
                if self.last_char_is_unicode() {
                    let c = self.current_char();
                    let err = self.error(ErrorType::SyntaxError(SyntaxErrorType::UnexpectedChar(c.unwrap())));
                    return Err(err);
                }
            },
            Some(';') => try!(self.push(TokenType::Semicolon)),
            Some(',') => try!(self.push(TokenType::Comma)),
            Some('{') => try!(self.push(TokenType::LeftBrace)),
            Some('}') => try!(self.push(TokenType::RightBrace)),
            Some('[') => try!(self.push(TokenType::LeftBracket)),
            Some(']') => try!(self.push(TokenType::RightBracket)),
            Some('(') => try!(self.push(TokenType::LeftParen)),
            Some(')') => try!(self.push(TokenType::RightParen)),
            Some('~') => try!(self.push(TokenType::Tilde)),
            Some(':') => try!(self.push(TokenType::Colon)),
            Some('?') => try!(self.push(TokenType::QuestionMark)),
            Some('.') => self.start_punctuator(TokenType::Point),
            Some('|') => self.start_punctuator(TokenType::OrBitwise),
            Some('*') => self.start_punctuator(TokenType::Multiple),
            Some('&') => self.start_punctuator(TokenType::AndBitwise),
            Some('^') => self.start_punctuator(TokenType::Xor),
            Some('+') => self.start_punctuator(TokenType::Plus),
            Some('-') => self.start_punctuator(TokenType::Minus),
            Some('%') => self.start_punctuator(TokenType::Mod),
            Some('=') => self.start_punctuator(TokenType::Equal),
            Some('<') => self.start_punctuator(TokenType::SmallThan),
            Some('/') => self.start_punctuator(TokenType::Divide),
            Some('!') => self.start_punctuator(TokenType::Invert),
            Some('>') => self.start_punctuator(TokenType::GreaterThan),
            None => {
                self.update(LexerMode::EOF)
            }
            Some('\\') => {
                let unicode = self.read_unicode();
                match unicode {
                    Some(c) => {
                        println!("{:?}", c);
                        self.overwrite_current_char_with_unicode(c);
                        handled = false
                    }
                    _ => {
                        panic!("Unhandled Parser State Reached: {:?}, {:?}, {:?}, col {:?}, line {:?}", c, self.mode(), self.is_escaped(), self.col(), self.line());
                    }
                }
            }
            _ => {
                panic!("Unhandled Parser State Reached: {:?}, {:?}, {:?}, col {:?}, line {:?}", c, self.mode(), self.is_escaped(), self.col(), self.line());
                //self.update(LexerMode::EOF);
            }
        }
        Ok(handled)
    }
}