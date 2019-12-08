use crate::lex::Lexer;
use crate::token::Token;

#[path = "ir.rs"] mod ir;
use ir::Reg;
use ir::Instruction;

pub struct Error {
    msg: String,
    tok: Token,
}

#[derive(Debug)]
pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    program: Vec<Instruction>,
    done: bool,
}

impl<'a> Parser<'a> {
    pub fn new(lex : &'a mut Lexer<'a>) -> Parser<'a> {
        Parser {
            lexer: lex,
            done: false,
            program: Vec::new(),
        }
    }

    pub fn parse(&mut self) {
        while !self.done {
            self.parse_one_inst();
        }
    }

    pub fn parse_one_inst(&mut self) {
        self.parse_3addr_inst();
    }

    pub fn parse_3addr_inst(&mut self) {
        let tok = self.lexer.next_token();
        if tok.is_none() {
            self.done = true;
            return;
        }

        let tok = tok.unwrap();
        if let Token::Label(_label) = tok {
            return;
        }

        if let Token::Iden(mnemonic) = tok {
            let dest = self.lexer.next_token();
            if dest.is_none() { self.done = true; return; }

            let dest: Token = dest.unwrap();

            if let Err((msg, ln)) = self.lexer.eat(Token::Comma) {
                self.inst_error(&msg, ln);
            }

            let tok = self.lexer.next_token();
            if tok.is_none() {
                self.unexpeted_end_error();
                return;
            }
            let src = tok.unwrap();

            if let Err((msg, ln)) = self.lexer.eat(Token::Comma) {
                self.inst_error(&msg, ln);
            }

            let tok = self.lexer.next_token();
            if tok.is_none() {
                self.unexpeted_end_error();
                return;
            }
            let src2 = tok.unwrap();

            println!("mnemmonic : {} {:?} {:?} {:?}", mnemonic, dest, src, src2);
        } else {
            self.error("Expected a mnemmonic, found ", tok);
            self.done = true;
        }
    }

    pub fn inst_error(&mut self, msg: &str, ln: u32) {
        println!("On line {} - {}", ln, msg);
    }

    pub fn unexpeted_end_error(&mut self) {
        println!("On line {} - Unexpected end of input", self.lexer.linenum());
    }

    pub fn error(&mut self, msg: &str, tok: Token) {
        println!("On line {} - {} {:?}", self.lexer.linenum(), msg, tok);
    }
}
