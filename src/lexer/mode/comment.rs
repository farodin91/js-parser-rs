use lexer::enums::{LexerMode, CommentType, TokenType};
use lexer::state::{LexerState};

impl LexerState {
    fn comment(&mut self) {
        let tmp = self.tmp();
        self.push(TokenType::CommentLiteral(tmp));
        self.update(LexerMode::None);
    }

    pub fn parse_comment(&mut self) -> bool {
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
                    self.comment();
                    self.push(TokenType::LineTerminate);
                }
                (Some('/'), CommentType::MultiLineEnd) => {
                    self.comment();
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
                    self.push(TokenType::CommentLiteral(tmp));
                    self.update(LexerMode::EOF);
                }
            };
            if self.mode() == LexerMode::None || self.mode() == LexerMode::EOF {
                break
            }
            self.next_char();
        }
        true
    }
}