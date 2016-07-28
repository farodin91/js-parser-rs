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
pub enum Position {
    Prefix,
    Content,
    Postfix
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum IterationType {
    DoWhile,
    While,
    For
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum TryStatementPosition {
    Try,
    Catch,
    Finally,
    None
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum StatementPosition {
    Initial,
    None
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum FunctionStatementPosition {
    Function,
    Symbol,
    None
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum IfStatementPosition {
    If,
    Else,
    ElseIf,
    None
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum WithStatementPosition {
    With,
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
    IterationStatement(IterationType),
    If,
    With,
    Function,
    Try,
    None,
    EOF
}

pub struct JsScope {
    state: CurrentState,
    current_token: Option<Token>,
    tokens: TokenIterator
}

pub type TokenIterator = Box<Iterator<Item = Token>>;

impl JsScope {
    fn new(tokens: TokenIterator) -> JsScope {
        JsScope {
            state: CurrentState::None,
            current_token: None,
            tokens: tokens
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let token = self.tokens.next();
        self.current_token = token.clone();
        token
    }

    pub fn current_token(&self) -> Option<Token> {
        self.current_token.clone()
    }

    pub fn from_tokens(tokens: Vec<Token>) -> Result<(), Error> {
        let iter = Box::new(tokens.into_iter());
        let scope = &mut JsScope::new(iter);
        scope.handle_block(false)
    }

    fn handle_catch_parameter(&mut self) -> Result<bool, Error> {
        let mut pos = Position::Prefix;
        loop {
            let token = match self.current_token() {
                Some(t) => t,
                None => {
                    self.state = CurrentState::EOF;
                    return Ok(false)
                }
            };
            println!("handle_catch_parameter {:?}", token);
            let token_type = token.clone().token;
            match (token_type.clone(), pos) {
                (TokenType::LineTerminate, _) => (),
                (TokenType::CommentLiteral(_), _) => (),
                (TokenType::Punctuator(Punctuator::LeftParen), Position::Prefix) => {
                    pos = Position::Content
                }
                (TokenType::SymbolLiteral(_), Position::Content) => {
                    pos = Position::Postfix
                }
                (TokenType::Punctuator(Punctuator::RightParen), Position::Postfix) => {
                    self.next_token();
                    break
                }
                _ => {
                    let (line, col) = token.location();
                    return Err(Error::new(ErrorType::SyntaxError(SyntaxErrorType::Unexpected(token_type)), col, line, None))
                }
            }
            self.next_token();
        }
        Ok(true)
    }

    fn handle_declare_variable(&mut self, pos: DeclareVariablePosition) -> Result<bool, Error> {
        let mut current_pos = pos;
        loop {
            let token = match self.current_token() {
                Some(t) => t,
                None => {
                    self.state = CurrentState::EOF;
                    return Ok(false)
                }
            };
            println!("handle_declare_variable {:?} {:?}", pos, token);
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
                    self.next_token();
                    break
                }
                _ => {
                    let (line, col) = token.location();
                    return Err(Error::new(ErrorType::SyntaxError(SyntaxErrorType::Unexpected(token_type)), col, line, None))
                }
            }
            self.next_token();
        }
        Ok(true)
    }

    fn handle_expression_statement(&mut self) -> Result<bool, Error> {
        let mut i = 1;
        loop {
            let token = match self.current_token() {
                Some(t) => t,
                None => {
                    self.state = CurrentState::EOF;
                    return Ok(false)
                }
            };
            println!("handle_expression_statement {:?}", token);
            let token_type = token.clone().token;
            match token_type.clone() {
                TokenType::Punctuator(Punctuator::LeftParen) => {
                    i += 1;
                }
                TokenType::Punctuator(Punctuator::RightParen) => {
                    i -= 1;
                    if i == 0 {
                        self.next_token();
                        break;
                    }
                }
                //TokenType::Punctuator(Punctuator::LeftBrace) => {
                //    let (line, col) = token.location();
                //    return Err(Error::new(ErrorType::SyntaxError(SyntaxErrorType::Unexpected(token_type)), col, line, None))
                //}
                _ => ()
            }
            self.next_token();
        }
        Ok(true)
    }

    fn handle_if_statement(&mut self) -> Result<bool, Error> {
        loop {
            let token = match self.current_token() {
                Some(t) => t,
                None => {
                    self.state = CurrentState::EOF;
                    return Ok(false)
                }
            };
            println!("handle_if_statement {:?}", token);
            let token_type = token.clone().token;
            match token_type.clone() {
                TokenType::LineTerminate => (),
                TokenType::CommentLiteral(_) => (),
                TokenType::Punctuator(Punctuator::LeftParen) => {
                    self.next_token();
                    let result = self.handle_expression_statement();
                    match result {
                        Ok(_) => (),
                        Err(t) => {
                            return Err(t)
                        }
                    };
                    let result = self.handle_statement();
                    self.state = CurrentState::None;
                    match result {
                        Ok(_) => break,
                        Err(t) => {
                            return Err(t)
                        }
                    }
                }
                _ => {
                    let (line, col) = token.location();
                    return Err(Error::new(ErrorType::SyntaxError(SyntaxErrorType::Unexpected(token_type)), col, line, None))
                }
            }
            self.next_token();
        }
        Ok(true)
    }

    fn handle_with_statement(&mut self) -> Result<bool, Error> {
        loop {
            let token = match self.current_token() {
                Some(t) => t,
                None => {
                    self.state = CurrentState::EOF;
                    return Ok(false)
                }
            };
            println!("handle_with_statement {:?}", token);
            let token_type = token.clone().token;
            match token_type.clone() {
                TokenType::LineTerminate => (),
                TokenType::CommentLiteral(_) => (),
                TokenType::Punctuator(Punctuator::LeftParen) => {
                    let result = self.handle_expression_statement();
                    match result {
                        Ok(_) => (),
                        Err(t) => {
                            return Err(t)
                        }
                    };
                    let result = self.handle_statement();
                    self.state = CurrentState::None;
                    match result {
                        Ok(_) => break,
                        Err(t) => {
                            return Err(t)
                        }
                    }
                }
                _ => {
                    let (line, col) = token.location();
                    return Err(Error::new(ErrorType::SyntaxError(SyntaxErrorType::Unexpected(token_type)), col, line, None))
                }
            }
            self.next_token();
        }
        Ok(true)
    }

    fn handle_function_statement(&mut self) -> Result<bool, Error> {
        loop {
            let token = match self.current_token() {
                Some(t) => t,
                None => {
                    self.state = CurrentState::EOF;
                    return Ok(false)
                }
            };
            println!("handle_function_statement {:?}", token);
            let token_type = token.clone().token;
            match token_type.clone() {
                TokenType::LineTerminate => (),
                TokenType::CommentLiteral(_) => (),
                TokenType::Punctuator(Punctuator::LeftParen) => {
                    self.next_token();
                    let result = self.handle_expression_statement();
                    match result {
                        Ok(_) => (),
                        Err(t) => {
                            return Err(t)
                        }
                    }
                    let result = self.handle_statement();
                    self.state = CurrentState::None;
                    match result {
                        Ok(_) => break,
                        Err(t) => {
                            return Err(t)
                        }
                    }
                }
                TokenType::SymbolLiteral(_) => (),
                _ => {
                    let (line, col) = token.location();
                    return Err(Error::new(ErrorType::SyntaxError(SyntaxErrorType::Unexpected(token_type)), col, line, None))
                }
            }
            self.next_token();
        }
        Ok(true)
    }

    fn handle_for_iteration_statement(&mut self) -> Result<bool, Error> {
        loop {
            let token = match self.current_token() {
                Some(t) => t,
                None => {
                    self.state = CurrentState::EOF;
                    return Ok(false)
                }
            };
            println!("handle_for_iteration_statement {:?}", token);
            let token_type = token.clone().token;
            match token_type.clone() {
                TokenType::LineTerminate => (),
                TokenType::CommentLiteral(_) => (),
                TokenType::Punctuator(Punctuator::LeftParen) => {
                    self.next_token();
                    let result = self.handle_expression_statement();
                    match result {
                        Ok(_) => (),
                        Err(t) => {
                            return Err(t)
                        }
                    };
                    let result = self.handle_statement();
                    self.state = CurrentState::None;
                    match result {
                        Ok(_) => break,
                        Err(t) => {
                            return Err(t)
                        }
                    }
                }
                _ => {
                    let (line, col) = token.location();
                    return Err(Error::new(ErrorType::SyntaxError(SyntaxErrorType::Unexpected(token_type)), col, line, None))
                }
            }
            self.next_token();
        }
        Ok(true)
    }

    fn handle_while_iteration_statement(&mut self) -> Result<bool, Error> {
        loop {
            let token = match self.current_token() {
                Some(t) => t,
                None => {
                    self.state = CurrentState::EOF;
                    return Ok(false)
                }
            };
            println!("handle_while_iteration_statement {:?}", token);
            let token_type = token.clone().token;
            match token_type.clone() {
                TokenType::LineTerminate => (),
                TokenType::CommentLiteral(_) => (),
                TokenType::Punctuator(Punctuator::LeftParen) => {
                    self.next_token();
                    let result = self.handle_expression_statement();
                    match result {
                        Ok(_) => (),
                        Err(t) => {
                            return Err(t)
                        }
                    };
                    let result = self.handle_statement();
                    self.state = CurrentState::None;
                    match result {
                        Ok(_) => break,
                        Err(t) => {
                            return Err(t)
                        }
                    }
                }
                _ => {
                    let (line, col) = token.location();
                    return Err(Error::new(ErrorType::SyntaxError(SyntaxErrorType::Unexpected(token_type)), col, line, None))
                }
            }
            self.next_token();
        }
        Ok(true)
    }

    fn handle_do_while_iteration_statement(&mut self) -> Result<bool, Error> {
        println!("handle_do_while_iteration_statement {:?}", self.current_token());
        let result = self.handle_statement();
        self.state = CurrentState::None;
        match result {
            Ok(_) => (),
            Err(t) => {
                return Err(t)
            }
        }
        loop {
            let token = match self.current_token() {
                Some(t) => t,
                None => {
                    self.state = CurrentState::EOF;
                    return Ok(false)
                }
            };
            println!("handle_do_while_iteration_statement {:?}", token);
            let token_type = token.clone().token;
            match token_type.clone() {
                TokenType::LineTerminate => (),
                TokenType::CommentLiteral(_) => (),
                TokenType::Keyword(Keyword::While) => {
                    self.next_token();
                    self.state = CurrentState::None;
                    break
                }
                _ => {
                    let (line, col) = token.location();
                    return Err(Error::new(ErrorType::SyntaxError(SyntaxErrorType::Unexpected(token_type)), col, line, None))
                }
            }
            self.next_token();
        }
        Ok(true)
    }

    fn handle_try_statement(&mut self) -> Result<bool, Error> {
        let mut current_pos = TryStatementPosition::None;
        let mut handled = true;
        loop {
            let token = match self.current_token() {
                Some(t) => t,
                None => {
                    match current_pos {
                        TryStatementPosition::None => {
                            return Err(Error::new(ErrorType::SyntaxError(SyntaxErrorType::UnexpectedEOF), 0,0,None))
                        }
                        _ => {
                            self.state = CurrentState::EOF;
                            return Ok(false)
                        }
                    }
                }
            };
            println!("handle_try_statement {:?} {:?}", token, current_pos);
            let token_type = token.clone().token;
            match (token_type.clone(), current_pos) {
                (TokenType::LineTerminate, _) => (),
                (TokenType::CommentLiteral(_), _) => (),
                (TokenType::Semicolon, TryStatementPosition::Try) => {
                    self.next_token();
                    self.state = CurrentState::None;
                    break;
                },
                (TokenType::Punctuator(Punctuator::LeftBrace), TryStatementPosition::None) => {
                    self.next_token();
                    let result = self.handle_block(true);
                    current_pos = TryStatementPosition::Try;
                    handled = false;
                    match result {
                        Ok(_) => (),
                        Err(t) => {
                            return Err(t)
                        }
                    }
                }
                (TokenType::Punctuator(Punctuator::LeftBrace), TryStatementPosition::Finally) => {
                    self.next_token();
                    let result = self.handle_block(true);
                    self.state = CurrentState::None;
                    match result {
                        Ok(_) => break,
                        Err(t) => {
                            return Err(t)
                        }
                    }
                }
                (TokenType::Keyword(Keyword::Catch), TryStatementPosition::Try) => {
                    self.next_token();
                    let result = self.handle_catch_parameter();
                    match result {
                        Ok(_) => (),
                        Err(t) => {
                            return Err(t)
                        }
                    };
                    let result = self.handle_statement();
                    current_pos = TryStatementPosition::Try;
                    match result {
                        Ok(_) => (),
                        Err(t) => {
                            return Err(t)
                        }
                    }
                }
                (TokenType::Keyword(Keyword::Finally), TryStatementPosition::Try) => {
                    current_pos = TryStatementPosition::Finally
                }
                (_, TryStatementPosition::Try) => break,
                _ => {
                    let (line, col) = token.location();
                    return Err(Error::new(ErrorType::SyntaxError(SyntaxErrorType::Unexpected(token_type)), col, line, None))
                }
            }
            if handled {
                self.next_token();
            }
            handled = true
        }
        Ok(true)
    }

    fn handle_statement(&mut self) -> Result<(), Error> {
        let mut line = 0;
        loop {
            let token = match self.current_token() {
                Some(t) => t,
                None => {
                    return Ok(())
                }
            };
            println!("handle_statement {:?} {:?}", token, line);
            let token_type = token.clone().token;
            match (token_type.clone(), line) {
                (TokenType::LineTerminate, _) => {
                    line += 1;
                },
                (TokenType::CommentLiteral(_), _) => (),
                (TokenType::Punctuator(Punctuator::LeftBrace), _) => {
                    self.next_token();
                    let result = self.handle_block(true);
                    self.state = CurrentState::None;
                    match result {
                        Ok(_) => break,
                        Err(t) => {
                            return Err(t)
                        }
                    }
                }
                (TokenType::SymbolLiteral(_), 0 ... 1) => {
                    self.state = CurrentState::None;
                    break
                }
                (TokenType::Keyword(Keyword::Break), 0 ... 1) => {
                    self.state = CurrentState::None;
                    break
                }
                (TokenType::Keyword(Keyword::Var), 0 ... 1) => {
                    self.next_token();
                    let result = self.handle_declare_variable(DeclareVariablePosition::None);
                    self.state = CurrentState::None;
                    match result {
                        Ok(_) => break,
                        Err(t) => {
                            return Err(t)
                        }
                    }
                }
                _ => {
                    let (line, col) = token.location();
                    return Err(Error::new(ErrorType::SyntaxError(SyntaxErrorType::Unexpected(token_type)), col, line, None))
                }
            }
            self.next_token();
        }
        Ok(())
    }

    fn handle_block(&mut self, internal: bool) -> Result<(), Error> {
        if !internal {
            self.next_token();
        }
        self.state = CurrentState::None;
        loop {
            let result = match self.state {
                CurrentState::IterationStatement(IterationType::While) => {
                    self.handle_while_iteration_statement()
                }
                CurrentState::IterationStatement(IterationType::For) => {
                    self.handle_for_iteration_statement()
                }
                CurrentState::DeclareVariable(pos) => {
                    self.handle_declare_variable(pos)
                }
                CurrentState::Try => {
                    self.handle_try_statement()
                }
                CurrentState::If => {
                    self.handle_if_statement()
                }
                CurrentState::With => {
                    self.handle_with_statement()
                }
                CurrentState::IterationStatement(IterationType::DoWhile) => {
                    self.handle_do_while_iteration_statement()
                }
                CurrentState::Function => {
                    self.handle_function_statement()
                }
                CurrentState::EOF => break,
                _ => {
                    self.handle_token(internal)
                }
            };
            //println!("handle_block_statement {:?} {:?} {:?} {:?}", self.current_token(), result, self.state, internal);
            match result {
                Ok(true) => (),
                Ok(false) => break,
                Err(err) => { return Err(err) }
            }
        }
        Ok(())
    }

    fn handle_token(&mut self, internal: bool) -> Result<bool, Error> {
        let token = match self.current_token() {
            Some(t) => t,
            None => {
                self.state = CurrentState::EOF;
                return Ok(false)
            }
        };
        let (line, col) = token.location();
        //println!("handle_token {:?} {:?}", token, self.state);
        let token = token.token;
        match (token.clone(), self.state) {
            (TokenType::Literal(LiteralType::String(_)), CurrentState::None) => {
                self.state = CurrentState::String
            }
            (TokenType::Keyword(Keyword::Var), _) => {
                self.state = CurrentState::DeclareVariable(DeclareVariablePosition::None)
            }
            (TokenType::Keyword(Keyword::Try), CurrentState::None) => {
                self.state = CurrentState::Try
            }
            (TokenType::Keyword(Keyword::If), CurrentState::None) => {
                self.state = CurrentState::If
            }
            (TokenType::Keyword(Keyword::With), CurrentState::None) => {
                self.state = CurrentState::With
            }
            (TokenType::Keyword(Keyword::Function), CurrentState::None) => {
                self.state = CurrentState::Function
            }
            (TokenType::Punctuator(Punctuator::RightBrace), CurrentState::None) => {
                if internal {
                    self.next_token();
                    return Ok(false)
                }
            }
            (TokenType::Keyword(Keyword::While), _) => {
                self.state = CurrentState::IterationStatement(IterationType::While)
            }
            (TokenType::Keyword(Keyword::For), _) => {
                self.state = CurrentState::IterationStatement(IterationType::For)
            }
            (TokenType::Keyword(Keyword::Do), _) => {
                self.state = CurrentState::IterationStatement(IterationType::DoWhile)
            }
            (TokenType::Keyword(Keyword::Catch), CurrentState::None) |
            (TokenType::Keyword(Keyword::Finally), CurrentState::None) |
            (TokenType::Literal(LiteralType::String(_)), CurrentState::String) => {
                return Err(Error::new(ErrorType::SyntaxError(SyntaxErrorType::Unexpected(token)), col, line, None))
            }
            _ => {
                self.state = CurrentState::None
            }
        }
        self.next_token();
        Ok(true)
    }
}