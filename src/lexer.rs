use crate::token::{lookup_ident, Token, TokenType};

#[derive(Debug)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    char: char,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut a = Self {
            input: input.to_string(),
            position: 0,
            read_position: 0,
            char: input.chars().nth(0).unwrap(),
        };
        a.read_char();
        a
    }

    pub fn read_char(&mut self) {
        // dbg!(self.char);
        if self.read_position >= self.input.len() {
            self.char = '\0';
        } else {
            self.char = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        dbg!(self.char);
        let token = match self.char {
            '=' => new_token(TokenType::Assign, self.char),
            ';' => new_token(TokenType::Semicolon, self.char),
            '(' => new_token(TokenType::Lparen, self.char),
            ')' => new_token(TokenType::Rparen, self.char),
            ',' => new_token(TokenType::Comma, self.char),
            '+' => new_token(TokenType::Plus, self.char),
            '-' => new_token(TokenType::Minus, self.char),
            '!' => new_token(TokenType::Bang, self.char),
            '/' => new_token(TokenType::Slash, self.char),
            '*' => new_token(TokenType::Asterisk, self.char),
            '<' => new_token(TokenType::Lt, self.char),
            '>' => new_token(TokenType::Gt, self.char),
            '{' => new_token(TokenType::Lbrace, self.char),
            '}' => new_token(TokenType::Rbrace, self.char),
            '\0' => new_token(TokenType::Eof, '\0'),
            _ => {
                if is_letter(self.char) {
                    let literal = self.read_identifier();
                    return Token {
                        kind: lookup_ident(&literal),
                        literal,
                    };
                } else if is_digit(self.char) {
                    return Token {
                        kind: TokenType::Int,
                        literal: self.read_number(),
                    };
                } else {
                    return new_token(TokenType::Illegal, '_');
                }
            }
        };
        self.read_char();
        token
    }

    fn read_identifier(&mut self) -> String {
        let start_position = self.position;
        while is_letter(self.char) {
            self.read_char();
        }
        // dbg!(self.input[start_position..self.position].to_string());
        self.input[start_position..self.position].to_string()
    }

    fn read_number(&mut self) -> String {
        let start_position = self.position;
        while is_digit(self.char) {
            self.read_char();
        }
        self.input[start_position..self.position].to_string()
    }

    fn skip_whitespace(&mut self) {
        while self.char == ' ' || self.char == '\t' || self.char == '\n' || self.char == '\r' {
            self.read_char();
        }
    }
}

fn new_token(token_type: TokenType, char: char) -> Token {
    Token {
        kind: token_type,
        literal: char.to_string(),
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_ascii_lowercase() || ch.is_ascii_uppercase() || ch == '_'
}

fn is_digit(ch: char) -> bool {
    ch.is_ascii_digit()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::TokenType;

    #[test]
    fn test_next_token_with_identifiers() {
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

    #[test]
    fn test_next_token_with_actual_syntax() {
        let input = "let five = 5;
        let ten = 10;
           let add = fn(x, y) {
             x + y;
        };
           let result = add(five, ten);
           !-/*5;
           5 < 10 > 5;
           
           if (5 < 10) {
                return true;
           } else {
                return false;
           }
           ";

        let mut lexer = Lexer::new(input);
        let tests = vec![
            (TokenType::Let, "let"),
            (TokenType::Ident, "five"),
            (TokenType::Assign, "="),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "ten"),
            (TokenType::Assign, "="),
            (TokenType::Int, "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "add"),
            (TokenType::Assign, "="),
            (TokenType::Function, "fn"),
            (TokenType::Lparen, "("),
            (TokenType::Ident, "x"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "y"),
            (TokenType::Rparen, ")"),
            (TokenType::Lbrace, "{"),
            (TokenType::Ident, "x"),
            (TokenType::Plus, "+"),
            (TokenType::Ident, "y"),
            (TokenType::Semicolon, ";"),
            (TokenType::Rbrace, "}"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "result"),
            (TokenType::Assign, "="),
            (TokenType::Ident, "add"),
            (TokenType::Lparen, "("),
            (TokenType::Ident, "five"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "ten"),
            (TokenType::Rparen, ")"),
            (TokenType::Semicolon, ";"),
            (TokenType::Bang, "!"),
            (TokenType::Minus, "-"),
            (TokenType::Slash, "/"),
            (TokenType::Asterisk, "*"),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Int, "5"),
            (TokenType::Lt, "<"),
            (TokenType::Int, "10"),
            (TokenType::Gt, ">"),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::If, "if"),
            (TokenType::Lparen, "("),
            (TokenType::Int, "5"),
            (TokenType::Lt, "<"),
            (TokenType::Int, "10"),
            (TokenType::Rparen, ")"),
            (TokenType::Lbrace, "{"),
            (TokenType::Return, "return"),
            (TokenType::True, "true"),
            (TokenType::Semicolon, ";"),
            (TokenType::Rbrace, "}"),
            (TokenType::Else, "else"),
            (TokenType::Lbrace, "{"),
            (TokenType::Return, "return"),
            (TokenType::False, "false"),
            (TokenType::Semicolon, ";"),
            // (TokenType::Rbrace, "}"),
            // (TokenType::Int, "10"),
            // (TokenType::Eq, "=="),
            // (TokenType::Int, "10"),
            // (TokenType::Semicolon, ";"),
            // (TokenType::Int, "10"),
            // (TokenType::NotEq, "!="),
            // (TokenType::Int, "9"),
            // (TokenType::Semicolon, ";"),
            // (TokenType::Eof, "\0"),
        ];
        for (i, (expected_type, expected_literal)) in tests.iter().enumerate() {
            let tok = lexer.next_token();
            // dbg!(&tok);
            assert_eq!(
                tok.literal, *expected_literal,
                "tests[{}] - literal wrong. expected={}, got={}",
                i, expected_literal, tok.literal
            );
            assert_eq!(
                tok.kind, *expected_type,
                "tests[{}] - tokentype wrong. expected={:?}, got={:?}",
                i, expected_type, tok.kind
            );
        }
    }
}
