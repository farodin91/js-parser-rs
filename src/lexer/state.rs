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
    current_char: Option<char>
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
            current_char: None
        }
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
        self.last_token = Some(t.clone());
        self.tokens.push(t)
    }

    pub fn tokens(&self) -> Vec<TokenType> {
        self.tokens.clone()
    }
}