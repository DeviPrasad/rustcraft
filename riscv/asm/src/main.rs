#[path = "lexer/lex.rs"] mod lex;
#[path = "lexer/token.rs"] mod token;
#[path = "pass1/builder.rs"] mod builder;

use crate::lex::Lexer;
use crate::builder::Parser;

//use lex::Source;
//use token::Token;

fn main() {
    println!("\nRISC-V Assembler.\n");

    //let src = lex::read_asm("../samples/strcpy.as");
    let src = lex::read_asm("../samples/inst_mix.asm");
    //println!("{:?}", src);

    //let _ = Lexer::new(&src.text); // ^oo^: please the compliler.
/*
    let _lexr = Lexer::from(&src);
    for t in _lexr {
        println!("{:?}", t);
    }
*/

    let mut _lexr = Lexer::from(&src);
    let mut _parser = Parser::new(&mut _lexr);
    _parser.parse();

}
