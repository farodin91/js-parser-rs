use error::JsResult;
use lexer::enums::{LexerMode, CommentType, TokenType};
use lexer::state::{LexerState};

impl LexerState {
    fn comment(&mut self) -> JsResult<()> {
        let tmp = self.tmp();
        try!(self.push(TokenType::CommentLiteral(tmp)));
        self.update(LexerMode::None);
        Ok(())
    }

    pub fn parse_comment(&mut self) -> JsResult<bool> {
        loop {
            let c = self.current_char();
            let t = match self.mode() {
                LexerMode::Comment(t) => t,
                _ => {
                    panic!("Unhandled Parser State Reached: {:?}, {:?}, {:?}, col {:?}, line {:?}", c, self.mode(), self.is_escaped(), self.col(), self.line())
                }
            };
            match (c, t) {
                (Some('\n'), CommentType::SingleLine) => {
                    try!(self.comment());
                    try!(self.push(TokenType::LineTerminate));
                }
                (Some('/'), CommentType::MultiLineEnd) => {
                    try!(self.comment());
                }
                (Some('*'), CommentType::MultiLineStart) => {
                    self.update(LexerMode::Comment(CommentType::MultiLineEnd));
                }
                (Some('*'), CommentType::SingleLine) => {
                    self.tmp_push(c.unwrap());
                }
                (Some('*'), CommentType::MultiLineEnd) => {
                    self.tmp_push(c.unwrap());
                    self.update(LexerMode::Comment(CommentType::MultiLineEnd));
                }
                (Some('*'), _) => {
                    self.update(LexerMode::Comment(CommentType::MultiLineEnd));
                }
                (Some(c), CommentType::MultiLineEnd) => {
                    self.tmp_push('*');
                    self.tmp_push(c);
                    self.update(LexerMode::Comment(CommentType::MultiLineNormal));
                }
                (Some(c), CommentType::SingleLine) => {
                    self.tmp_push(c);
                }
                (Some(c), _) => {
                    self.tmp_push(c);
                    self.update(LexerMode::Comment(CommentType::MultiLineNormal));
                }
                (None, _) => {
                    let tmp = self.tmp();
                    try!(self.push(TokenType::CommentLiteral(tmp)));
                    self.update(LexerMode::EOF);
                }
            };
            if self.mode() == LexerMode::None || self.mode() == LexerMode::EOF {
                break
            }
            self.next_char();
        }
        Ok(true)
    }
}