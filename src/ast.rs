// Define the Token struct

use crate::token::Token;
use std::any::Any;
use std::fmt::Debug;

// Define the Node trait
pub trait Node: Any {
    fn token_literal(&self) -> String;
}

// Define the Statement trait extending Node
pub trait Statement: Node + Debug {
    fn statement_node(&self);
    fn as_any(&self) -> &dyn Any;
}

// Define the Expression trait extending Node

pub trait Expression: Node + Debug {
    fn expression_node(&self);
}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.is_empty() {
            return String::new();
        }
        self.statements[0].token_literal()
    }
}

// Define the LetStatement struct
#[derive(Debug)]
pub struct LetStatement {
    pub token: Token, // the token.LET token
    pub name: Identifier,
    pub value: Box<dyn Expression>,
}

// Implement Node for LetStatement
impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

// Implement Statement for LetStatement
impl Statement for LetStatement {
    fn statement_node(&self) {}
    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Define the Identifier struct
#[derive(Debug)]
pub struct Identifier {
    pub token: Token, // the token.IDENT token
    pub value: String,
}

// Implement Node for Identifier
impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

// Implement Expression for Identifier
impl Expression for Identifier {
    fn expression_node(&self) {}
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub token: Token, // the 'return' token
    pub return_value: Box<dyn Expression>,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}
impl Statement for ReturnStatement {
    fn statement_node(&self) {}
    fn as_any(&self) -> &dyn Any {
        self
    }
}