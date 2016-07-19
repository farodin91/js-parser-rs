use lexer::enums::{TokenType, LexerMode};

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