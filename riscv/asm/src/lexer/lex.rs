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
        //Lexer { buf: src.text.chars().peekable(), lnum: 1 }
        Lexer::new(&src.text)
    }

    fn next(&mut self) -> Option<char> {
        self.buf.next()
    }

    fn peek(&mut self) -> Option<&char> {
        self.buf.peek()
    }
    fn eat_whitespaces(&mut self) {
        while let Some(&c) = self.buf.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.next();
        }
    }

    fn is_letter(ch: char) -> bool {
        ch.is_alphabetic()
    }

    fn is_alphanumeric(ch: char) -> bool {
        ch.is_alphanumeric() || ch == '_'
    }

    fn is_alphanumeric_next(&mut self) -> bool {
        match self.peek() {
            Some(&ch) => Lexer::is_alphanumeric(ch),
            None => false
        }
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

        while self.is_alphanumeric_next() {
            ident.push(self.next().unwrap());
        }

        ident
    }

    fn next_iden(&mut self) -> String {
        let c = self.next().unwrap();
        self.iden(c)
    }

    fn number(&mut self, first: char) -> String {
        let mut number = String::new();
        number.push(first);

        while let Some(&c) = self.peek() {
            if !c.is_numeric() {
                break;
            }
            number.push(self.next().unwrap());
        }

        number
    }

    fn hexnum(&mut self) -> Option<Token> {
        let mut hexstr = String::from("0X");
        while let Some(&c) = self.buf.peek() {
            if !c.is_digit(16) {
                break;
            }
            hexstr.push(self.next().unwrap());
        }

        if hexstr.len() > 2 {
            return Some(Token::Hex(hexstr));
        }

        return Some(Token::BadLexeme(hexstr));
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
                '.' => {
                    if self.is_letter_next() {
                        let lexme = self.next_iden();
                        let mut s = ".".to_owned();
                        s.push_str(&lexme);
                        if let Some(tok) = Self::lookup_directive(&s) {
                            return Some(tok);
                        } else {
                            return Some(Token::BadLexeme(lexme));
                        }
                    }
                    Some(Token::Bad('.'))
                }
                _ => {
                    if Self::is_letter(c) {
                        let lexme = self.iden(c);
                        if let Some(tok) = Self::lookup_keyword(&lexme) {
                            Some(tok)
                        } else if self.peek() == Some(&':') {
                            self.next();
                            Some(Token::Label(lexme))
                        } else {
                            Some(Token::Iden(lexme))
                        }
                    } else if c.is_digit(10) {
                        if c != '0' {
                            return Some(Token::Number(self.number(c)));
                        }
                        let ch = self.next().unwrap();
                        if !(ch == 'x' || ch == 'X') {
                            return Some(Token::Number(self.number(ch)));
                        } else {
                            return self.hexnum();
                        }
                    } else {
                        Some(Token::Bad(c))
                    }
                }
            }
        } else {
            None
        }
    }

    fn lookup_keyword(id: &str) -> Option<Token> {
        match id.to_lowercase().as_str() {
            "x0"     => Some(Token::Reg(0)),
            "zero"   => Some(Token::Reg(0)),
            "x1"     => Some(Token::Reg(1)),
            "ra"     => Some(Token::Reg(1)),
            "x2"     => Some(Token::Reg(2)),
            "sp"     => Some(Token::Reg(2)),
            "x3"     => Some(Token::Reg(3)),
            "gp"     => Some(Token::Reg(3)),
            "x4"     => Some(Token::Reg(4)),
            "tp"     => Some(Token::Reg(4)),
            "x5"     => Some(Token::Reg(5)),
            "t0"     => Some(Token::Reg(5)),
            "x6"     => Some(Token::Reg(6)),
            "t1"     => Some(Token::Reg(6)),
            "x7"     => Some(Token::Reg(7)),
            "t2"     => Some(Token::Reg(7)),
            "x8"     => Some(Token::Reg(8)),
            "s0"     => Some(Token::Reg(8)),
            "fp"     => Some(Token::Reg(8)),
            "x9"     => Some(Token::Reg(9)),
            "s1"     => Some(Token::Reg(9)),
            "x10"    => Some(Token::Reg(10)),
            "a0"     => Some(Token::Reg(10)),
            "x11"    => Some(Token::Reg(11)),
            "a1"     => Some(Token::Reg(11)),
            "x12"    => Some(Token::Reg(12)),
            "a2"     => Some(Token::Reg(12)),
            "x13"    => Some(Token::Reg(13)),
            "a3"     => Some(Token::Reg(13)),
            "x14"    => Some(Token::Reg(14)),
            "a4"     => Some(Token::Reg(14)),
            "x15"    => Some(Token::Reg(15)),
            "a5"     => Some(Token::Reg(15)),
            "x16"    => Some(Token::Reg(16)),
            "a6"     => Some(Token::Reg(16)),
            "x17"    => Some(Token::Reg(17)),
            "a7"     => Some(Token::Reg(17)),
            "x18"    => Some(Token::Reg(18)),
            "s2"     => Some(Token::Reg(18)),
            "x19"    => Some(Token::Reg(19)),
            "s3"     => Some(Token::Reg(19)),
            "x20"    => Some(Token::Reg(20)),
            "s4"     => Some(Token::Reg(20)),
            "x21"    => Some(Token::Reg(21)),
            "s5"     => Some(Token::Reg(21)),
            "x22"    => Some(Token::Reg(22)),
            "s6"     => Some(Token::Reg(22)),
            "x23"    => Some(Token::Reg(23)),
            "s7"     => Some(Token::Reg(23)),
            "x24"    => Some(Token::Reg(24)),
            "s8"     => Some(Token::Reg(24)),
            "x25"    => Some(Token::Reg(25)),
            "s9"     => Some(Token::Reg(25)),
            "x26"    => Some(Token::Reg(26)),
            "s10"    => Some(Token::Reg(26)),
            "x27"    => Some(Token::Reg(27)),
            "s11"    => Some(Token::Reg(27)),
            "x28"    => Some(Token::Reg(28)),
            "t3"     => Some(Token::Reg(28)),
            "x29"    => Some(Token::Reg(29)),
            "t4"     => Some(Token::Reg(29)),
            "x30"    => Some(Token::Reg(30)),
            "t5"     => Some(Token::Reg(30)),
            "x31"    => Some(Token::Reg(31)),
            "t6"     => Some(Token::Reg(31)),
            "true"   => Some(Token::True),
            "false"  => Some(Token::False),

            "proc"   => Some(Token::Proc),
            "macro"  => Some(Token::Macro),
            "begin"  => Some(Token::Begin),
            "end"    => Some(Token::End),
            "return" => Some(Token::Return),
            _        => Lexer::lookup_directive(id),
        }
    }

    fn lookup_directive(id: &str) -> Option<Token> {
        match id.to_lowercase().as_str() {
            ".byte"   => Some(Token::Byte),
            ".2byte"  => Some(Token::HalfWord),
            ".half"   => Some(Token::HalfWord),
            ".short"  => Some(Token::HalfWord),
            ".4byte"  => Some(Token::Word),
            ".word"   => Some(Token::Word),
            ".long"   => Some(Token::Word),
            ".8byte"  => Some(Token::DoubleWord),
            ".dword"  => Some(Token::DoubleWord),
            ".quad"   => Some(Token::DoubleWord),
            ".string" => Some(Token::Str),
            ".asciz"  => Some(Token::Strz),
            ".equ"    => Some(Token::Equ),

            ".proc"   => Some(Token::Proc),
            ".macro"  => Some(Token::Macro),
            ".begin"  => Some(Token::Begin),
            ".end"    => Some(Token::End),

            ".align"   => Some(Token::ByteAlign),
            ".p2align" => Some(Token::P2Align),
            ".section" => Some(Token::Section),
            ".data"    => Some(Token::DataSection),
            ".text"    => Some(Token::TextSection),
            ".rodata"  => Some(Token::ReadOnlyData),
            ".bss"     => Some(Token::UninitializedData),
            _          => None
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        self.next_token()
    }
}
