use error::error::{Error, ErrorType, SyntaxErrorType};
use lexer::enums::{LexerMode, TokenType, Punctuator, NumberType, StringType};
use lexer::state::{LexerState};
use std::result::Result;

impl LexerState {
    fn start_punctuator(&mut self, t: Punctuator) {
        self.update(LexerMode::Punctuator(t, 0));
    }

    pub fn parse_normal(&mut self, c: Option<char>) -> Result<bool, Error> {
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
            Some('\r') => self.push(TokenType::LineTerminate),
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
            Some(';') => self.push(TokenType::Semicolon),
            Some(',') => self.push(TokenType::Comma),
            Some('{') => self.push(TokenType::Punctuator(Punctuator::LeftBrace)),
            Some('}') => self.push(TokenType::Punctuator(Punctuator::RightBrace)),
            Some('[') => self.push(TokenType::Punctuator(Punctuator::LeftBracket)),
            Some(']') => self.push(TokenType::Punctuator(Punctuator::RightBracket)),
            Some('(') => self.push(TokenType::Punctuator(Punctuator::LeftParen)),
            Some(')') => self.push(TokenType::Punctuator(Punctuator::RightParen)),
            Some('~') => self.push(TokenType::Punctuator(Punctuator::Tilde)),
            Some(':') => self.push(TokenType::Punctuator(Punctuator::DoublePoint)),
            Some('?') => self.push(TokenType::Punctuator(Punctuator::QuestionMark)),
            Some('.') => self.start_punctuator(Punctuator::Point),
            Some('|') => self.start_punctuator(Punctuator::OrBitwise),
            Some('*') => self.start_punctuator(Punctuator::Multiple),
            Some('&') => self.start_punctuator(Punctuator::AndBitwise),
            Some('^') => self.start_punctuator(Punctuator::Xor),
            Some('+') => self.start_punctuator(Punctuator::Plus),
            Some('-') => self.start_punctuator(Punctuator::Minus),
            Some('%') => self.start_punctuator(Punctuator::Mod),
            Some('=') => self.start_punctuator(Punctuator::Equal),
            Some('<') => self.start_punctuator(Punctuator::SmallThan),
            Some('/') => self.start_punctuator(Punctuator::Divide),
            Some('!') => self.start_punctuator(Punctuator::Invert),
            Some('>') => self.start_punctuator(Punctuator::GreaterThan),
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