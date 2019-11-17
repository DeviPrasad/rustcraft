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
    Return,
    True,
    False,

    Byte,
    HalfWord,
    Word,
    DoubleWord,

    Strz,
    Str,
    Equ,

    ByteAlign,
    P2Align,

    Eof,
    Illegal(char),
}
