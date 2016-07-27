use lexer::token::Token;
use lexer::enums::{TokenType, LiteralType, Keyword, Punctuator};
use error::error::{Error, ErrorType, SyntaxErrorType, CodePos};

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum IterationStatementPosition {
    Statement,
    Expression,
    None
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum DeclareVariablePosition {
    Symbol,
    AssignmentExpression,
    None
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum CurrentState {
    String,
    DeclareVariable(DeclareVariablePosition),
    IterationStatement(IterationStatementPosition),
    None
}

pub struct JsScope {
    state: CurrentState,
    tokens: TokenIterator
}

pub type TokenIterator = Box<Iterator<Item = Token>>;

impl JsScope {
    fn new(tokens: TokenIterator) -> JsScope {
        JsScope {
            state: CurrentState::None,
            tokens: tokens
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    pub fn from_tokens(tokens: Vec<Token>) -> Result<(), Error> {
        let iter = Box::new(tokens.into_iter());
        let scope = &mut JsScope::new(iter);
        loop {
            let result = match scope.state {
                CurrentState::IterationStatement(pos) => {
                    scope.handle_iteration_statement(pos)
                }
                CurrentState::DeclareVariable(pos) => {
                    scope.handle_declare_variable(pos)
                }
                _ => {
                    scope.handle_token()
                }
            };
            match result {
                Ok(true) => (),
                Ok(false) => break,
                Err(err) => { return Err(err) }
            }
        }
        Ok(())
    }

    fn handle_declare_variable(&mut self, pos: DeclareVariablePosition) -> Result<bool, Error> {
        let mut handled = true;
        let mut current_pos = pos;
        loop {
            let token = match self.next_token() {
                Some(t) => t,
                None => {
                    handled = false;
                    break
                }
            };
            //println!("scope {:?} {:?}", self.state, token);
            let token_type = token.clone().token;
            match (token_type.clone(), current_pos) {
                (TokenType::SymbolLiteral(_), DeclareVariablePosition::None) => {
                    self.state = CurrentState::DeclareVariable(DeclareVariablePosition::Symbol);
                    current_pos = DeclareVariablePosition::Symbol
                }
                (TokenType::Comma, DeclareVariablePosition::Symbol) => {
                    self.state = CurrentState::DeclareVariable(DeclareVariablePosition::None);
                    current_pos = DeclareVariablePosition::None
                },
                (TokenType::Punctuator(Punctuator::Equal), DeclareVariablePosition::Symbol) |
                (TokenType::Semicolon, DeclareVariablePosition::Symbol) => {
                    self.state = CurrentState::None;
                    break
                }
                _ => {
                    let (line, col) = token.location();
                    return Err(Error::new(ErrorType::SyntaxError(SyntaxErrorType::Unexpected(token_type)), col, line, None))
                }
            }
        }
        Ok(handled)
    }

    fn handle_iteration_statement(&mut self, pos: IterationStatementPosition) -> Result<bool, Error> {
        let mut handled = true;
        let mut current_pos = pos;
        loop {
            let token = match self.next_token() {
                Some(t) => t,
                None => {
                    handled = false;
                    break
                }
            };
            //println!("scope {:?} {:?}", self.state, token);
            let token_type = token.clone().token;
            match (token_type.clone(), current_pos) {
                (TokenType::LineTerminate, _) => (),
                (TokenType::Punctuator(Punctuator::LeftParen), IterationStatementPosition::None) => {
                    self.state = CurrentState::IterationStatement(IterationStatementPosition::Expression);
                    current_pos = IterationStatementPosition::Expression
                }
                (TokenType::Punctuator(Punctuator::RightParen), IterationStatementPosition::Expression) => {
                    self.state = CurrentState::None;
                    break
                }
                (TokenType::Punctuator(Punctuator::LeftBrace), IterationStatementPosition::Expression) => {
                    let (line, col) = token.location();
                    return Err(Error::new(ErrorType::SyntaxError(SyntaxErrorType::Unexpected(token_type)), col, line, None))
                },
                (_, IterationStatementPosition::Expression) => (),
                _ => {
                    let (line, col) = token.location();
                    return Err(Error::new(ErrorType::SyntaxError(SyntaxErrorType::Unexpected(token_type)), col, line, None))
                }
            }
        }
        Ok(handled)
    }

    fn handle_token(&mut self) -> Result<bool, Error> {
        let token = match self.next_token() {
            Some(t) => t,
            None => {
                return Ok(false)
            }
        };
        let (line, col) = token.location();
        let token = token.token;
        match (token.clone(), self.state) {
            (TokenType::Literal(LiteralType::String(_)), CurrentState::None) => {
                self.state = CurrentState::String
            }
            (TokenType::Keyword(Keyword::Var), _) => {
                self.state = CurrentState::DeclareVariable(DeclareVariablePosition::None)
            }
            (TokenType::Keyword(Keyword::While), _) => {
                self.state = CurrentState::IterationStatement(IterationStatementPosition::None)
            }
            (TokenType::Literal(LiteralType::String(_)), CurrentState::String) => {
                return Err(Error::new(ErrorType::SyntaxError(SyntaxErrorType::Unexpected(token)), col, line, None))
            }
            _ => {
                self.state = CurrentState::None
            }
        }
        Ok(true)
    }
}