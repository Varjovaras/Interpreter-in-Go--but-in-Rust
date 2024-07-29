//https://edu.anarcho-copy.org/Programming%20Languages/Go/writing%20an%20INTERPRETER%20in%20go.pdf
//44
use std::io::{self, BufReader, BufWriter};

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod repl;
pub mod token;

fn main() {
    println!("Hello! This is the Monkey programming language!");
    println!("Feel free to type in commands");

    let stdin = io::stdin();
    let stdout = io::stdout();

    let input = BufReader::new(stdin.lock());
    let output = BufWriter::new(stdout.lock());

    repl::start(input, output);
}
