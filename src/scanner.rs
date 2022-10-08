use crate::{
    token::{Literal, Token},
    token_type::TokenType,
};

pub struct Scanner {
    source: String,
    source_as_bytes: Vec<u8>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Scanner {
            source: source.to_string(),
            source_as_bytes: source.as_bytes().to_vec(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_eof() {
            self.scan_token();
            self.start = self.current;
        }

        return &self.tokens;
    }
}

impl Scanner {
    fn is_eof(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let current = self.advance();
        match current {
            '(' => self.add_token_without_literal(TokenType::LeftParen),
            ')' => self.add_token_without_literal(TokenType::RightParen),
            '{' => self.add_token_without_literal(TokenType::LeftBrace),
            '}' => self.add_token_without_literal(TokenType::RightBrace),
            ';' => self.add_token_without_literal(TokenType::Semicolon),
            ',' => self.add_token_without_literal(TokenType::Comma),
            '.' => self.add_token_without_literal(TokenType::Dot),
            '+' => self.add_token_without_literal(TokenType::Plus),
            '-' => self.add_token_without_literal(TokenType::Minus),
            '*' => self.add_token_without_literal(TokenType::Star),
            '/' => {
                if self.match_next('/') {
                    while self.peek() != '\n' && !self.is_eof() {
                        self.advance();
                    }
                } else {
                    self.add_token_without_literal(TokenType::Slash);
                }
            }
            '!' => {
                if self.match_next('=') {
                    self.add_token_without_literal(TokenType::BangEqual);
                } else {
                    self.add_token_without_literal(TokenType::Bang);
                }
            }
            '=' => {
                if self.match_next('=') {
                    self.add_token_without_literal(TokenType::EqualEqual);
                } else {
                    self.add_token_without_literal(TokenType::Equal);
                }
            }
            '<' => {
                if self.match_next('=') {
                    self.add_token_without_literal(TokenType::LessEqual);
                } else {
                    self.add_token_without_literal(TokenType::Less);
                }
            }
            '>' => {
                if self.match_next('=') {
                    self.add_token_without_literal(TokenType::GreaterEqual);
                } else {
                    self.add_token_without_literal(TokenType::Greater);
                }
            }
            _ => self.add_token_without_literal(TokenType::Invalid),
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source_as_bytes[self.current - 1] as char
    }

    fn peek(&self) -> char {
        if self.is_eof() {
            return '\0';
        }
        return self.source_as_bytes[self.current] as char;
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_eof() {
            return false;
        };
        if self.peek() != expected {
            return false;
        };

        self.current += 1;
        return true;
    }

    fn add_token_without_literal(&mut self, token_type: TokenType) {
        self.add_token(token_type, None);
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        self.tokens.push(Token::new(
            token_type,
            self.source[self.start..self.current].to_string(),
            literal,
            self.line,
        ));
    }
}
