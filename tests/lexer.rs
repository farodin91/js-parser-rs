extern crate js_parser_rs;

use js_parser_rs::lexer::enums::{TokenType, Punctuator, Keyword, LiteralType, RegexIdentifier};
use std::fs::File;
use std::io::Read;

#[test]
fn test_empty() {
    assert_eq!(js_parser_rs::parse("".chars()), Ok(vec![]));
}

#[test]
fn test_semicolon() {
    assert_eq!(js_parser_rs::parse(";".chars()), Ok(vec![TokenType::Semicolon]));
    assert_eq!(js_parser_rs::parse(";;;".chars()), Ok(vec![TokenType::Semicolon,TokenType::Semicolon,TokenType::Semicolon]));
    assert_eq!(js_parser_rs::parse(";;;;".chars()), Ok(vec![TokenType::Semicolon,TokenType::Semicolon,TokenType::Semicolon,TokenType::Semicolon]));
    assert_eq!(js_parser_rs::parse(";;".chars()), Ok(vec![TokenType::Semicolon,TokenType::Semicolon]));
}

#[test]
fn test_useless_spaces() {
    assert_eq!(js_parser_rs::parse(" ".chars()), Ok(vec![]));
    assert_eq!(js_parser_rs::parse("; ".chars()), Ok(vec![TokenType::Semicolon]));
    assert_eq!(js_parser_rs::parse(",".chars()), Ok(vec![TokenType::Comma]));
}

#[test]
fn test_useless_double_quote_string() {
    assert_eq!(js_parser_rs::parse("\"Hello World!\"".chars()), Ok(vec![TokenType::Literal(LiteralType::String(String::from("Hello World!")))]));
    assert_eq!(js_parser_rs::parse("\"Hel{}\" \"Hello World!\"".chars()),Ok(vec![TokenType::Literal(LiteralType::String(String::from("Hel{}"))),TokenType::Literal(LiteralType::String(String::from("Hello World!")))]));
}

#[test]
fn test_useless_double_quote_escaped_string() {
    assert_eq!(js_parser_rs::parse("\"Hello\\\" World!\"".chars()), Ok(vec![TokenType::Literal(LiteralType::String(String::from("Hello\" World!")))]));
    assert_eq!(js_parser_rs::parse("\"Hello\\\\ World!\"".chars()), Ok(vec![TokenType::Literal(LiteralType::String(String::from("Hello\\ World!")))]));
}

#[test]
fn test_useless_number() {
    assert_eq!(js_parser_rs::parse("0".chars()), Ok(vec![TokenType::Literal(LiteralType::Integer(0))]));
    assert_eq!(js_parser_rs::parse("0xabcdef0".chars()), Ok(vec![TokenType::Literal(LiteralType::Integer(0xabcdef0))]));
    assert_eq!(js_parser_rs::parse("0xABCDEF0".chars()), Ok(vec![TokenType::Literal(LiteralType::Integer(0xABCDEF0))]));
    assert_eq!(js_parser_rs::parse("0XABCDEF0".chars()), Ok(vec![TokenType::Literal(LiteralType::Integer(0xABCDEF0))]));
    assert_eq!(js_parser_rs::parse("0.0".chars()), Ok(vec![TokenType::Literal(LiteralType::Float(0.0))]));
    assert_eq!(js_parser_rs::parse("0.123".chars()), Ok(vec![TokenType::Literal(LiteralType::Float(0.123))]));
    assert_eq!(js_parser_rs::parse("123456789.123".chars()), Ok(vec![TokenType::Literal(LiteralType::Float(123456789.123))]));
}

#[test]
fn test_literal() {
    assert_eq!(js_parser_rs::parse("true".chars()), Ok(vec![TokenType::Literal(LiteralType::Boolean(true))]));
    assert_eq!(js_parser_rs::parse("false".chars()), Ok(vec![TokenType::Literal(LiteralType::Boolean(false))]));
    assert_eq!(js_parser_rs::parse("null".chars()), Ok(vec![TokenType::Literal(LiteralType::Null)]));
}

#[test]
fn test_punctuator() {
    assert_eq!(js_parser_rs::parse("{".chars()), Ok(vec![TokenType::Punctuator(Punctuator::LeftBrace)]));
    assert_eq!(js_parser_rs::parse("}".chars()), Ok(vec![TokenType::Punctuator(Punctuator::RightBrace)]));
    assert_eq!(js_parser_rs::parse("[".chars()), Ok(vec![TokenType::Punctuator(Punctuator::LeftBracket)]));
    assert_eq!(js_parser_rs::parse("]".chars()), Ok(vec![TokenType::Punctuator(Punctuator::RightBracket)]));
    assert_eq!(js_parser_rs::parse("(".chars()), Ok(vec![TokenType::Punctuator(Punctuator::LeftParen)]));
    assert_eq!(js_parser_rs::parse(")".chars()), Ok(vec![TokenType::Punctuator(Punctuator::RightParen)]));
    assert_eq!(js_parser_rs::parse("+".chars()), Ok(vec![TokenType::Punctuator(Punctuator::Plus)]));
    assert_eq!(js_parser_rs::parse("-".chars()), Ok(vec![TokenType::Punctuator(Punctuator::Minus)]));
    assert_eq!(js_parser_rs::parse("<".chars()), Ok(vec![TokenType::Punctuator(Punctuator::SmallThan)]));
    assert_eq!(js_parser_rs::parse(">".chars()), Ok(vec![TokenType::Punctuator(Punctuator::GreaterThan)]));
    assert_eq!(js_parser_rs::parse("!".chars()), Ok(vec![TokenType::Punctuator(Punctuator::Invert)]));
    assert_eq!(js_parser_rs::parse("=>".chars()), Ok(vec![TokenType::Punctuator(Punctuator::Lamda)]));
    assert_eq!(js_parser_rs::parse(".".chars()), Ok(vec![TokenType::Punctuator(Punctuator::Point)]));
    assert_eq!(js_parser_rs::parse("...".chars()), Ok(vec![TokenType::Punctuator(Punctuator::ThreePoints)]));
    assert_eq!(js_parser_rs::parse(":".chars()), Ok(vec![TokenType::Punctuator(Punctuator::DoublePoint)]));
    assert_eq!(js_parser_rs::parse("=".chars()), Ok(vec![TokenType::Punctuator(Punctuator::Equal)]));
    assert_eq!(js_parser_rs::parse("++".chars()), Ok(vec![TokenType::Punctuator(Punctuator::Increment)]));
    assert_eq!(js_parser_rs::parse("--".chars()), Ok(vec![TokenType::Punctuator(Punctuator::Decrement)]));
    assert_eq!(js_parser_rs::parse("<<".chars()), Ok(vec![TokenType::Punctuator(Punctuator::LeftShift)]));
    assert_eq!(js_parser_rs::parse(">>".chars()), Ok(vec![TokenType::Punctuator(Punctuator::RightShift)]));
    assert_eq!(js_parser_rs::parse("<<=".chars()), Ok(vec![TokenType::Punctuator(Punctuator::LeftShiftAssign)]));
    assert_eq!(js_parser_rs::parse(">>=".chars()), Ok(vec![TokenType::Punctuator(Punctuator::RightShiftAssign)]));
    assert_eq!(js_parser_rs::parse(">>>".chars()), Ok(vec![TokenType::Punctuator(Punctuator::RightShiftUnsigned)]));
    assert_eq!(js_parser_rs::parse(">>>=".chars()), Ok(vec![TokenType::Punctuator(Punctuator::RightShiftUnsignedAssign)]));
    assert_eq!(js_parser_rs::parse("==".chars()), Ok(vec![TokenType::Punctuator(Punctuator::IsEqual)]));
    assert_eq!(js_parser_rs::parse("!=".chars()), Ok(vec![TokenType::Punctuator(Punctuator::IsNotEqual)]));
    assert_eq!(js_parser_rs::parse("===".chars()), Ok(vec![TokenType::Punctuator(Punctuator::IsSame)]));
    assert_eq!(js_parser_rs::parse("!==".chars()), Ok(vec![TokenType::Punctuator(Punctuator::IsNotSame)]));
    assert_eq!(js_parser_rs::parse("<=".chars()), Ok(vec![TokenType::Punctuator(Punctuator::SmallAndEqualThan)]));
    assert_eq!(js_parser_rs::parse(">=".chars()), Ok(vec![TokenType::Punctuator(Punctuator::GreaterAndEqualThan)]));
    assert_eq!(js_parser_rs::parse("/".chars()), Ok(vec![TokenType::Punctuator(Punctuator::Divide)]));
    assert_eq!(js_parser_rs::parse("/=".chars()), Ok(vec![TokenType::Punctuator(Punctuator::DivideAssign)]));
    assert_eq!(js_parser_rs::parse("?".chars()), Ok(vec![TokenType::Punctuator(Punctuator::QuestionMark)]));
    assert_eq!(js_parser_rs::parse("~".chars()), Ok(vec![TokenType::Punctuator(Punctuator::Tilde)]));
    assert_eq!(js_parser_rs::parse("%".chars()), Ok(vec![TokenType::Punctuator(Punctuator::Mod)]));
    assert_eq!(js_parser_rs::parse("%=".chars()), Ok(vec![TokenType::Punctuator(Punctuator::ModAssign)]));
    assert_eq!(js_parser_rs::parse("^".chars()), Ok(vec![TokenType::Punctuator(Punctuator::Xor)]));
    assert_eq!(js_parser_rs::parse("^=".chars()), Ok(vec![TokenType::Punctuator(Punctuator::XorAssign)]));
    assert_eq!(js_parser_rs::parse("|".chars()), Ok(vec![TokenType::Punctuator(Punctuator::OrBitwise)]));
    assert_eq!(js_parser_rs::parse("|=".chars()), Ok(vec![TokenType::Punctuator(Punctuator::OrBitwiseAssign)]));
    assert_eq!(js_parser_rs::parse("||".chars()), Ok(vec![TokenType::Punctuator(Punctuator::Or)]));
    assert_eq!(js_parser_rs::parse("*".chars()), Ok(vec![TokenType::Punctuator(Punctuator::Multiple)]));
    assert_eq!(js_parser_rs::parse("*=".chars()), Ok(vec![TokenType::Punctuator(Punctuator::MultipleAssign)]));
    assert_eq!(js_parser_rs::parse("&".chars()), Ok(vec![TokenType::Punctuator(Punctuator::AndBitwise)]));
    assert_eq!(js_parser_rs::parse("&=".chars()), Ok(vec![TokenType::Punctuator(Punctuator::AndBitwiseAssign)]));
    assert_eq!(js_parser_rs::parse("&&".chars()), Ok(vec![TokenType::Punctuator(Punctuator::And)]));
    assert_eq!(js_parser_rs::parse("**".chars()), Ok(vec![TokenType::Punctuator(Punctuator::Exp)]));
    assert_eq!(js_parser_rs::parse("**=".chars()), Ok(vec![TokenType::Punctuator(Punctuator::ExpAssign)]));

    assert_eq!(js_parser_rs::parse("..".chars()), Ok(vec![TokenType::Punctuator(Punctuator::Point),TokenType::Punctuator(Punctuator::Point)]));
}

#[test]
fn test_raw() {
    assert_eq!(js_parser_rs::parse("Hello".chars()), Ok(vec![TokenType::SymbolLiteral(String::from("Hello"))]));
    assert_eq!(js_parser_rs::parse("Hello.Hello".chars()), Ok(vec![TokenType::SymbolLiteral(String::from("Hello")),TokenType::Punctuator(Punctuator::Point),TokenType::SymbolLiteral(String::from("Hello"))]));
}

#[test]
fn test_keyword() {
    //assert_eq!(js_parser_rs::parse("let".chars()), Ok(vec![TokenType::Keyword(Keyword::Let)]));

    assert_eq!(js_parser_rs::parse("var".chars()), Ok(vec![TokenType::Keyword(Keyword::Var)]));
    assert_eq!(js_parser_rs::parse("if".chars()), Ok(vec![TokenType::Keyword(Keyword::If)]));
    assert_eq!(js_parser_rs::parse("else".chars()), Ok(vec![TokenType::Keyword(Keyword::Else)]));
    assert_eq!(js_parser_rs::parse("do".chars()), Ok(vec![TokenType::Keyword(Keyword::Do)]));
    assert_eq!(js_parser_rs::parse("typeof".chars()), Ok(vec![TokenType::Keyword(Keyword::Typeof)]));
    assert_eq!(js_parser_rs::parse("switch".chars()), Ok(vec![TokenType::Keyword(Keyword::Switch)]));
    assert_eq!(js_parser_rs::parse("catch".chars()), Ok(vec![TokenType::Keyword(Keyword::Catch)]));
    assert_eq!(js_parser_rs::parse("try".chars()), Ok(vec![TokenType::Keyword(Keyword::Try)]));
    assert_eq!(js_parser_rs::parse("instanceof".chars()), Ok(vec![TokenType::Keyword(Keyword::Instanceof)]));
    assert_eq!(js_parser_rs::parse("export".chars()), Ok(vec![TokenType::Keyword(Keyword::Export)]));
    assert_eq!(js_parser_rs::parse("return".chars()), Ok(vec![TokenType::Keyword(Keyword::Return)]));
    assert_eq!(js_parser_rs::parse("void".chars()), Ok(vec![TokenType::Keyword(Keyword::Void)]));
    assert_eq!(js_parser_rs::parse("extends".chars()), Ok(vec![TokenType::Keyword(Keyword::Extends)]));
    assert_eq!(js_parser_rs::parse("const".chars()), Ok(vec![TokenType::Keyword(Keyword::Const)]));
    assert_eq!(js_parser_rs::parse("finally".chars()), Ok(vec![TokenType::Keyword(Keyword::Finally)]));
    assert_eq!(js_parser_rs::parse("super".chars()), Ok(vec![TokenType::Keyword(Keyword::Super)]));
    assert_eq!(js_parser_rs::parse("with".chars()), Ok(vec![TokenType::Keyword(Keyword::With)]));
    assert_eq!(js_parser_rs::parse("yield".chars()), Ok(vec![TokenType::Keyword(Keyword::Yield)]));
    assert_eq!(js_parser_rs::parse("default".chars()), Ok(vec![TokenType::Keyword(Keyword::Default)]));
    assert_eq!(js_parser_rs::parse("function".chars()), Ok(vec![TokenType::Keyword(Keyword::Function)]));
    assert_eq!(js_parser_rs::parse("of".chars()), Ok(vec![TokenType::Keyword(Keyword::Of)]));
    assert_eq!(js_parser_rs::parse("in".chars()), Ok(vec![TokenType::Keyword(Keyword::In)]));
    assert_eq!(js_parser_rs::parse("for".chars()), Ok(vec![TokenType::Keyword(Keyword::For)]));
    assert_eq!(js_parser_rs::parse("while".chars()), Ok(vec![TokenType::Keyword(Keyword::While)]));
    assert_eq!(js_parser_rs::parse("class".chars()), Ok(vec![TokenType::Keyword(Keyword::Class)]));
    assert_eq!(js_parser_rs::parse("break".chars()), Ok(vec![TokenType::Keyword(Keyword::Break)]));
    assert_eq!(js_parser_rs::parse("continue".chars()), Ok(vec![TokenType::Keyword(Keyword::Continue)]));
    assert_eq!(js_parser_rs::parse("new".chars()), Ok(vec![TokenType::Keyword(Keyword::New)]));
}

#[test]
fn test_terminate() {
    assert_eq!(js_parser_rs::parse("\n".chars()), Ok(vec![TokenType::LineTerminate]));
    assert_eq!(js_parser_rs::parse("\n ;".chars()), Ok(vec![TokenType::LineTerminate,TokenType::Semicolon]));
}

#[test]
fn test_regex() {
    assert_eq!(js_parser_rs::parse("= /ab+b/g;".chars()), Ok(vec![TokenType::Punctuator(Punctuator::Equal), TokenType::Regex(String::from("ab+b"), RegexIdentifier::Global),TokenType::Semicolon]));
    assert_eq!(js_parser_rs::parse("= /ab+b/;".chars()), Ok(vec![TokenType::Punctuator(Punctuator::Equal), TokenType::Regex(String::from("ab+b"), RegexIdentifier::None),TokenType::Semicolon]));
    assert_eq!(js_parser_rs::parse(": /ab+b/g;".chars()), Ok(vec![TokenType::Punctuator(Punctuator::DoublePoint), TokenType::Regex(String::from("ab+b"), RegexIdentifier::Global),TokenType::Semicolon]));
    assert_eq!(js_parser_rs::parse(", /ab+b/g;".chars()), Ok(vec![TokenType::Comma, TokenType::Regex(String::from("ab+b"), RegexIdentifier::Global),TokenType::Semicolon]))
}

#[test]
fn test_comment() {
    assert_eq!(js_parser_rs::parse("/*Hello World!".chars()), Ok(vec![TokenType::CommentLiteral(String::from("Hello World!"))]));
    assert_eq!(js_parser_rs::parse("// */ sdfsd".chars()), Ok(vec![TokenType::CommentLiteral(String::from(" */ sdfsd"))]));
    assert_eq!(js_parser_rs::parse("/*Hello */;".chars()), Ok(vec![TokenType::CommentLiteral(String::from("Hello ")),TokenType::Semicolon]));
    assert_eq!(js_parser_rs::parse("/*Hello **/;".chars()), Ok(vec![TokenType::CommentLiteral(String::from("Hello *")),TokenType::Semicolon]));
    assert_eq!(js_parser_rs::parse("/**Hello **/;".chars()), Ok(vec![TokenType::CommentLiteral(String::from("*Hello *")),TokenType::Semicolon]));
    assert_eq!(js_parser_rs::parse("/*Hello * */;".chars()), Ok(vec![TokenType::CommentLiteral(String::from("Hello * ")),TokenType::Semicolon]));
    assert_eq!(js_parser_rs::parse("//Hello \n;".chars()), Ok(vec![TokenType::CommentLiteral(String::from("Hello ")),TokenType::LineTerminate,TokenType::Semicolon]));
    assert_eq!(js_parser_rs::parse("/*Hello \n;*/".chars()), Ok(vec![TokenType::CommentLiteral(String::from("Hello \n;"))]));
}

#[test]
fn test_parse_typical_file() {
    let mut file = File::open("tests/js/jquery.js").unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    js_parser_rs::parse(OwningChars::new(s)).unwrap();
    //let a = js_parser_rs::parse(OwningChars::new(s)).unwrap();
    //assert_eq!(a, vec![])
}



struct OwningChars { s: String, pos: usize }

impl OwningChars {
    pub fn new(s: String) -> OwningChars {
        OwningChars { s: s, pos: 0 }
    }
}

impl Iterator for OwningChars {
    type Item = char;
    fn next(&mut self) -> Option<char> {
        if let Some(c) = self.s[self.pos..].chars().next() {
            self.pos += c.len_utf8();
            Some(c)
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.s.len() - self.pos;
        ((len + 3) / 4, Some(len)) // see the Chars impl for detail
    }
}

//#[test]
//fn some_code() {
//    assert_eq!(js_parser_rs::parse("var y = 2 +3; // defines the variable y and assigns to it the value 2\n while(y< 10) {\n y++;\n}\n console.log(y)".chars()), Ok(vec![]));
//}
