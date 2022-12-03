pub mod node;
pub mod lexer;
pub mod parser;

use std::env;
use std::fs;
use crate::lexer::Lexer;
use crate::parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("In file {}", file_path);

    let code = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut lexer = Lexer::new(code);
    let token_list = lexer.tokenize();
    let mut parser = Parser::new(&token_list);
    let node_list = parser.prog();
}