use token_literal::TokenLiteral;
use token_type::TokenType;

pub mod token_literal;
pub mod token_type;

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize,
    literal: Option<TokenLiteral>,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        line: usize,
        literal: Option<TokenLiteral>,
    ) -> Self {
        Self {
            lexeme,
            line,
            token_type,
            literal,
        }
    }

    pub fn token_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn lexeme(&self) -> &str {
        &self.lexeme
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn literal(&self) -> Option<&TokenLiteral> {
        self.literal.as_ref()
    }
}
