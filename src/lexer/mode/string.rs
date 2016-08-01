use error::JsResult;
use error::error::{ErrorType, SyntaxErrorType};
use lexer::enums::{LexerMode, TokenType, LiteralType};
use lexer::enums::StringType::*;
use lexer::state::{LexerState};

impl LexerState {
    pub fn parse_string(&mut self) -> JsResult<bool> {
        loop {
            let escaped = self.is_escaped();
            let c = self.current_char();
            let t = match self.mode() {
                LexerMode::String(t) => t,
                _ => {
                    panic!("Unhandled Parser State Reached: {:?}, {:?}, {:?}, col {:?}, line {:?}", c, self.mode(), self.is_escaped(), self.col(), self.line())
                }
            };
            match (c, escaped, t) {
                (Some('"'), true, DoubleQuote) => {
                    self.escaped(false);
                    self.tmp_push('"');
                }
                (Some('"'), false, DoubleQuote) => {
                    let tmp = self.tmp();
                    try!(self.push(TokenType::Literal(LiteralType::String(tmp))));
                    self.update(LexerMode::None);
                }
                (Some('\''), true, SingleQuote) => {
                    self.escaped(false);
                    self.tmp_push('\'');
                }
                (Some('\''), false, SingleQuote) => {
                    let tmp = self.tmp();
                    try!(self.push(TokenType::Literal(LiteralType::String(tmp))));
                    self.update(LexerMode::None);
                }
                (Some('\\'), false, _) => {
                    self.escaped(true);
                }
                (Some('\\'), true, _) => {
                    self.escaped(false);
                    self.tmp_push('\\');
                }
                (Some(c), true, _) => {
                    self.escaped(false);
                    self.tmp_push('\\');
                    self.tmp_push(c);
                }
                (Some(x), _, _) => {
                    self.tmp_push(x)
                }
                (None, _, _) => {
                    let err = self.error(ErrorType::SyntaxError(SyntaxErrorType::UnexpectedEOF));
                    return Err(err);
                }
            }
            if self.mode() == LexerMode::None {
                break
            }
            self.next_char();
        }
        Ok(true)
    }
}