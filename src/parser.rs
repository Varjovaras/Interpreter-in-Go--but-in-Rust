// parser.rs
use crate::ast::{Program, Token};
use crate::lexer::Lexer;

pub struct Parser {
    l: Lexer,
    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(mut l: Lexer) -> Self {
        let mut p = Parser {
            l,
            cur_token: Token { literal: String::new() },
            peek_token: Token { literal: String::new() },
        };
        // Read two tokens, so cur_token and peek_token are both set
        p.next_token();
        p.next_token();
        p
    }

    pub fn next_token(&mut self) {
        self.cur_token = std::mem::replace(&mut self.peek_token, self.l.next_token());
    }
    pub fn parse_program(&mut self) -> Program {
        Program { statements: Vec::new() }
    }
}