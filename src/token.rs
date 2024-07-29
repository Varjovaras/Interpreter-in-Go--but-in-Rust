#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenType, // Use an enum for different token types
    pub literal: String,
}

impl Token {
    #[must_use]
    pub const fn new(token_type: TokenType, literal: String) -> Self {
        Self {
            kind: token_type,
            literal,
        }
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, PartialEq, Eq, Clone)]
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
    LParen,
    RParen,
    LBrace,
    RBrace,
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
    Eq,
    NotEq,
}

impl TokenType {
    fn from_keyword(ident: &str) -> Self {
        // dbg!(ident);
        match ident {
            "fn" => Self::Function,
            "let" => Self::Let,
            "true" => Self::True,
            "false" => Self::False,
            "if" => Self::If,
            "else" => Self::Else,
            "return" => Self::Return,
            _ => Self::Ident,
        }
    }
}

#[must_use]
pub fn lookup_ident(ident: &str) -> TokenType {
    TokenType::from_keyword(ident)
}
