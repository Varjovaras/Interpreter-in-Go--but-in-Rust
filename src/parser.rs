// parser.rs
use crate::ast::{Program};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

pub struct Parser {
    l: Lexer,
    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    #[allow(unused_mut)]
    pub fn new(mut l: Lexer) -> Self {
        let mut p = Self {
            l,
            cur_token: Token { kind: TokenType::Eof, literal: String::new() },
            peek_token: Token { kind: TokenType::Eof, literal: String::new() },
        };
        // Read two tokens, so cur_token and peek_token are both set
        p.next_token();
        p.next_token();
        p
    }

    pub fn next_token(&mut self) {
        self.cur_token = std::mem::replace(&mut self.peek_token, self.l.next_token());
    }

    #[allow(clippy::needless_pass_by_ref_mut, clippy::unused_self)]
    pub fn parse_program(&mut self) -> Program {
        Program { statements: Vec::new() }
    }
}