#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NumberType {
    None,
    NoneLiteral,
    Hex,
    Octal,
    Float
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CommentType {
    SingleLine,
    MultiLineStart,
    MultiLineEnd,
    MultiLineNormal
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StringType {
    SingleQuote,
    DoubleQuote
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Clone, Copy)]
pub enum Keyword {
    Var,
    If,
    Else,
    Do,
    Typeof,
    Switch,
    Catch,
    Try,
    Instanceof,
    Export,
    Return,
    Void,
    Extends,
    Const,
    Finally,
    Super,
    With,
    Yield,
    Default,
    Function,
    Of,
    In,
    For,
    While,
    Class,
    Break,
    Continue,
    New,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RegexState {
    Normal,
    Identifier
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LexerMode {
    None,
    String(StringType),
    Number(NumberType),
    Punctuator(Punctuator, i32),
    Comment(CommentType),
    Raw,
    Regex(RegexState),
    EOF
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Clone, Copy)]
pub enum Punctuator {
    Plus,
    // +
    Minus,
    // -
    Divide,
    // /
    DivideAssign,
    // /=
    SmallThan,
    // <
    GreaterThan,
    // >
    SmallAndEqualThan,
    // <=
    GreaterAndEqualThan,
    // >
    RightBrace,
    // }
    LeftBrace,
    // {
    RightBracket,
    // ]
    LeftBracket,
    // [
    RightParen,
    // )
    LeftParen,
    // (
    Point,
    // .
    DoublePoint,
    // :
    Equal,
    // =
    IsEqual,
    // ==
    IsNotEqual,
    // !=
    IsSame,
    // ===
    IsNotSame,
    // !==
    Increment,
    // ++
    Decrement,
    // --
    LeftShift,
    // <<
    RightShift,
    // >>
    Invert,
    // !
    Lamda,
    // =>
    RightShiftUnsigned,
    // >>>
    QuestionMark,
    // ?
    Tilde,
    //~
    Mod,
    //%
    ModAssign,
    //%=
    Xor,
    //^
    XorAssign,
    //^=
    OrBitwise,
    // |
    OrBitwiseAssign,
    // |=
    Or,
    // ||
    Multiple,
    // *
    MultipleAssign,
    // *=
    AndBitwise,
    // &
    AndBitwiseAssign,
    // &=
    And,
    // &&
    Exp,
    // **
    ExpAssign,
    // **=
    LeftShiftAssign,
    // <<=
    RightShiftAssign,
    // >>=
    ThreePoints,
    // ...
    RightShiftUnsignedAssign
    // >>>=
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum LiteralType {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum RegexIdentifier {
    Global,
    Ignore,
    None
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum TokenType {
    Punctuator(Punctuator),
    Literal(LiteralType),
    CommentLiteral(String),
    SymbolLiteral(String),
    Keyword(Keyword),
    Semicolon,
    Comma,
    Regex(String, RegexIdentifier),
    LineTerminate
}