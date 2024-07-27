// parser.rs
use crate::ast::{Identifier, LetStatement, Program, Statement};
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
            cur_token: Token {
                kind: TokenType::Eof,
                literal: String::new(),
            },
            peek_token: Token {
                kind: TokenType::Eof,
                literal: String::new(),
            },
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
        let mut program = Program {
            statements: Vec::new(),
        };

        while self.cur_token.kind != TokenType::Eof {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }
            self.next_token();
        }

        program
    }

    fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.cur_token.kind {
            TokenType::Let => self.parse_let_statement(),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn Statement>> {
        let mut stmt = LetStatement {
            token: self.cur_token.clone(),
            name: Box::new(Identifier {
                token: Token {
                    kind: TokenType::Illegal,
                    literal: String::new(),
                },
                value: String::new(),
            }),
            value: Box::new(Identifier {
                token: Token {
                    kind: TokenType::Illegal,
                    literal: String::new(),
                },
                value: String::new(),
            }),
        };

        if !self.expect_peek(&TokenType::Ident) {
            return None;
        }

        stmt.name = Box::new(Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        });

        if !self.expect_peek(&TokenType::Assign) {
            return None;
        }

        while !self.cur_token_is(&TokenType::Semicolon) {
            self.next_token();
        }

        Some(Box::new(stmt))
    }

    fn cur_token_is(&self, t: &TokenType) -> bool {
        self.cur_token.kind == t.clone()
    }

    fn peek_token_is(&self, t: &TokenType) -> bool {
        self.peek_token.kind == t.clone()
    }

    fn expect_peek(&mut self, t: &TokenType) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{LetStatement, Statement};
    use crate::lexer::Lexer;

    #[test]
    fn test_let_statements() {
        let input = "
        let x = 5;
        let y = 10;
        let foobar = 838383;
        ";

        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert_eq!(
            program.statements.len(),
            3,
            "program.statements does not contain 3 statements. got={}",
            program.statements.len()
        );

        let tests = vec!["x", "y", "foobar"];

        for (i, &expected_identifier) in tests.iter().enumerate() {
            let stmt = &program.statements[i];
            assert!(test_let_statement(stmt, expected_identifier));
        }
    }

    fn test_let_statement(s: &Box<dyn Statement>, name: &str) -> bool {
        if s.token_literal() != "let" {
            eprintln!("s.token_literal not 'let'. got={}", s.token_literal());
            return false;
        }

        let let_stmt = match s.as_any().downcast_ref::<LetStatement>() {
            Some(stmt) => stmt,
            None => {
                eprintln!("s not LetStatement. got={:?}", s);
                return false;
            }
        };

        if let_stmt.name.value != name {
            eprintln!(
                "let_stmt.name.value not '{}'. got={}",
                name, let_stmt.name.value
            );
            return false;
        }

        if let_stmt.name.token_literal() != name {
            eprintln!(
                "let_stmt.name.token_literal not '{}'. got={}",
                name,
                let_stmt.name.token_literal()
            );
            return false;
        }

        true
    }
}