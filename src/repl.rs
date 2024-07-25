use std::io::{BufRead, Write};

use crate::{lexer::Lexer, token::TokenType};

const PROMPT: &str = "ali :D >> ";

pub fn start<R: BufRead, W: Write>(input: R, mut output: W) {
    let mut scanner = input.lines();

    loop {
        #[allow(clippy::uninlined_format_args)]
        write!(output, "{}", PROMPT).unwrap();
        output.flush().unwrap();

        if let Some(Ok(line)) = scanner.next() {
            let mut lexer = Lexer::new(&line);

            loop {
                let tok = lexer.next_token();
                if tok.kind == TokenType::Eof {
                    break;
                }
                #[allow(clippy::uninlined_format_args)]
                writeln!(output, "{:?}", tok).unwrap();
            }
        } else {
            break;
        }
    }
}
