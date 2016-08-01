use error::JsResult;
use lexer::enums::{LexerMode, NumberType, TokenType, LiteralType};
use lexer::state::{LexerState};
use std::str::FromStr;

impl LexerState {
    fn number(&mut self, t: LiteralType) -> JsResult<()> {
        try!(self.push(TokenType::Literal(t)));
        self.update(LexerMode::None);
        Ok(())
    }

    pub fn parse_number(&mut self) -> JsResult<bool> {
        let mut handled: bool;
        loop {
            let c = self.current_char();
            let t = match self.mode() {
                LexerMode::Number(t) => t,
                _ => {
                    panic!("Unhandled Parser State Reached: {:?}, {:?}, {:?}, col {:?}, line {:?}", c, self.mode(), self.is_escaped(), self.col(), self.line())
                }
            };
            handled = match (c, t) {
                (Some('x'), NumberType::None) |
                (Some('X'), NumberType::None) => {
                    self.update(LexerMode::Number(NumberType::Hex));
                    self.reset_tmp();
                    true
                }
                (Some('o'), NumberType::None) |
                (Some('O'), NumberType::None) => {
                    self.update(LexerMode::Number(NumberType::Octal));
                    self.reset_tmp();
                    true
                }
                (Some('0' ... '9'), NumberType::None) => {
                    self.update(LexerMode::Number(NumberType::NoneLiteral));
                    self.tmp_push(c.unwrap());
                    true
                }
                (Some('0' ... '7'), _) => {
                    self.tmp_push(c.unwrap());
                    true
                }
                (Some('8' ... '9'), NumberType::Octal) => {
                    panic!("Unhandled Parser State Reached: {:?}, {:?}, {:?}, col {:?}, line {:?}", c, self.mode(), self.is_escaped(), self.col(), self.line());
                }
                (Some('8' ... '9'), _) => {
                    self.tmp_push(c.unwrap());
                    true
                }
                (Some('a' ... 'f'), NumberType::Hex) | (Some('A' ... 'F'), NumberType::Hex) => {
                    self.tmp_push(c.unwrap());
                    true
                }
                (Some('.'), NumberType::None) | (Some('.'), NumberType::NoneLiteral) => {
                    self.update(LexerMode::Number(NumberType::Float));
                    self.tmp_push(c.unwrap());
                    true
                }
                (_, NumberType::None) | (_, NumberType::NoneLiteral) => {
                    let i = i64::from_str_radix(&self.tmp(), 10).unwrap();
                    try!(self.number(LiteralType::Integer(i)));
                    false
                }
                (_, NumberType::Hex) => {
                    let i = i64::from_str_radix(&self.tmp(), 16).unwrap();
                    try!(self.number(LiteralType::Integer(i)));
                    false
                }
                (_, NumberType::Octal) => {
                    let i = i64::from_str_radix(&self.tmp(), 8).unwrap();
                    try!(self.number(LiteralType::Integer(i)));
                    false
                }
                (_, NumberType::Float) => {
                    let i = f64::from_str(&self.tmp()).unwrap();
                    try!(self.number(LiteralType::Float(i)));
                    false
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