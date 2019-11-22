use std::fs;
use std::iter::Peekable;
use std::str::Chars;

#[path = "token.rs"] mod token;
use token::Token;

#[derive(Debug)]
pub struct Source {
    fname: String,
    text: String,
    error: String,
}

pub fn read_asm(asmfile: &str) -> Source {
    let asmsrc = fs::read_to_string(asmfile);
    match asmsrc {
        Ok(text) => {
            return Source { fname: String::from(asmfile), text: text, error: String::new() };
        }
        Err(e) => {
            return Source { fname: asmfile.to_string(), text: String::new(), error: e.to_string() };
        }
    }
}

#[derive(Debug)]
pub struct Lexer<'a> {
    buf: Peekable<Chars<'a>>,
    lnum: u32,
}

impl<'a> Lexer<'a> {
    pub fn new(text: &str) -> Lexer {
        Lexer { buf: text.chars().peekable(), lnum: 1 }
    }

    pub fn from(src: &Source) -> Lexer {
        Lexer { buf: src.text.chars().peekable(), lnum: 1 }
    }

    fn next(&mut self) -> Option<char> {
        self.buf.next()
    }

    fn peek(&mut self) -> Option<&char> {
        self.buf.peek()
    }

    fn eat_whitespaces(&mut self) {
        while let Some(&c) = self.peek() {

            if !c.is_whitespace() {
                break;
            }
            self.next();
        }
    }

    fn is_letter(ch: char) -> bool {
        ch.is_alphabetic() || ch == '_'
    }

    fn is_letter_next(&mut self) -> bool {
        match self.peek() {
            Some(&ch) => Lexer::is_letter(ch),
            None => false,
        }
    }

    fn iden(&mut self, first: char) -> String {
        let mut ident = String::new();
        ident.push(first);

        self.eat_whitespaces();

        while self.is_letter_next() {
            ident.push(self.next().unwrap());
        }

        ident
    }

    fn number(&mut self, first: char) -> String {
        let mut number = String::new();
        number.push(first);

        self.eat_whitespaces();

        while let Some(&c) = self.peek() {
            if !c.is_numeric() {
                break;
            }
            number.push(self.next().unwrap());
        }

        number
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.eat_whitespaces();
        if let Some(c) = self.next() {
            match c {
                '=' => Some(Token::Assign),
                '!' => Some(Token::Negate),
                '+' => Some(Token::Plus),
                '-' => Some(Token::Minus),
                '/' => Some(Token::Div),
                '*' => Some(Token::Mult),
                '%' => Some(Token::Mod),
                ',' => Some(Token::Comma),
                ':' => Some(Token::Colon),
                ';' => Some(Token::Semicolon),
                '#' => Some(Token::Sharp),
                '(' => Some(Token::LeftParen),
                ')' => Some(Token::RightParen),
                '[' => Some(Token::LeftSqBrace),
                ']' => Some(Token::RightSqBrace),
                _ => {
                    if Self::is_letter(c) {
                        Some(Self::lookup_keyword(self.iden(c)))
                    } else if c.is_digit(10) {
                        Some(Token::Number(self.number(c)))
                    } else {
                        Some(Token::Illegal(c))
                    }
                }
            }
        } else {
            None
        }
    }

    fn lookup_keyword(id: String) -> Token {
        match id.as_str() {
            ".byte"   => Token::Byte,
            ".2byte"  => Token::HalfWord,
            ".half"   => Token::HalfWord,
            ".short"  => Token::HalfWord,
            ".4byte"  => Token::Word,
            ".word"   => Token::Word,
            ".long"   => Token::Word,
            ".8byte"  => Token::DoubleWord,
            ".dword"  => Token::DoubleWord,
            ".quad"   => Token::DoubleWord,
            ".string" => Token::Str,
            ".asciz"  => Token::Strz,
            ".equ"    => Token::Equ,

            ".proc"   => Token::Proc,
            ".macro"  => Token::Macro,
            ".begin"  => Token::Begin,
            ".end"    => Token::End,
            "return"  => Token::Return,
            "true"    => Token::True,
            "false"   => Token::False,
            "zero"    => Token::Zero,

            ".align"   => Token::ByteAlign,
            ".p2align" => Token::P2Align,
            ".section" => Token::Section,
            ".data"    => Token::DataSection,
            ".text"    => Token::TextSection,
            ".rodata"  => Token::ReadOnlyData,
            ".bss"     => Token::UninitializedData,


            _ =>  Token::Iden(id),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        self.next_token()
    }
}

