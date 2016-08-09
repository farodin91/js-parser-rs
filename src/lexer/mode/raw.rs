use error::JsResult;
use lexer::enums::{LexerMode, TokenType, LiteralType};
use lexer::state::{LexerState};

impl LexerState {
    fn raw(&mut self) -> JsResult<()> {
        self.update(LexerMode::None);
        let tmp = self.tmp();
        let tmp = tmp.as_str();
        let token = match tmp {
            "var" => TokenType::Var,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "do" => TokenType::Do,
            "typeof" => TokenType::Typeof,
            "switch" => TokenType::Switch,
            "catch" => TokenType::Catch,
            "try" => TokenType::Try,
            "instanceof" => TokenType::Instanceof,
            "export" => TokenType::Export,
            "return" => TokenType::Return,
            "void" => TokenType::Void,
            "extends" => TokenType::Extends,
            "const" => TokenType::Const,
            "finally" => TokenType::Finally,
            "super" => TokenType::Super,
            "with" => TokenType::With,
            "yield" => TokenType::Yield,
            "default" => TokenType::Default,
            "function" => TokenType::Function,
            "of" => TokenType::Of,
            "in" => TokenType::In,
            "for" => TokenType::For,
            "while" => TokenType::While,
            "class" => TokenType::Class,
            "break" => TokenType::Break,
            "continue" => TokenType::Continue,
            "new" => TokenType::New,
            "case" => TokenType::Case,
            "debugger" => TokenType::Debugger,
            "throw" => TokenType::Throw,
            "let" => TokenType::Let,
            "this" => TokenType::This,
            "target" => TokenType::Target,
            "delete" => TokenType::Delete,
            "set" => TokenType::Set,
            "get" => TokenType::Get,
            "true" => TokenType::Literal(LiteralType::Boolean(true)),
            "false" => TokenType::Literal(LiteralType::Boolean(false)),
            "null" => TokenType::Literal(LiteralType::Null),
            tmp => {
                TokenType::Identifier(String::from(tmp))
            }
        };
        self.push(token)
    }

    pub fn parse_raw(&mut self) -> JsResult<bool> {
        let mut handled: bool;
        loop {
            let c = self.current_char();
            match c {
                Some('a' ... 'z') | Some('A' ... 'Z') | Some('_') | Some('$') | Some('0' ... '9') => {
                    self.tmp_push(c.unwrap());
                    handled = true
                }
                Some(' ') |
                Some('\t') |
                Some('\u{c}') |
                Some('\u{b}') |
                Some('\u{a0}') |
                None => {
                    try!(self.raw());
                    handled = false
                }
                Some('\r') |
                Some('\n') |
                Some(':') |
                Some('*') |
                Some('+') |
                Some('-') |
                Some('!') |
                Some('{') |
                Some('}') |
                Some('(') |
                Some(')') |
                Some('[') |
                Some(']') |
                Some(';') |
                Some('.') |
                Some(',') |
                Some('<') |
                Some('>') |
                Some('?') |
                Some('%') |
                Some('=') |
                Some('&') |
                Some('|') |
                Some('/') => {
                    try!(self.raw());
                    handled = false
                }
                Some('\\') => {
                    let unicode = self.read_unicode();
                    match unicode {
                        Some(c) => {
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
                    //true
                }
            }
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