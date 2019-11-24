#[derive(Debug,PartialEq)]
pub enum Token {
    Iden(String),
    Number(String),
    Hex(String),
    Label(String),
    Reg(u8),
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

    Bad(char),
    BadLexeme(String)
}
