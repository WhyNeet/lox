use lexer::token::{token_type::TokenType, Token};

#[derive(Debug)]
pub enum Operator {
    Equal,
    NotEqual,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Negation,
    Assignment,
    Conjunction,
    Disjunction,
}

impl TryFrom<&Token> for Operator {
    type Error = String;

    fn try_from(value: &Token) -> Result<Self, Self::Error> {
        match value.token_type() {
            TokenType::EqualEqual => Ok(Self::Equal),
            TokenType::BangEqual => Ok(Self::NotEqual),
            TokenType::And => Ok(Self::Conjunction),
            TokenType::Bang => Ok(Self::Negation),
            TokenType::Equal => Ok(Self::Assignment),
            TokenType::Or => Ok(Self::Disjunction),
            TokenType::Greater => Ok(Self::Greater),
            TokenType::GreaterEqual => Ok(Self::GreaterOrEqual),
            TokenType::Less => Ok(Self::Less),
            TokenType::LessEqual => Ok(Self::LessOrEqual),
            TokenType::Minus => Ok(Self::Subtraction),
            TokenType::Plus => Ok(Self::Addition),
            TokenType::Star => Ok(Self::Multiplication),
            TokenType::Slash => Ok(Self::Division),
            other => Err(format!("unknown operator: {other:?}")),
        }
    }
}
