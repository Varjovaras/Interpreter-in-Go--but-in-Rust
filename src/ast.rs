// Define the Token struct

use crate::token::Token;

// Define the Node trait
pub trait Node {
    fn token_literal(&self) -> String;
}

// Define the Statement trait extending Node
pub trait Statement: Node {
    fn statement_node(&self);
}

// Define the Expression trait extending Node
pub trait Expression: Node {
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
pub struct LetStatement {
    pub token: Token, // the token.LET token
    pub name: Box<Identifier>,
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
}

// Define the Identifier struct
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