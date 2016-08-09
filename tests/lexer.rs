extern crate js_parser_rs;

use js_parser_rs::lexer::enums::{TokenType, LiteralType, RegexIdentifier};
use js_parser_rs::error::error::{ErrorType, SyntaxErrorType};
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
fn test_white_spaces() {
    assert_eq!(js_parser_rs::parse(" ".chars()), Ok(vec![]));
    assert_eq!(js_parser_rs::parse("\u{a0}".chars()), Ok(vec![]));
    //assert_eq!(js_parser_rs::parse("\u{9}".chars()), Ok(vec![]));
    assert_eq!(js_parser_rs::parse("\u{b}".chars()), Ok(vec![]));
    assert_eq!(js_parser_rs::parse("\u{c}".chars()), Ok(vec![]));
    assert_eq!(js_parser_rs::parse("\t".chars()), Ok(vec![]));
    assert_eq!(js_parser_rs::parse("; ".chars()), Ok(vec![TokenType::Semicolon]));
    assert_eq!(js_parser_rs::parse(",".chars()), Ok(vec![TokenType::Comma]));
}

#[test]
fn test_useless_string() {
    assert_eq!(js_parser_rs::parse("+ \"-\\f]' ms=''>\" +\n".chars()), Ok(vec![TokenType::Plus,TokenType::Literal(LiteralType::String(String::from("-\\f]' ms=''>"))),TokenType::Plus,TokenType::LineTerminate]));
    assert_eq!(js_parser_rs::parse("\"Hello World!\"".chars()), Ok(vec![TokenType::Literal(LiteralType::String(String::from("Hello World!")))]));
    assert_eq!(js_parser_rs::parse("\"Hello".chars()), Err(ErrorType::SyntaxError(SyntaxErrorType::UnexpectedEOF)));
    assert_eq!(js_parser_rs::parse("\"Hel{}\" \"Hello World!\"".chars()),Ok(vec![TokenType::Literal(LiteralType::String(String::from("Hel{}"))),TokenType::Literal(LiteralType::String(String::from("Hello World!")))]));
    assert_eq!(js_parser_rs::parse("\"Hello\\\" World!\"".chars()), Ok(vec![TokenType::Literal(LiteralType::String(String::from("Hello\" World!")))]));
    assert_eq!(js_parser_rs::parse("'Hello World!'".chars()), Ok(vec![TokenType::Literal(LiteralType::String(String::from("Hello World!")))]));
    assert_eq!(js_parser_rs::parse("'Hello\\' World!'".chars()), Ok(vec![TokenType::Literal(LiteralType::String(String::from("Hello' World!")))]));
    assert_eq!(js_parser_rs::parse("\"Hello\\\\ World!\"".chars()), Ok(vec![TokenType::Literal(LiteralType::String(String::from("Hello\\ World!")))]));
}

#[test]
fn test_useless_number() {
    assert_eq!(js_parser_rs::parse("0".chars()), Ok(vec![TokenType::Literal(LiteralType::Integer(0))]));
    assert_eq!(js_parser_rs::parse("0xabcdef0".chars()), Ok(vec![TokenType::Literal(LiteralType::Integer(0xabcdef0))]));
    assert_eq!(js_parser_rs::parse("0xABCDEF0".chars()), Ok(vec![TokenType::Literal(LiteralType::Integer(0xABCDEF0))]));
    assert_eq!(js_parser_rs::parse("0XABCDEF0".chars()), Ok(vec![TokenType::Literal(LiteralType::Integer(0xABCDEF0))]));
    assert_eq!(js_parser_rs::parse("0o34".chars()), Ok(vec![TokenType::Literal(LiteralType::Integer(0o34))]));
    assert_eq!(js_parser_rs::parse("0O34".chars()), Ok(vec![TokenType::Literal(LiteralType::Integer(0o34))]));
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
    assert_eq!(js_parser_rs::parse("{".chars()), Ok(vec![TokenType::LeftBrace]));
    assert_eq!(js_parser_rs::parse("}".chars()), Ok(vec![TokenType::RightBrace]));
    assert_eq!(js_parser_rs::parse("[".chars()), Ok(vec![TokenType::LeftBracket]));
    assert_eq!(js_parser_rs::parse("]".chars()), Ok(vec![TokenType::RightBracket]));
    assert_eq!(js_parser_rs::parse("(".chars()), Ok(vec![TokenType::LeftParen]));
    assert_eq!(js_parser_rs::parse(")".chars()), Ok(vec![TokenType::RightParen]));
    assert_eq!(js_parser_rs::parse("+".chars()), Ok(vec![TokenType::Plus]));
    assert_eq!(js_parser_rs::parse("+=".chars()), Ok(vec![TokenType::PlusAssign]));
    assert_eq!(js_parser_rs::parse("-".chars()), Ok(vec![TokenType::Minus]));
    assert_eq!(js_parser_rs::parse("-=".chars()), Ok(vec![TokenType::MinusAssign]));
    assert_eq!(js_parser_rs::parse("<".chars()), Ok(vec![TokenType::SmallThan]));
    assert_eq!(js_parser_rs::parse(">".chars()), Ok(vec![TokenType::GreaterThan]));
    assert_eq!(js_parser_rs::parse("!".chars()), Ok(vec![TokenType::Invert]));
    assert_eq!(js_parser_rs::parse("=>".chars()), Ok(vec![TokenType::Lamda]));
    assert_eq!(js_parser_rs::parse(".".chars()), Ok(vec![TokenType::Point]));
    assert_eq!(js_parser_rs::parse("...".chars()), Ok(vec![TokenType::ThreePoints]));
    assert_eq!(js_parser_rs::parse(":".chars()), Ok(vec![TokenType::Colon]));
    assert_eq!(js_parser_rs::parse("=".chars()), Ok(vec![TokenType::Equal]));
    assert_eq!(js_parser_rs::parse("++".chars()), Ok(vec![TokenType::Increment]));
    assert_eq!(js_parser_rs::parse("--".chars()), Ok(vec![TokenType::Decrement]));
    assert_eq!(js_parser_rs::parse("<<".chars()), Ok(vec![TokenType::LeftShift]));
    assert_eq!(js_parser_rs::parse(">>".chars()), Ok(vec![TokenType::RightShift]));
    assert_eq!(js_parser_rs::parse("<<=".chars()), Ok(vec![TokenType::LeftShiftAssign]));
    assert_eq!(js_parser_rs::parse(">>=".chars()), Ok(vec![TokenType::RightShiftAssign]));
    assert_eq!(js_parser_rs::parse(">>>".chars()), Ok(vec![TokenType::RightShiftUnsigned]));
    assert_eq!(js_parser_rs::parse(">>>=".chars()), Ok(vec![TokenType::RightShiftUnsignedAssign]));
    assert_eq!(js_parser_rs::parse("==".chars()), Ok(vec![TokenType::IsEqual]));
    assert_eq!(js_parser_rs::parse("!=".chars()), Ok(vec![TokenType::IsNotEqual]));
    assert_eq!(js_parser_rs::parse("===".chars()), Ok(vec![TokenType::IsSame]));
    assert_eq!(js_parser_rs::parse("!==".chars()), Ok(vec![TokenType::IsNotSame]));
    assert_eq!(js_parser_rs::parse("<=".chars()), Ok(vec![TokenType::SmallAndEqualThan]));
    assert_eq!(js_parser_rs::parse(">=".chars()), Ok(vec![TokenType::GreaterAndEqualThan]));
    assert_eq!(js_parser_rs::parse("/".chars()), Ok(vec![TokenType::Divide]));
    assert_eq!(js_parser_rs::parse("/=".chars()), Ok(vec![TokenType::DivideAssign]));
    assert_eq!(js_parser_rs::parse("?".chars()), Ok(vec![TokenType::QuestionMark]));
    assert_eq!(js_parser_rs::parse("~".chars()), Ok(vec![TokenType::Tilde]));
    assert_eq!(js_parser_rs::parse("%".chars()), Ok(vec![TokenType::Mod]));
    assert_eq!(js_parser_rs::parse("%=".chars()), Ok(vec![TokenType::ModAssign]));
    assert_eq!(js_parser_rs::parse("^".chars()), Ok(vec![TokenType::Xor]));
    assert_eq!(js_parser_rs::parse("^=".chars()), Ok(vec![TokenType::XorAssign]));
    assert_eq!(js_parser_rs::parse("|".chars()), Ok(vec![TokenType::OrBitwise]));
    assert_eq!(js_parser_rs::parse("|=".chars()), Ok(vec![TokenType::OrBitwiseAssign]));
    assert_eq!(js_parser_rs::parse("||".chars()), Ok(vec![TokenType::Or]));
    assert_eq!(js_parser_rs::parse("*".chars()), Ok(vec![TokenType::Multiple]));
    assert_eq!(js_parser_rs::parse("*=".chars()), Ok(vec![TokenType::MultipleAssign]));
    assert_eq!(js_parser_rs::parse("&".chars()), Ok(vec![TokenType::AndBitwise]));
    assert_eq!(js_parser_rs::parse("&=".chars()), Ok(vec![TokenType::AndBitwiseAssign]));
    assert_eq!(js_parser_rs::parse("&&".chars()), Ok(vec![TokenType::And]));
    assert_eq!(js_parser_rs::parse("**".chars()), Ok(vec![TokenType::Exp]));
    assert_eq!(js_parser_rs::parse("**=".chars()), Ok(vec![TokenType::ExpAssign]));
    assert_eq!(js_parser_rs::parse("+-f".chars()), Ok(vec![TokenType::Plus,TokenType::Minus,TokenType::Identifier(String::from("f"))]));
    assert_eq!(js_parser_rs::parse("..".chars()), Ok(vec![TokenType::Point,TokenType::Point]));
    assert_eq!(js_parser_rs::parse("...".chars()), Ok(vec![TokenType::ThreePoints]));
}

#[test]
fn test_raw() {
    assert_eq!(js_parser_rs::parse("Hello".chars()), Ok(vec![TokenType::Identifier(String::from("Hello"))]));
    assert_eq!(js_parser_rs::parse("Hello\n".chars()), Ok(vec![TokenType::Identifier(String::from("Hello")),TokenType::LineTerminate]));
    assert_eq!(js_parser_rs::parse("Hello\r".chars()), Ok(vec![TokenType::Identifier(String::from("Hello")),TokenType::LineTerminate]));
    assert_eq!(js_parser_rs::parse("Hello\u{a0}".chars()), Ok(vec![TokenType::Identifier(String::from("Hello"))]));
    //assert_eq!(js_parser_rs::parse("Hello\u{9}".chars()), Ok(vec![TokenType::Identifier(String::from("Hello"))]));
    assert_eq!(js_parser_rs::parse("Hello\u{b}".chars()), Ok(vec![TokenType::Identifier(String::from("Hello"))]));
    assert_eq!(js_parser_rs::parse("Hello\u{c}".chars()), Ok(vec![TokenType::Identifier(String::from("Hello"))]));
    assert_eq!(js_parser_rs::parse("Hello\t".chars()), Ok(vec![TokenType::Identifier(String::from("Hello"))]));
    assert_eq!(js_parser_rs::parse("Hello ".chars()), Ok(vec![TokenType::Identifier(String::from("Hello"))]));
    assert_eq!(js_parser_rs::parse("Hello=".chars()), Ok(vec![TokenType::Identifier(String::from("Hello")),TokenType::Equal]));
    assert_eq!(js_parser_rs::parse("Hello('sd'".chars()), Ok(vec![TokenType::Identifier(String::from("Hello")),TokenType::LeftParen,TokenType::Literal(LiteralType::String(String::from("sd")))]));
    assert_eq!(js_parser_rs::parse("Hello|Hello".chars()), Ok(vec![TokenType::Identifier(String::from("Hello")),TokenType::OrBitwise,TokenType::Identifier(String::from("Hello"))]));
    assert_eq!(js_parser_rs::parse("Hello.Hello".chars()), Ok(vec![TokenType::Identifier(String::from("Hello")),TokenType::Point,TokenType::Identifier(String::from("Hello"))]));
    assert_eq!(js_parser_rs::parse("Hello/Hello".chars()), Ok(vec![TokenType::Identifier(String::from("Hello")),TokenType::Divide,TokenType::Identifier(String::from("Hello"))]));

    assert_eq!(js_parser_rs::parse("\\u005f\\u005f\\u0076\\u0061\\u0072".chars()), Ok(vec![TokenType::Identifier(String::from("__var"))]));
}

#[test]
fn test_keyword() {
    //assert_eq!(js_parser_rs::parse("let".chars()), Ok(vec![TokenType::Let)]));

    assert_eq!(js_parser_rs::parse("var".chars()), Ok(vec![TokenType::Var]));
    assert_eq!(js_parser_rs::parse("if".chars()), Ok(vec![TokenType::If]));
    assert_eq!(js_parser_rs::parse("else".chars()), Ok(vec![TokenType::Else]));
    assert_eq!(js_parser_rs::parse("do".chars()), Ok(vec![TokenType::Do]));
    assert_eq!(js_parser_rs::parse("typeof".chars()), Ok(vec![TokenType::Typeof]));
    assert_eq!(js_parser_rs::parse("switch".chars()), Ok(vec![TokenType::Switch]));
    assert_eq!(js_parser_rs::parse("catch".chars()), Ok(vec![TokenType::Catch]));
    assert_eq!(js_parser_rs::parse("try".chars()), Ok(vec![TokenType::Try]));
    assert_eq!(js_parser_rs::parse("instanceof".chars()), Ok(vec![TokenType::Instanceof]));
    assert_eq!(js_parser_rs::parse("export".chars()), Ok(vec![TokenType::Export]));
    assert_eq!(js_parser_rs::parse("return".chars()), Ok(vec![TokenType::Return]));
    assert_eq!(js_parser_rs::parse("void".chars()), Ok(vec![TokenType::Void]));
    assert_eq!(js_parser_rs::parse("extends".chars()), Ok(vec![TokenType::Extends]));
    assert_eq!(js_parser_rs::parse("const".chars()), Ok(vec![TokenType::Const]));
    assert_eq!(js_parser_rs::parse("finally".chars()), Ok(vec![TokenType::Finally]));
    assert_eq!(js_parser_rs::parse("super".chars()), Ok(vec![TokenType::Super]));
    assert_eq!(js_parser_rs::parse("with".chars()), Ok(vec![TokenType::With]));
    assert_eq!(js_parser_rs::parse("yield".chars()), Ok(vec![TokenType::Yield]));
    assert_eq!(js_parser_rs::parse("default".chars()), Ok(vec![TokenType::Default]));
    assert_eq!(js_parser_rs::parse("function".chars()), Ok(vec![TokenType::Function]));
    assert_eq!(js_parser_rs::parse("of".chars()), Ok(vec![TokenType::Of]));
    assert_eq!(js_parser_rs::parse("in".chars()), Ok(vec![TokenType::In]));
    assert_eq!(js_parser_rs::parse("for".chars()), Ok(vec![TokenType::For]));
    assert_eq!(js_parser_rs::parse("while".chars()), Ok(vec![TokenType::While]));
    assert_eq!(js_parser_rs::parse("class".chars()), Ok(vec![TokenType::Class]));
    assert_eq!(js_parser_rs::parse("break".chars()), Ok(vec![TokenType::Break]));
    assert_eq!(js_parser_rs::parse("continue".chars()), Ok(vec![TokenType::Continue]));
    assert_eq!(js_parser_rs::parse("new".chars()), Ok(vec![TokenType::New]));
}

#[test]
fn test_terminate() {
    assert_eq!(js_parser_rs::parse("\n".chars()), Ok(vec![]));
    assert_eq!(js_parser_rs::parse("\r".chars()), Ok(vec![]));
    assert_eq!(js_parser_rs::parse("\n ;".chars()), Ok(vec![TokenType::Semicolon]));
}

#[test]
fn test_regex() {
    assert_eq!(js_parser_rs::parse("= /ab+b/g;".chars()), Ok(vec![TokenType::Equal, TokenType::Literal(LiteralType::Regex(String::from("ab+b"), RegexIdentifier::Global)),TokenType::Semicolon]));
    assert_eq!(js_parser_rs::parse("( /ab+b/g)".chars()), Ok(vec![TokenType::LeftParen, TokenType::Literal(LiteralType::Regex(String::from("ab+b"), RegexIdentifier::Global)),TokenType::RightParen]));
    assert_eq!(js_parser_rs::parse("= /ab+b/;".chars()), Ok(vec![TokenType::Equal, TokenType::Literal(LiteralType::Regex(String::from("ab+b"), RegexIdentifier::None)),TokenType::Semicolon]));
    assert_eq!(js_parser_rs::parse(": /ab+b/g;".chars()), Ok(vec![TokenType::Colon, TokenType::Literal(LiteralType::Regex(String::from("ab+b"), RegexIdentifier::Global)),TokenType::Semicolon]));
    assert_eq!(js_parser_rs::parse(", /ab+b/g;".chars()), Ok(vec![TokenType::Comma, TokenType::Literal(LiteralType::Regex(String::from("ab+b"), RegexIdentifier::Global)),TokenType::Semicolon]));
    assert_eq!(js_parser_rs::parse(", /*a*/ /ab+b/g;".chars()), Ok(vec![TokenType::Comma, TokenType::Literal(LiteralType::Regex(String::from("ab+b"), RegexIdentifier::Global)),TokenType::Semicolon]));
    assert_eq!(js_parser_rs::parse(", /ab\\/b/g".chars()), Ok(vec![TokenType::Comma, TokenType::Literal(LiteralType::Regex(String::from("ab/b"), RegexIdentifier::Global))]));
    assert_eq!(js_parser_rs::parse(", /ab\\\\b/g".chars()), Ok(vec![TokenType::Comma, TokenType::Literal(LiteralType::Regex(String::from("ab\\b"), RegexIdentifier::Global))]));
    assert_eq!(js_parser_rs::parse(", /^h\\d$/i".chars()), Ok(vec![TokenType::Comma, TokenType::Literal(LiteralType::Regex(String::from("^h\\d$"), RegexIdentifier::Ignore))]));
}

#[test]
fn test_comment() {
    assert_eq!(js_parser_rs::parse("/*Hello World!".chars()), Ok(vec![]));
    assert_eq!(js_parser_rs::parse("// */ sdfsd".chars()), Ok(vec![]));
    assert_eq!(js_parser_rs::parse("/*Hello */;".chars()), Ok(vec![TokenType::Semicolon]));
    assert_eq!(js_parser_rs::parse("/*Hello **/;".chars()), Ok(vec![TokenType::Semicolon]));
    assert_eq!(js_parser_rs::parse("/**Hello **/;".chars()), Ok(vec![TokenType::Semicolon]));
    assert_eq!(js_parser_rs::parse("/*Hello * */;".chars()), Ok(vec![TokenType::Semicolon]));
    assert_eq!(js_parser_rs::parse("//Hello \n;".chars()), Ok(vec![TokenType::Semicolon]));
    assert_eq!(js_parser_rs::parse("/*Hello \n;*/".chars()), Ok(vec![]));
    //assert_eq!(js_parser_rs::parse("// IE \\r a".chars()), Ok(vec![TokenType::CommentLiteral(String::from(" IE \r a"))]));
}


#[test]
#[should_panic]
fn sould_panic_number() {
    println!("{:?}", js_parser_rs::parse("0o394".chars()));
}

#[test]
fn test_parse_test_file() {
    let mut file = File::open("tests/js/test.js").unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    js_parser_rs::parse(OwningChars::new(s)).unwrap();
    //let a = js_parser_rs::parse(OwningChars::new(s)).unwrap();
    //assert_eq!(a, vec![])
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


struct OwningChars {
    s: String,
    pos: usize
}

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
