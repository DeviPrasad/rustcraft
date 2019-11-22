#[path = "lexer/lex.rs"] mod lex;
//#[path = "lexer/token.rs"] mod token;

use lex::Lexer;
//use lex::Source;
//use token::Token;

fn main() {
    println!("\nRISC-V Assembler.\n");

    let src = lex::read_asm("../samples/strcpy.as");
    println!("{:?}", src);
    //let src = lex::read_asm("../samples/strcpy.asm");

    //let _ = Lexer::new(&src.text); // ^oo^: please the compliler.
    let _lexr = Lexer::from(&src);
    println!("{:?}", _lexr);
}
