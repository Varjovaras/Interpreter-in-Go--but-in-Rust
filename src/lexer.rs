use crate::token::{Token, TokenType};

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    char: char,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
            position: 0,
            read_position: 1,
            char: input.chars().nth(0).unwrap(),
        }
    }

    pub fn read_char(&mut self) {
        if self.position >= self.input.len() {
            self.char = '\0';
        } else {
            self.char = self.input.chars().nth(self.position).unwrap()
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.read_char();
        match self.char {
            '=' => new_token(TokenType::Assign, self.char),
            ';' => new_token(TokenType::Semicolon, self.char),
            '(' => new_token(TokenType::Lparen, self.char),
            ')' => new_token(TokenType::Rparen, self.char),
            ',' => new_token(TokenType::Comma, self.char),
            '+' => new_token(TokenType::Plus, self.char),
            '{' => new_token(TokenType::Lbrace, self.char),
            '}' => new_token(TokenType::Rbrace, self.char),
            '\0' => new_token(TokenType::Eof, self.char),
            _ => new_token(TokenType::Illegal, '_'),
        }
    }
}

fn new_token(token_type: TokenType, char: char) -> Token {
    Token {
        kind: token_type,
        literal: char.to_string(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::TokenType;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";
        let mut lexer = Lexer::new(input);

        let tests = vec![
            (TokenType::Assign, "="),
            (TokenType::Plus, "+"),
            (TokenType::Lparen, "("),
            (TokenType::Rparen, ")"),
            (TokenType::Lbrace, "{"),
            (TokenType::Rbrace, "}"),
            (TokenType::Comma, ","),
            (TokenType::Semicolon, ";"),
            (TokenType::Eof, "\0"),
        ];

        for (i, (expected_type, expected_literal)) in tests.iter().enumerate() {
            let tok = lexer.next_token();
            assert_eq!(
                tok.kind, *expected_type,
                "tests[{}] - tokentype wrong. expected={:?}, got={:?}",
                i, expected_type, tok.kind
            );
            assert_eq!(
                tok.literal, *expected_literal,
                "tests[{}] - literal wrong. expected={}, got={}",
                i, expected_literal, tok.literal
            );
        }
    }
}
