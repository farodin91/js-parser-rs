#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum NumberType {
    None,
    NoneLiteral,
    Hex,
    Octal,
    Float
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum CommentType {
    SingleLine,
    MultiLineStart,
    MultiLineEnd,
    MultiLineNormal
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum StringType {
    SingleQuote,
    DoubleQuote
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum RegexState {
    Normal,
    Identifier
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum LexerMode {
    None,
    String(StringType),
    Number(NumberType),
    Punctuator(TokenType, i32),
    Comment(CommentType),
    Raw,
    Regex(RegexState),
    EOF
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum LiteralType {
    String(String),
    Regex(String, RegexIdentifier),
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
    Literal(LiteralType),
    CommentLiteral(String),
    Identifier(String),
    Plus,
    // +
    Minus,
    // -
    PlusAssign,
    // +=
    MinusAssign,
    // -=
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
    Colon,
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
    RightShiftUnsignedAssign,
    // >>>=
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
    Delete,
    Yield,
    Default,
    Function,
    Of,
    In,
    For,
    While,
    Class,
    Case,
    Break,
    Continue,
    New,
    Let,
    Throw,
    Debugger,
    This,
    Target,
    Semicolon,
    Comma,
    Get,
    Set,
    LineTerminate
}