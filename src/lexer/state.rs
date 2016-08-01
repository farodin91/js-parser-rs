use lexer::enums::{TokenType, LexerMode, Keyword, Punctuator};
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

    pub fn parse(&mut self) -> Result<(), Error> {
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
        self.mode
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
                    Some(TokenType::Semicolon) => None,
                    Some(TokenType::Punctuator(Punctuator::LeftBrace)) => None,
                    Some(TokenType::Punctuator(Punctuator::RightBrace)) => None,
                    None => None,
                    Some(TokenType::Keyword(Keyword::Return)) => Some(TokenType::Semicolon),
                    Some(TokenType::Keyword(Keyword::Break)) => Some(TokenType::Semicolon),
                    Some(TokenType::Keyword(Keyword::Continue)) => Some(TokenType::Semicolon),
                    Some(TokenType::Keyword(Keyword::Yield)) => {
                        return Err(Error::new(ErrorType::SyntaxError(SyntaxErrorType::UnexpectedEOL), self.col, self.line, None))
                    },
                    _ => Some(TokenType::LineTerminate)
                }
            }
            TokenType::Punctuator(Punctuator::Lamda) => {
                match self.last_token {
                    Some(TokenType::LineTerminate) => {
                        return Err(Error::new(ErrorType::SyntaxError(SyntaxErrorType::UnexpectedEOL), self.col, self.line, None))
                    },
                    _ => Some(TokenType::Punctuator(Punctuator::Lamda))
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