#[derive(Debug,PartialEq)]
pub enum Token {
    Iden(String),
    Number(String),
    Label(String),

    Zero,
    Assign,
    Negate,
    Minus,
    Plus,
    Div,
    Mult,
    Mod,

    Comma,
    Colon,
    Semicolon,
    Sharp,

    LeftSqBrace,
    RightSqBrace,
    LeftParen,
    RightParen,

    Proc,
    Macro,
    Return,
    True,
    False,
    Begin,
    End,

    Byte,
    HalfWord,
    Word,
    DoubleWord,

    Strz,
    Str,
    Equ,

    ByteAlign,
    P2Align,

    Section,
    TextSection,
    DataSection,
    UninitializedData,
    ReadOnlyData,

    Eof,
    Illegal(char),
}
