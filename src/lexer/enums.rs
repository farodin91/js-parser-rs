#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NumberType {
    None,
    NoneLiteral,
    Hex,
    Float
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operation {
    Start,
    End,
    Normal
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
pub enum LexerMode {
    None,
    String,
    Number(NumberType),
    Punctuator(Punctuator),
    Comment(Operation),
    Raw,
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
    DivideEq,
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
    RightSquaredBrace,
    // ]
    LeftSquaredBrace,
    // [
    RightRoundedBrace,
    // )
    LeftRoundedBrace,
    // (
    Point,
    // .
    DoublePoint,
    // :
    Semicolon,
    // ;
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
    If,
    // ?
    Tilde,
    //~
    Mod,
    //%
    ModEq,
    //%=
    Xor,
    //^
    XorEq,
    //^=
    OrBitwise,
    // |
    OrBitwiseEq,
    // |=
    Or,
    // ||
    Multiple,
    // *
    MultipleEq,
    // *=
    AndBitwise,
    // &
    AndBitwiseEq,
    // &=
    And,
    // &&
    Exp,
    // **
    ExpEq,
    // **=
    LeftShiftEq,
    // <<=
    RightShiftEq,
    // >>=
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
pub enum TokenType {
    Punctuator(Punctuator),
    Literal(LiteralType),
    CommentLiteral(String),
    SymbolLiteral(String),
    Keyword(Keyword)
}