use lexer::token::Token;
use lexer::enums::{TokenType, LiteralType, Keyword, Punctuator};
use error::error::{Error, ErrorType, SyntaxErrorType, CodePos};

#[derive(Debug, Clone, PartialEq, Copy)]
enum Position {
    Prefix,
    Content,
    Postfix
}

#[derive(Debug, Clone, PartialEq, Copy)]
enum FunctionStatementPosition {
    Function,
    BindingIdentifier,
    None
}

#[derive(Debug, Clone, PartialEq, Copy)]
enum StatementListType {
    None,
    Function
}

#[derive(Debug, Clone, PartialEq, Copy)]
enum DeclareVariablePosition {
    Symbol,
    AssignmentExpression,
    None
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum CurrentState {
    String,
    None,
    EOF
}

pub struct JsScope {
    state: CurrentState,
    current_token: Option<Token>,
    tokens: TokenIterator,
    levels: Vec<StatementListType>
}

pub type TokenIterator = Box<Iterator<Item = Token>>;

impl JsScope {
    fn new(tokens: TokenIterator) -> JsScope {
        JsScope {
            state: CurrentState::None,
            current_token: None,
            tokens: tokens,
            levels: Vec::new()
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
        scope.handle_statement_list(false, StatementListType::None)
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

    fn handle_formal_parameters(&mut self) -> Result<bool, Error> {
        let mut pos = Position::Prefix;
        loop {
            let token = match self.current_token() {
                Some(t) => t,
                None => {
                    self.state = CurrentState::EOF;
                    return Ok(false)
                }
            };
            println!("handle_formal_parameters {:?}", token);
            let token_type = token.clone().token;
            match (token_type.clone(), pos) {
                (TokenType::LineTerminate, _) => (),
                (TokenType::CommentLiteral(_), _) => (),
                (TokenType::Punctuator(Punctuator::LeftParen), Position::Prefix) => {
                    pos = Position::Content
                }
                (TokenType::SymbolLiteral(_), Position::Content) => (),
                (TokenType::Comma, Position::Content) => (),
                (TokenType::Punctuator(Punctuator::RightParen), Position::Content) => {
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

    fn handle_function_statement(&mut self) -> Result<bool, Error> {
        let mut current_pos = FunctionStatementPosition::None;
        loop {
            let token = match self.current_token() {
                Some(t) => t,
                None => {
                    self.state = CurrentState::EOF;
                    return Ok(false)
                }
            };
            println!("prefix function_statement {:?}", token);
            let token_type = token.clone().token;
            match (token_type.clone(), current_pos) {
                (TokenType::LineTerminate, _) => (),
                (TokenType::CommentLiteral(_), _) => (),
                (TokenType::Keyword(Keyword::Function), FunctionStatementPosition::None) => {
                    current_pos = FunctionStatementPosition::BindingIdentifier;
                }
                (TokenType::SymbolLiteral(_), FunctionStatementPosition::BindingIdentifier) => {
                    current_pos = FunctionStatementPosition::Function;
                },
                (TokenType::Punctuator(Punctuator::Multiple), FunctionStatementPosition::BindingIdentifier) => (),
                (TokenType::Punctuator(Punctuator::LeftParen), FunctionStatementPosition::BindingIdentifier) |
                (TokenType::Punctuator(Punctuator::LeftParen), FunctionStatementPosition::Function) => {
                    let result = self.handle_formal_parameters();
                    match result {
                        Ok(_) => (),
                        Err(t) => {
                            return Err(t)
                        }
                    }
                    println!("  function_statement {:?}", token);
                    let result = self.handle_function_body();
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

    fn handle_declare_variable(&mut self) -> Result<bool, Error> {
        let mut current_pos = DeclareVariablePosition::None;
        loop {
            let token = match self.current_token() {
                Some(t) => t,
                None => {
                    self.state = CurrentState::EOF;
                    return Ok(false)
                }
            };
            println!("handle_declare_variable {:?} {:?}", token, current_pos);
            let token_type = token.clone().token;
            match (token_type.clone(), current_pos) {
                (TokenType::LineTerminate, _) => (),
                (TokenType::CommentLiteral(_), _) => (),
                (TokenType::Keyword(Keyword::Var), DeclareVariablePosition::None) => {
                    current_pos = DeclareVariablePosition::Symbol
                }
                (TokenType::SymbolLiteral(_), DeclareVariablePosition::Symbol) => {
                    current_pos = DeclareVariablePosition::AssignmentExpression
                }
                (TokenType::Comma, DeclareVariablePosition::AssignmentExpression) => {
                    current_pos = DeclareVariablePosition::Symbol
                },
                (TokenType::Punctuator(Punctuator::Equal), DeclareVariablePosition::AssignmentExpression) |
                (TokenType::Semicolon, DeclareVariablePosition::AssignmentExpression) => {
                    self.next_token();
                    return Ok(false)
                }
                _ => {
                    let (line, col) = token.location();
                    return Err(Error::new(ErrorType::SyntaxError(SyntaxErrorType::Unexpected(token_type)), col, line, None))
                }
            }
            self.next_token();
        }
    }

    fn handle_expression_statement(&mut self) -> Result<bool, Error> {
        let mut i = 0;
        let mut has_content = 0;
        let mut pos = Position::Prefix;
        loop {
            let token = match self.current_token() {
                Some(t) => t,
                None => {
                    self.state = CurrentState::EOF;
                    return Ok(false)
                }
            };
            println!("handle_expression_statement {:?} {:?} {:?}", token, pos, i);
            let token_type = token.clone().token;
            match (token_type.clone(), pos) {
                (TokenType::Punctuator(Punctuator::LeftParen), Position::Prefix) => {
                    pos = Position::Content;
                }
                (TokenType::Punctuator(Punctuator::LeftParen), Position::Content) => {
                    i += 1;
                }
                (TokenType::Punctuator(Punctuator::RightParen), Position::Content) => {
                    i -= 1;
                    if i == -1 {
                        if has_content != 0 {
                            self.next_token();
                            break
                        } else {
                            let (line, col) = token.location();
                            return Err(Error::new(ErrorType::SyntaxError(SyntaxErrorType::Unexpected(token_type)), col, line, None))
                        }
                    }
                }
                (_, Position::Content) => {
                    has_content +=1;
                }
                _ => {
                    let (line, col) = token.location();
                    return Err(Error::new(ErrorType::SyntaxError(SyntaxErrorType::Unexpected(token_type)), col, line, None))
                }
                //TokenType::Punctuator(Punctuator::LeftBrace) => {
                //}
            }
            self.next_token();
        }
        Ok(true)
    }

    fn handle_if_statement(&mut self) -> Result<bool, Error> {
        println!("prefix if_statement {:?}", self.current_token());
        self.next_token();
        let result = self.handle_expression_statement();
        match result {
            Ok(_) => (),
            Err(t) => return Err(t)
        };
        let result = self.handle_statement();
        match result {
            Ok(_) => (),
            Err(t) => return Err(t)
        }
        self.step_over_line_and_comment();

        let token = match self.current_token() {
            Some(t) => t,
            None => {
                self.state = CurrentState::EOF;
                return Ok(false)
            }
        };
        let token_type = token.clone().token;
        println!("  if_statement {:?}", self.current_token());
        match token_type {
            TokenType::Keyword(Keyword::Else) => {
                self.next_token();
                let result = self.handle_statement();
                match result {
                    Ok(_) => return Ok(true),
                    Err(t) => {
                        return Err(t)
                    }
                };
            }
            _ => return Ok(false)
        }

    }

    fn handle_with_statement(&mut self) -> Result<bool, Error> {
        println!("handle_with_statement {:?}", self.current_token());
        self.next_token();
        let result = self.handle_expression_statement();
        match result {
            Ok(_) => (),
            Err(t) => {
                return Err(t)
            }
        };
        let result = self.handle_statement();
        match result {
            Ok(_) => Ok(true),
            Err(t) => Err(t)
        }
    }

    fn handle_function_body(&mut self) -> Result<bool, Error> {
        println!("    prefix handle_function_body {:?}", self.current_token());
        self.step_over_line_and_comment();
        let token = match self.current_token() {
            Some(t) => t,
            None => {
                return Err(Error::new(ErrorType::SyntaxError(SyntaxErrorType::UnexpectedEOF),0,0,None))
            }
        };
        if token.token.clone() != TokenType::Punctuator(Punctuator::LeftBrace) {
            let (line, col) = token.location();
            return Err(Error::new(ErrorType::SyntaxError(SyntaxErrorType::Unexpected(token.token.clone())), col, line, None))
        }

        self.next_token();
        let result = self.handle_statement_list(true, StatementListType::Function);
        match result {
            Ok(_) => (),
            Err(t) => {
                return Err(t)
            }
        }
        println!("    postfix handle_function_body {:?}", self.current_token());
        self.next_token();
        Ok(true)
    }

    fn handle_for_iteration_statement(&mut self) -> Result<bool, Error> {
        println!("handle_for_iteration_statement {:?}", self.current_token());
        self.next_token();
        let result = self.handle_expression_statement();
        match result {
            Ok(_) => (),
            Err(t) => {
                return Err(t)
            }
        };
        let result = self.handle_statement();
        match result {
            Ok(_) => Ok(true),
            Err(t) => Err(t)
        }
    }

    fn handle_while_iteration_statement(&mut self) -> Result<bool, Error> {
        println!("handle_while_iteration_statement {:?}", self.current_token());
        self.next_token();
        let result = self.handle_expression_statement();
        match result {
            Ok(_) => (),
            Err(t) => {
                return Err(t)
            }
        };
        let result = self.handle_statement();
        match result {
            Ok(_) => Ok(true),
            Err(t) => Err(t)
        }
    }

    fn step_over_line_and_comment(&mut self) {
        loop {
            let token = match self.current_token() {
                Some(t) => t,
                None => {
                    return
                }
            };
            let token_type = token.clone().token;
            match token_type {
                TokenType::LineTerminate => (),
                TokenType::CommentLiteral(_) => (),
                _ => break
            }
            self.next_token();
        }
    }

    fn handle_do_while_iteration_statement(&mut self) -> Result<bool, Error> {
        println!("prefix do_while_statement {:?} ", self.current_token());
        self.next_token();
        self.step_over_line_and_comment();
        let result = self.handle_statement();
        match result {
            Ok(_) => (),
            Err(t) => {
                return Err(t)
            }
        }
        self.step_over_line_and_comment();
        self.next_token();
        let result = self.handle_expression_statement();
        match result {
            Ok(_) => (),
            Err(t) => {
                return Err(t)
            }
        }
        Ok(true)
    }

    fn handle_try_statement(&mut self) -> Result<bool, Error> {
        println!("prefix try_statement {:?} ", self.current_token());
        self.next_token();//try
        self.step_over_line_and_comment();// block
        let result = self.handle_block();
        match result {
            Ok(_) => (),
            Err(t) => {
                return Err(t)
            }
        }
        self.step_over_line_and_comment();

        let token = match self.current_token() {
            Some(t) => t,
            None => {
                self.state = CurrentState::EOF;
                return Ok(false)
            }
        };
        let token_type = token.clone().token;
        println!("  try_statement {:?}", self.current_token());
        match token_type {
            TokenType::Keyword(Keyword::Catch) => {
                self.next_token();
                let result = self.handle_catch_parameter();
                match result {
                    Ok(_) => (),
                    Err(t) => {
                        return Err(t)
                    }
                };
                //println!("  try_statement {:?}", self.current_token());
                let result = self.handle_block();
                self.step_over_line_and_comment();
                match result {
                    Ok(_) => (),
                    Err(t) => {
                        return Err(t)
                    }
                }
            },
            TokenType::Keyword(Keyword::Finally) => {
                self.next_token();
                let result = self.handle_block();
                return match result {
                    Ok(_) => Ok(true),
                    Err(t) => Err(t)
                }
            }
            _ => {
                return Ok(true)
            }
        }
        self.step_over_line_and_comment();

        let token = match self.current_token() {
            Some(t) => t,
            None => {
                self.state = CurrentState::EOF;
                return Ok(false)
            }
        };
        let token_type = token.clone().token;
        println!("  try_statement {:?}", self.current_token());
        match token_type {
            TokenType::Keyword(Keyword::Finally) => {
                self.next_token();
                let result = self.handle_block();
                return match result {
                    Ok(_) => Ok(true),
                    Err(t) => Err(t)
                }
            }
            _ => {
                return Ok(false)
            }
        }
    }

    fn handle_switch_statement(&mut self) -> Result<bool, Error> {
        println!("handle_switch_statement {:?}", self.current_token());
        self.next_token();
        let result = self.handle_expression_statement();
        match result {
            Ok(_) => (),
            Err(t) => {
                return Err(t)
            }
        };
        let result = self.handle_block();
        match result {
            Ok(_) => Ok(true),
            Err(t) => Err(t)
        }
    }

    fn handle_statement(&mut self) -> Result<(), Error> {
        self.state = CurrentState::None;
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
                (TokenType::LineTerminate, 0) => {
                    line += 1;
                },
                (TokenType::CommentLiteral(_), _) => (),
                (TokenType::Punctuator(Punctuator::LeftBrace), _) => {
                    let result = self.handle_block();
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
                    let result = self.handle_declare_variable();
                    self.state = CurrentState::None;
                    match result {
                        Ok(_) => break,
                        Err(t) => {
                            return Err(t)
                        }
                    }
                }
                (TokenType::Semicolon, 0 ... 1) => {
                    self.state = CurrentState::None;
                    break;
                }
                (TokenType::Punctuator(Punctuator::LeftParen), 0 ... 1) => {
                    let result = self.handle_expression_statement();
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

    fn handle_block(&mut self) -> Result<bool, Error> {
        println!("    prefix handle_block {:?}", self.current_token());
        self.step_over_line_and_comment();
        let token = match self.current_token() {
            Some(t) => t,
            None => {
                return Err(Error::new(ErrorType::SyntaxError(SyntaxErrorType::UnexpectedEOF),0,0,None))
            }
        };
        if token.token.clone() != TokenType::Punctuator(Punctuator::LeftBrace) {
            let (line, col) = token.location();
            return Err(Error::new(ErrorType::SyntaxError(SyntaxErrorType::Unexpected(token.token.clone())), col, line, None))
        }

        self.next_token();
        let result = self.handle_statement_list(true, StatementListType::None);
        match result {
            Ok(_) => (),
            Err(t) => {
                return Err(t)
            }
        }
        println!("    postfix handle_block {:?}", self.current_token());
        self.next_token();
        Ok(true)
    }

    fn handle_statement_list(&mut self, internal: bool, list_type: StatementListType) -> Result<(), Error> {
        self.levels.push(list_type);
        if !internal {
            self.next_token();
        }
        loop {
            let token = match self.current_token() {
                Some(t) => t,
                None => {
                    self.levels.pop();
                    return Ok(())
                }
            };
            println!("handle_statement_list {:?} {:?}", token, self.state);
            let token_type = token.clone().token;
            let result = match (token_type.clone(), self.state) {
                (TokenType::Literal(LiteralType::String(_)), CurrentState::None) => {
                    self.state = CurrentState::String;
                    Ok(true)
                }
                (TokenType::Keyword(Keyword::Var), _) => {
                    self.handle_declare_variable()
                }
                (TokenType::Keyword(Keyword::Try), CurrentState::None) => {
                    self.handle_try_statement()
                }
                (TokenType::Keyword(Keyword::If), CurrentState::None) => {
                    self.handle_if_statement()
                }
                (TokenType::Keyword(Keyword::With), CurrentState::None) => {
                    self.handle_with_statement()
                }
                (TokenType::Keyword(Keyword::Function), CurrentState::None) => {
                    self.handle_function_statement()
                }
                (TokenType::Punctuator(Punctuator::LeftBrace), CurrentState::None) => {
                    self.handle_block()
                }
                (TokenType::Punctuator(Punctuator::RightBrace), CurrentState::None) => {
                    self.levels.pop();
                    return Ok(())
                }
                (TokenType::Keyword(Keyword::While), _) => {
                    self.handle_while_iteration_statement()
                }
                (TokenType::Keyword(Keyword::For), _) => {
                    self.handle_for_iteration_statement()
                }
                (TokenType::Keyword(Keyword::Do), _) => {
                    self.handle_do_while_iteration_statement()
                }
                (TokenType::Keyword(Keyword::Switch), _) => {
                    self.handle_switch_statement()
                }
                (TokenType::Keyword(Keyword::Return), _) => {
                    let levels = self.levels.clone();
                    let mut function = false;
                    for list_type in levels {
                        if list_type == StatementListType::Function {
                            function = true;
                            break
                        }
                    }
                    if !function {
                        let (line, col) = token.location();
                        return Err(Error::new(ErrorType::SyntaxError(SyntaxErrorType::Unexpected(token_type)), col, line, None))
                    }
                    Ok(true)
                }
                (TokenType::Keyword(Keyword::Catch), _) |
                (TokenType::Keyword(Keyword::Finally), _) |
                (TokenType::Literal(LiteralType::String(_)), CurrentState::String) => {
                    let (line, col) = token.location();
                    Err(Error::new(ErrorType::SyntaxError(SyntaxErrorType::Unexpected(token_type)), col, line, None))
                }
                _ => {
                    self.state = CurrentState::None;
                    Ok(true)
                }
            };
            let result = match result {
                Ok(t) => t,
                Err(t) => {
                    return Err(t)
                }
            };
            if result {
                self.next_token();
            }
        }
    }
}