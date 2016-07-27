use lexer::enums::TokenType;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum SyntaxErrorType {
    UnexpectedEOF,
    UnexpectedEOL,
    UnexpectedChar(char),
    Unexpected(TokenType),
    MissingParameter(String)
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum ErrorType {
    SyntaxError(SyntaxErrorType),
    ReferenceError(String)
}

pub trait CodePos {
    fn location(&self) -> (u64, u32);
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Error {
    pub error_type: ErrorType,
    line: u64,
    col: u32,
    expected: Option<&'static str>
}

impl Error {
    pub fn new(etype: ErrorType, col: u32, line: u64, expected: Option<&'static str>) -> Error {
        Error {
            error_type: etype,
            line: line,
            col: col,
            expected: expected
        }
    }

    pub fn from_state<T>(etype: ErrorType, pos: &T, expected: Option<&'static str>) -> Error where T: CodePos {
        let p = pos.location();
        Error::new(etype, p.1, p.0, expected)
    }
}