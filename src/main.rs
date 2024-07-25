//https://edu.anarcho-copy.org/Programming%20Languages/Go/writing%20an%20INTERPRETER%20in%20go.pdf
//25

#![allow(dead_code)]
use std::io::{self, BufReader, BufWriter};

mod lexer;
mod repl;
mod token;
mod parser;
mod ast;

fn main() {
    println!("Hello! This is the Monkey programming language!");
    println!("Feel free to type in commands");

    let stdin = io::stdin();
    let stdout = io::stdout();

    let input = BufReader::new(stdin.lock());
    let output = BufWriter::new(stdout.lock());

    repl::start(input, output);
}
