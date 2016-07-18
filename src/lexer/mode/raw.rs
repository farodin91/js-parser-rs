use lexer::enums::{LexerMode, TokenType, Keyword, LiteralType};
use lexer::state::{LexerState};

impl LexerState {
    fn raw(&mut self) {
        self.update(LexerMode::None);
        let tmp = self.tmp();
        let tmp = tmp.as_str();
        let token = match tmp {
            "var" => TokenType::Keyword(Keyword::Var),
            "if" => TokenType::Keyword(Keyword::If),
            "else" => TokenType::Keyword(Keyword::Else),
            "do" => TokenType::Keyword(Keyword::Do),
            "typeof" => TokenType::Keyword(Keyword::Typeof),
            "switch" => TokenType::Keyword(Keyword::Switch),
            "catch" => TokenType::Keyword(Keyword::Catch),
            "try" => TokenType::Keyword(Keyword::Try),
            "instanceof" => TokenType::Keyword(Keyword::Instanceof),
            "export" => TokenType::Keyword(Keyword::Export),
            "return" => TokenType::Keyword(Keyword::Return),
            "void" => TokenType::Keyword(Keyword::Void),
            "extends" => TokenType::Keyword(Keyword::Extends),
            "const" => TokenType::Keyword(Keyword::Const),
            "finally" => TokenType::Keyword(Keyword::Finally),
            "super" => TokenType::Keyword(Keyword::Super),
            "with" => TokenType::Keyword(Keyword::With),
            "yield" => TokenType::Keyword(Keyword::Yield),
            "default" => TokenType::Keyword(Keyword::Default),
            "function" => TokenType::Keyword(Keyword::Function),
            "of" => TokenType::Keyword(Keyword::Of),
            "in" => TokenType::Keyword(Keyword::In),
            "for" => TokenType::Keyword(Keyword::For),
            "while" => TokenType::Keyword(Keyword::While),
            "class" => TokenType::Keyword(Keyword::Class),
            "break" => TokenType::Keyword(Keyword::Break),
            "continue" => TokenType::Keyword(Keyword::Continue),
            "new" => TokenType::Keyword(Keyword::New),
            "true" => TokenType::Literal(LiteralType::Boolean(true)),
            "false" => TokenType::Literal(LiteralType::Boolean(false)),
            "null" => TokenType::Literal(LiteralType::Null),
            tmp => {
                TokenType::SymbolLiteral(String::from(tmp))
            }
        };
        self.push(token);
    }
}

pub fn exec(state: &mut LexerState, c: Option<char>) -> bool {
    match c {
        Some('a' ... 'z') | Some('A' ... 'Z') | Some('_') | Some('$') | Some('0' ... '9') => {
            state.tmp_push(c.unwrap());
            true
        }
        Some(' ') |
        Some('\n') |
        Some('\t') |
        None => {
            state.raw();
            true
        }
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
        Some('&') => {
            state.raw();
            false
        }
        _ => {
            println!("Unhandled Parser State Reached: {:?}, {:?}, {:?}", c, state.mode(), state.is_escaped());
            state.update(LexerMode::EOF);
            true
        }
    }
}