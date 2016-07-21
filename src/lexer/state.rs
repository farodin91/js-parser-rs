use lexer::enums::{TokenType, LexerMode};
use std::char;

pub type LexerStateIterator = Box<Iterator<Item = char>>;

pub struct LexerState {
    input: LexerStateIterator,
    tokens: Vec<TokenType>,
    last_token: Option<TokenType>,
    mode: LexerMode,
    tmp: String,
    escaped: bool,
    last_char: Option<char>,
    current_char: Option<char>,
    col: i64,
    line: i64
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
            last_token: None,
            current_char: None,
            col: 0,
            line: 0
        }
    }

    pub fn parse(&mut self) -> Result<Vec<TokenType>, ()> {
        loop {
            self.next_char();
            let mut done = false; // mut done: bool
            while !done {
                let mode = self.mode();
                let c = self.current_char();
                done = match mode {
                    LexerMode::None => self.parse_normal(c),
                    LexerMode::String(_) => self.parse_string(),
                    LexerMode::Punctuator(t, i) => self.parse_punctuator(c, t, i),
                    LexerMode::Number(_) => self.parse_number(),
                    LexerMode::Comment(t) => self.parse_comment(c, t),
                    LexerMode::Raw => self.parse_raw(),
                    LexerMode::Regex(_) => self.parse_regex(),
                    LexerMode::EOF => true
                }
            }
            if self.mode() == LexerMode::EOF {
                break;
            }
        }
        let tokens = self.tokens();
        Ok(tokens)
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
                char::from_u32(i)
            },
            _ => None
        }
    }
    pub fn overwrite_current_char(&mut self, c: char) {
        self.last_char = self.current_char;
        self.current_char = Some(c)
    }

    pub fn col(&mut self) -> i64 {
        self.col
    }

    pub fn line(&mut self) -> i64 {
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
                self.col = 0;
            }
            _ => {
                self.col += 1;
            }
        }
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

    pub fn push(&mut self, t: TokenType) {
        match t {
            TokenType::CommentLiteral(_) => (),
            _ => {
                self.last_token = Some(t.clone())
            }
        }
        self.tokens.push(t)
    }

    pub fn tokens(&self) -> Vec<TokenType> {
        self.tokens.clone()
    }
}