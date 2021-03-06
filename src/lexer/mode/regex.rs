use error::JsResult;
use lexer::enums::{LexerMode, RegexState, TokenType, LiteralType, RegexIdentifier};
use lexer::state::{LexerState};

impl LexerState {
    fn regex(&mut self, t: RegexIdentifier) -> JsResult<()> {
        let tmp = self.tmp();
        try!(self.push(TokenType::Literal(LiteralType::Regex(tmp, t))));
        self.update(LexerMode::None);
        Ok(())
    }

    pub fn parse_regex(&mut self) -> JsResult<bool> {
        let mut handled: bool;
        loop {
            let c = self.current_char();
            let t = match self.mode() {
                LexerMode::Regex(t) => t,
                _ => {
                    panic!("Unhandled Parser State Reached: {:?}, {:?}, {:?}, col {:?}, line {:?}", c, self.mode(), self.is_escaped(), self.col(), self.line())
                }
            };
            let escaped = self.is_escaped();
            handled = match (c, t, escaped) {
                (Some('/'), RegexState::Normal, false) => {
                    self.update(LexerMode::Regex(RegexState::Identifier));
                    true
                }
                (Some('/'), RegexState::Normal, true) => {
                    self.tmp_push('/');
                    self.escaped(false);
                    true
                }
                (Some('g'), RegexState::Identifier, _) => {
                    try!(self.regex(RegexIdentifier::Global));
                    true
                }
                (Some('i'), RegexState::Identifier, _) => {
                    try!(self.regex(RegexIdentifier::Ignore));
                    true
                }
                (Some('\\'), RegexState::Normal, false) => {
                    self.escaped(true);
                    true
                }
                (Some('\\'), RegexState::Normal, true) => {
                    self.tmp_push('\\');
                    self.escaped(false);
                    true
                }
                (Some(_), RegexState::Identifier, _) => {
                    try!(self.regex(RegexIdentifier::None));
                    false
                }
                (Some(c), RegexState::Normal, true) => {
                    self.escaped(false);
                    self.tmp_push('\\');
                    self.tmp_push(c);
                    true
                }
                (Some(c), RegexState::Normal, false) => {
                    self.tmp_push(c);
                    true
                }
                (None, RegexState::Identifier, _) => {
                    try!(self.regex(RegexIdentifier::None));
                    self.update(LexerMode::EOF);
                    true
                }
                (None, RegexState::Normal, _) => {
                    panic!("Unhandled Parser State Reached: {:?}, {:?}, {:?}, col {:?}, line {:?}", c, self.mode(), self.is_escaped(), self.col(), self.line());
                    //self.update(LexerMode::EOF);
                    //true
                }
            };
            if self.mode() == LexerMode::None {
                break
            }
            if handled {
                self.next_char();
            }
        }
        Ok(handled)

    }
}
