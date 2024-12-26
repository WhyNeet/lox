use token_type::TokenType;

pub mod token_type;

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: u32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: u32) -> Self {
        Self {
            lexeme,
            line,
            token_type,
        }
    }

    pub fn token_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn lexeme(&self) -> &str {
        &self.lexeme
    }

    pub fn line(&self) -> u32 {
        self.line
    }
}
