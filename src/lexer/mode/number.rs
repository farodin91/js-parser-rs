use lexer::enums::{LexerMode, NumberType, TokenType, LiteralType};
use lexer::state::{LexerState};
use std::str::FromStr;

impl LexerState {
    fn number(&mut self, t: LiteralType) {
        self.push(TokenType::Literal(t));
        self.update(LexerMode::None);
    }

    pub fn parse_number(&mut self) -> bool {
        let mut handled: bool;
        loop {
            let c = self.current_char();
            let t = self.mode();
            handled = match (c, t) {
                (Some('x'), LexerMode::Number(NumberType::None)) |
                (Some('X'), LexerMode::Number(NumberType::None)) => {
                    self.update(LexerMode::Number(NumberType::Hex));
                    self.reset_tmp();
                    true
                }
                (Some('o'), LexerMode::Number(NumberType::None)) |
                (Some('O'), LexerMode::Number(NumberType::None)) => {
                    self.update(LexerMode::Number(NumberType::Octal));
                    self.reset_tmp();
                    true
                }
                (Some('0' ... '9'), LexerMode::Number(NumberType::None)) => {
                    self.update(LexerMode::Number(NumberType::NoneLiteral));
                    self.tmp_push(c.unwrap());
                    true
                }
                (Some('0' ... '7'), _) => {
                    self.tmp_push(c.unwrap());
                    true
                }
                (Some('8' ... '9'), LexerMode::Number(NumberType::Octal)) => {
                    panic!("Unhandled Parser State Reached: {:?}, {:?}, {:?}, col {:?}, line {:?}", c, self.mode(), self.is_escaped(), self.col(), self.line());
                }
                (Some('8' ... '9'), LexerMode::Number(_)) => {
                    self.tmp_push(c.unwrap());
                    true
                }
                (Some('a' ... 'f'), LexerMode::Number(NumberType::Hex)) | (Some('A' ... 'F'), LexerMode::Number(NumberType::Hex)) => {
                    self.tmp_push(c.unwrap());
                    true
                }
                (Some('.'), LexerMode::Number(NumberType::None)) | (Some('.'), LexerMode::Number(NumberType::NoneLiteral)) => {
                    self.update(LexerMode::Number(NumberType::Float));
                    self.tmp_push(c.unwrap());
                    true
                }
                (_, LexerMode::Number(NumberType::None)) | (_, LexerMode::Number(NumberType::NoneLiteral)) => {
                    let i = i64::from_str_radix(&self.tmp(), 10).unwrap();
                    self.number(LiteralType::Integer(i));
                    false
                }
                (_, LexerMode::Number(NumberType::Hex)) => {
                    let i = i64::from_str_radix(&self.tmp(), 16).unwrap();
                    self.number(LiteralType::Integer(i));
                    false
                }
                (_, LexerMode::Number(NumberType::Octal)) => {
                    let i = i64::from_str_radix(&self.tmp(), 8).unwrap();
                    self.number(LiteralType::Integer(i));
                    false
                }
                (_, LexerMode::Number(NumberType::Float)) => {
                    let i = f64::from_str(&self.tmp()).unwrap();
                    self.number(LiteralType::Float(i));
                    false
                }
                (_, _) => {
                    panic!("Unhandled Parser State Reached: {:?}, {:?}, {:?}, col {:?}, line {:?}", c, self.mode(), self.is_escaped(), self.col(), self.line());
                    //self.update(LexerMode::EOF)
                }
            };
            if self.mode() == LexerMode::None {
                break
            }
            if handled {
                self.next_char();
            }
        }
        handled
    }
}