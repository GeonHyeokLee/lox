use crate::token::Token;
use crate::token::TokenType;

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    fn new(source: &str) -> Self {
        Scanner {
            source: source.to_string(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::Eof, "", None, self.line));
        &self.tokens
    }

    fn scan_token(&mut self) {
        let char = self.advance().unwrap(); // TODO: 에러 핸들링 필요
        
        match char {
            '(' => {
                self.add_token(TokenType::LeftParen, None);
            },
            ')' => {
                self.add_token(TokenType::RightParen, None);
            },
            '{' => {
                self.add_token(TokenType::LeftBrace, None);
            },
            '}' => {
                self.add_token(TokenType::RightBrace, None);
            },
            ',' => {
                self.add_token(TokenType::Comma, None);
            },
            '.' => {
                self.add_token(TokenType::Dot, None);
            },
            '-' => {
                self.add_token(TokenType::Minus, None);
            },
            '+' => {
                self.add_token(TokenType::Plus, None);
            },
            ';' => {
                self.add_token(TokenType::Semicolon, None);
            },
            '*' => {
                self.add_token(TokenType::Star, None);
            },
            '!' => {
                let token_type = match self.is_match('=') {
                    true => {
                        TokenType::BangEqual
                    }
                    false => {
                        TokenType::Bang
                    }
                };
                self.add_token(token_type, None)
            },
            '=' => {
                let token_type = match self.is_match('=') {
                    true => {
                        TokenType::EqualEqual
                    }
                    false => {
                        TokenType::Equal
                    }
                };
                self.add_token(token_type, None)
            },
            '<' => {
                let token_type = match self.is_match('=') {
                    true => {
                        TokenType::LessEqual
                    }
                    false => {
                        TokenType::Less
                    }
                };
                self.add_token(token_type, None)
            },
            '>' => {
                let token_type = match self.is_match('=') {
                    true => {
                        TokenType::GreaterEqual
                    }
                    false => {
                        TokenType::Greater
                    }
                };
                self.add_token(token_type, None)
            },
            _ => {
                eprintln!("Unexpected character.");
            },
        }
    }

    fn is_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false
        }

        if let Some(char) = self.source.chars().nth(self.current) {
            if char != expected {
                return false
            }
        }

        self.current = self.current + 1;
        true
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> Option<char> {
        let mut chars = self.source.chars();
        self.current = self.current + 1;
        chars.nth(self.current)
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<String>) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token::new(token_type, text, literal, self.line));
    }

}