#[derive(Debug)]
pub struct Token {
    pub kind: TokenType, // Use an enum for different token types
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Self {
        Self {
            kind: token_type,
            literal,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Illegal,
    Eof,
    Ident,
    Int,
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Function,
    Let,
}

impl TokenType {
    fn from_keyword(ident: &str) -> Self {
        // dbg!(ident);
        match ident {
            "fn" => TokenType::Function,
            "let" => TokenType::Let,
            "=" => TokenType::Assign,
            _ => TokenType::Ident,
        }
    }
}

pub fn lookup_ident(ident: &str) -> TokenType {
    TokenType::from_keyword(ident)
}
