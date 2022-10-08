use crate::token_type::TokenType;

#[derive(Debug)]
pub enum Literal {
    String(String),
    Identifier(String),
    Number(usize),
}

pub struct Token {
    pub token_type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    pub line: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<Literal>,
        line: usize,
    ) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn print(&self) {
        println!(
            "type {:#?}, lexeme {}, literal {:#?}",
            self.token_type, self.lexeme, self.literal
        )
    }
}
