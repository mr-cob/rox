use std::collections::HashMap;

use crate::{
    errors::Error,
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
    keywords: HashMap<String, TokenType>,
    pub errors: Vec<Error>,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Scanner {
            source: source.to_string(),
            source_as_bytes: source.as_bytes().to_vec(),
            errors: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords: HashMap::new(),
            tokens: Vec::new(),
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        self.init_keywords();
        while !self.is_eof() {
            self.scan_token();
            self.start = self.current;
        }

        return &self.tokens;
    }
}

impl Scanner {
    fn scan_token(&mut self) {
        let current_charecter = self.advance();
        match current_charecter {
            ' ' | '\r' | '\t' => return,
            '\n' => self.line += 1,
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
                if self.is_match('/') {
                    self.comment();
                } else if self.is_match('*') {
                    self.multiline_comment();
                } else {
                    self.add_token_without_literal(TokenType::Slash);
                }
            }
            '!' => {
                if self.is_match('=') {
                    self.add_token_without_literal(TokenType::BangEqual);
                } else {
                    self.add_token_without_literal(TokenType::Bang);
                }
            }
            '=' => {
                if self.is_match('=') {
                    self.add_token_without_literal(TokenType::EqualEqual);
                } else {
                    self.add_token_without_literal(TokenType::Equal);
                }
            }
            '<' => {
                if self.is_match('=') {
                    self.add_token_without_literal(TokenType::LessEqual);
                } else {
                    self.add_token_without_literal(TokenType::Less);
                }
            }
            '>' => {
                if self.is_match('=') {
                    self.add_token_without_literal(TokenType::GreaterEqual);
                } else {
                    self.add_token_without_literal(TokenType::Greater);
                }
            }
            '"' => self.make_string(),
            _ => {
                if self.is_digit(current_charecter) {
                    self.make_number();
                } else if self.is_alpha(current_charecter) {
                    self.make_identifier();
                } else {
                    self.add_error("Unexpected Token");
                    self.add_token_without_literal(TokenType::Invalid);
                }
            }
        }
    }

    fn is_eof(&self) -> bool {
        self.current >= self.source.len()
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

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        return self.source_as_bytes[self.current + 1] as char;
    }

    fn is_match(&mut self, expected: char) -> bool {
        if self.is_eof() {
            return false;
        };
        if self.peek() != expected {
            return false;
        };

        self.current += 1;
        return true;
    }

    fn is_digit(&self, charecter: char) -> bool {
        charecter >= '0' && charecter <= '9'
    }

    fn is_alpha(&self, charecter: char) -> bool {
        charecter >= 'a' && charecter <= 'z'
            || charecter >= 'A' && charecter <= 'Z'
            || charecter == '_'
    }

    fn is_alpha_numeric(&self, charecter: char) -> bool {
        self.is_alpha(charecter) || self.is_digit(charecter)
    }

    fn comment(&mut self) {
        while self.peek() != '\n' && !self.is_eof() {
            self.advance();
        }
    }

    fn multiline_comment(&mut self) {
        loop {
            if self.is_eof() {
                self.add_error("Unterminated comment");
                self.add_token_without_literal(TokenType::Invalid);
                return;
            }
            match self.peek() {
                '*' => {
                    self.advance();
                    if self.is_match('/') {
                        return;
                    }
                }
                '/' => {
                    self.advance();
                    if self.is_match('*') {
                        self.multiline_comment();
                    }
                }
                '\n' => {
                    self.advance();
                    self.line += 1;
                }
                _ => {
                    self.advance();
                }
            }
        }
    }

    fn make_identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }
        let identifier = self.source[self.start..self.current].to_string();
        if let Some(_) = self.keywords.get(&identifier) {
            return;
        };
        self.add_token_without_literal(TokenType::Identifier);
    }

    fn make_number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance(); // consuming the dot
            while self.is_digit(self.peek()) {
                self.advance();
            }
        }
        let literal: f64 = self.source[self.start..self.current].parse().unwrap();
        self.add_token(TokenType::Number, Option::Some(Literal::Number(literal)));
    }

    fn make_string(&mut self) {
        while self.peek() != '"' && !self.is_eof() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_eof() {
            self.add_error("Unterminated String");
            self.add_token_without_literal(TokenType::Invalid);
            return;
        }

        self.advance(); // covering up the ending qoute

        self.add_token(
            TokenType::String,
            Option::Some(Literal::String(
                self.source[self.start + 1..self.current - 1].to_string(),
            )),
        );
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

    fn add_error(&mut self, message: &str) {
        self.errors.push(Error::new(self.line, message));
    }

    fn init_keywords(&mut self) {
        self.keywords.insert(String::from("and"), TokenType::And);
        self.keywords
            .insert(String::from("class"), TokenType::Class);
        self.keywords.insert(String::from("else"), TokenType::Else);
        self.keywords
            .insert(String::from("false"), TokenType::False);
        self.keywords.insert(String::from("for"), TokenType::For);
        self.keywords.insert(String::from("fun"), TokenType::Fun);
        self.keywords.insert(String::from("nil"), TokenType::Nil);
        self.keywords.insert(String::from("if"), TokenType::If);
        self.keywords.insert(String::from("or"), TokenType::Or);
        self.keywords
            .insert(String::from("print"), TokenType::Print);
        self.keywords
            .insert(String::from("return"), TokenType::Return);
        self.keywords
            .insert(String::from("super"), TokenType::Super);
        self.keywords.insert(String::from("this"), TokenType::This);
        self.keywords.insert(String::from("true"), TokenType::True);
        self.keywords.insert(String::from("var"), TokenType::Var);
        self.keywords
            .insert(String::from("while"), TokenType::While);
    }
}
