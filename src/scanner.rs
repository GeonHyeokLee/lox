use crate::token::Token;
use crate::token::TokenType;

use once_cell::sync::Lazy;
use std::collections::HashMap;

static KEYWORDS: Lazy<HashMap<&'static str, TokenType>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("and", TokenType::And);
    m.insert("class", TokenType::Class);
    m.insert("else", TokenType::Else);
    m.insert("false", TokenType::False);
    m.insert("for", TokenType::For);
    m.insert("fun", TokenType::Fun);
    m.insert("if", TokenType::If);
    m.insert("nil", TokenType::Nil);
    m.insert("or", TokenType::Or);
    m.insert("print", TokenType::Print);
    m.insert("return", TokenType::Return);
    m.insert("super", TokenType::Super);
    m.insert("this", TokenType::This);
    m.insert("true", TokenType::True);
    m.insert("var", TokenType::Var);
    m.insert("while", TokenType::While);
    m
});

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Scanner {
            source: source.to_string(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::Eof, "", None, self.line));
        &self.tokens
    }

    fn scan_token(&mut self) {
        let char = self.advance().expect("Not found char.");
        
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
            '/' => {
                if self.is_match('/') {
                    while self.peek() != Some('\n') && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.is_match('*') { // Annotation handling
                    while self.peek() != Some('*') && !self.is_at_end() {
                        self.advance();
                    }
                    self.advance(); // Consume '*'
                    self.advance(); // Consume '/'
                } else {
                    self.add_token(TokenType::Slash, None);
                }
            },
            ' ' => {},
            '\r' => {},
            '\t' => {},
            '\n' => {
                self.line = self.line + 1;
            },
            '"' => {
                self.string();
            },
            'o' => {
                if self.is_match('r') {
                    self.add_token(TokenType::Or, None);
                }
            }
            _ => {
                if Scanner::is_digit(char) {
                    self.number();
                } else if Scanner::is_alpha(char) { 
                    self.identifier();
                } else {
                    eprintln!("Unexpected character.");
                }
            },
        }
    }

    // 예약어
    fn identifier(&mut self) {
        while Scanner::is_alpha_numeric(self.peek().unwrap()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let token_type = KEYWORDS.get(text).cloned();
        match token_type {
            Some(token_type) => {
                self.add_token(token_type, None);
            }
            _ => {
                self.add_token(TokenType::Identifier, None);
            }
        }
    }

    fn number(&mut self) {
        while Scanner::is_digit(self.peek().unwrap())  {
            self.advance();
        }

        let char = self.peek();
        match char {
            Some(char) => {
                if char == '.' && Scanner::is_digit(self.peek_next().unwrap()) {
                    self.advance();

                    while Scanner::is_digit(self.peek().unwrap())  {
                        self.advance();
                    }
                }
            }
            _ => {}
        }

        let literal = &self.source[self.start..self.current];
        self.add_token(TokenType::Number, Some(literal.to_string()));
    }

    fn string(&mut self) {
        loop {
            let char = self.peek();
            match char {
                Some(char) => {
                    if char == '"' {
                        break;
                    }

                    if char == '\n' {
                        self.line = self.line + 1;
                    }
                    self.advance();
                }
                _ => {
                    if self.is_at_end() {
                        break;
                    }
                }
            }
        }

        if self.is_at_end() {
            eprintln!("Unterminated string.");
            return;
        }

        self.advance(); // Closed '"'

        let value = &self.source[self.start + 1.. self.current - 1];
        self.add_token(TokenType::String, Some(value.to_string()));
    }

    fn is_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false
        }

        let char = self.source.chars().nth(self.current);
        match char {
            Some(char) => {
                if char != expected {
                    return false
                }
            }
            _ => {}
        }

        self.current = self.current + 1;
        true
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            return Some('\0')
        }

        self.source.chars().nth(self.current)
    }

    fn peek_next(&self) -> Option<char> {
        if self.current + 1 >= self.source.len() {
            return Some('\0');
        }

        self.source.chars().nth(self.current + 1)
    }

    fn is_alpha_numeric(c: char) -> bool {
        Scanner::is_alpha(c) || Scanner::is_digit(c)
    }

    fn is_alpha(c: char) -> bool {
        c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c == '_'
    }

    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> Option<char> {
        let mut chars = self.source.chars();
        let char = chars.nth(self.current);
        self.current = self.current + 1;
        char
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<String>) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token::new(token_type, text, literal, self.line));
    }
}