use lexer::enums::{TokenType, LexerMode};
use lexer::token::Token;
use error::JsResult;
use error::error::{Error, ErrorType, SyntaxErrorType};
use std::char;

pub type LexerStateIterator = Box<Iterator<Item = char>>;

pub struct LexerState {
    input: LexerStateIterator,
    tokens: Vec<Token>,
    last_token: Option<TokenType>,
    mode: LexerMode,
    tmp: String,
    escaped: bool,
    last_char: Option<char>,
    current_char: Option<char>,
    last_char_is_unicode: bool,
    col: u32,
    line: u64
}

impl LexerState {
    pub fn new(input: LexerStateIterator) -> LexerState {
        LexerState {
            input: input,
            tokens: Vec::new(),
            mode: LexerMode::None,
            tmp: String::new(),
            escaped: false,
            last_char: None,
            last_char_is_unicode: false,
            last_token: None,
            current_char: None,
            col: 1,
            line: 1
        }
    }

    pub fn parse(&mut self) -> JsResult<()> {
        loop {
            self.next_char();
            let mut done = false; // mut done: bool
            while !done {
                let mode = self.mode();
                let c = self.current_char();
                let result = match mode {
                    LexerMode::None => self.parse_normal(c),
                    LexerMode::String(_) => self.parse_string(),
                    LexerMode::Punctuator(t, i) => self.parse_punctuator(c, t, i),
                    LexerMode::Number(_) => self.parse_number(),
                    LexerMode::Comment(_) => self.parse_comment(),
                    LexerMode::Raw => self.parse_raw(),
                    LexerMode::Regex(_) => self.parse_regex(),
                    LexerMode::EOF => Ok(true)
                };
                done = match result {
                    Ok(t) => t,
                    Err(err) => {
                        return Err(err)
                    }
                }
            }
            if self.mode() == LexerMode::EOF {
                break;
            }
        }
        Ok(())
    }

    pub fn read_unicode(&mut self) -> Option<char> {
        let indicated = self.next_char();
        match indicated {
            Some('u') => {
                let mut tmp = String::new();
                let a1 = self.next_char().unwrap();
                tmp.push(a1);
                let a1 = self.next_char().unwrap();
                tmp.push(a1);
                let a1 = self.next_char().unwrap();
                tmp.push(a1);
                let a1 = self.next_char().unwrap();
                tmp.push(a1);
                let i = u32::from_str_radix(&tmp, 16).unwrap();
                self.last_char_is_unicode = true;
                char::from_u32(i)
            },
            _ => None
        }
    }

    pub fn last_char_is_unicode(&mut self) -> bool {
        self.last_char_is_unicode
    }

    pub fn set_last_char_is_unicode(&mut self, e: bool) {
        self.last_char_is_unicode= e
    }

    pub fn error(&mut self, t: ErrorType) -> Error {
        Error::new(t, self.col, self.line, None)
    }

    pub fn overwrite_current_char_with_unicode(&mut self, c: char) {
        self.last_char = self.current_char;
        self.last_char_is_unicode = true;
        self.current_char = Some(c)
    }

    pub fn col(&mut self) -> u32 {
        self.col
    }

    pub fn line(&mut self) -> u64 {
        self.line
    }

    pub fn escaped(&mut self, e: bool) {
        self.escaped = e
    }

    pub fn is_escaped(&mut self) -> bool {
        self.escaped
    }

    pub fn reset_tmp(&mut self) {
        self.tmp = String::new()
    }

    pub fn tmp(&mut self) -> String {
        self.tmp.clone()
    }

    pub fn tmp_push(&mut self, c: char) {
        self.tmp.push(c)
    }

    pub fn next_char(&mut self) -> Option<char> {
        self.last_char = self.current_char;
        let char = self.input.next();
        match char {
            Some('\n') => {
                self.line += 1;
                self.col = 1;
            }
            _ => {
                self.col += 1;
            }
        }
        self.last_char_is_unicode = false;
        self.current_char = char;
        char
    }

    pub fn current_char(&self) -> Option<char> {
        self.current_char
    }

    pub fn last_char(&self) -> Option<char> {
        self.last_char
    }

    pub fn mode(&self) -> LexerMode {
        self.mode.clone()
    }

    pub fn update(&mut self, t: LexerMode) {
        self.mode = t
    }

    pub fn last_token(&self) -> Option<TokenType> {
        self.last_token.clone()
    }

    pub fn push(&mut self, t: TokenType) -> JsResult<()>{
        let t = match t {
            TokenType::CommentLiteral(_) => None,
            TokenType::LineTerminate => {
                match self.last_token {
                    None => None,
                    Some(TokenType::Semicolon) => None,
                    Some(TokenType::Yield) => {
                        return Err(Error::new(ErrorType::SyntaxError(SyntaxErrorType::UnexpectedEOL), self.col, self.line, None))
                    },
                    _ => Some(TokenType::LineTerminate)
                }
            }
            TokenType::Lamda => {
                match self.last_token {
                    Some(TokenType::LineTerminate) => {
                        return Err(Error::new(ErrorType::SyntaxError(SyntaxErrorType::UnexpectedEOL), self.col, self.line, None))
                    },
                    _ => Some(TokenType::Lamda)
                }
            }
            t => {
                Some(t)
            }
        };
        match t {
            Some(t) => {
                self.last_token = Some(t.clone());
                let token = Token::new(t, self.col, self.line);
                self.tokens.push(token)
            }
            None => ()
        };
        Ok(())
    }

    pub fn tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }
}

impl Iterator for LexerState {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        //let c = self.next_char();

        /*match c {
            Some('a' ... 'z') | Some('A' ... 'Z') | Some('_') | Some('$') => {
                self.reset_tmp();
                self.tmp_push(c.unwrap());
            }
            Some('"') => {
                self.reset_tmp();
            }
            Some('\'') => {
                self.reset_tmp();
            }
            Some('0') => {
                self.reset_tmp();
                self.tmp_push(c.unwrap());
            }
            Some('1'...'9') => {
                self.reset_tmp();
                self.tmp_push(c.unwrap());
            }
            Some('\n') |
            Some('\r') => Some(TokenType::LineTerminate),
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
            Some(';') => Some(TokenType::Semicolon),
            Some(',') => Some(TokenType::Comma),
            Some('{') => Some(TokenType::Punctuator(Punctuator::LeftBrace)),
            Some('}') => Some(TokenType::Punctuator(Punctuator::RightBrace)),
            Some('[') => Some(TokenType::Punctuator(Punctuator::LeftBracket)),
            Some(']') => Some(TokenType::Punctuator(Punctuator::RightBracket)),
            Some('(') => Some(TokenType::Punctuator(Punctuator::LeftParen)),
            Some(')') => Some(TokenType::Punctuator(Punctuator::RightParen)),
            Some('~') => Some(TokenType::Punctuator(Punctuator::Tilde)),
            Some(':') => Some(TokenType::Punctuator(Punctuator::Colon)),
            Some('?') => Some(TokenType::Punctuator(Punctuator::QuestionMark)),
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
        }*/
        None
    }
}