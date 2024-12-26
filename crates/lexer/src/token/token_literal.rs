pub enum TokenLiteral {
    String(String),
    Number(f64),
}

impl TokenLiteral {
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Self::String(str) => Some(str),
            _ => None,
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        match self {
            Self::Number(num) => Some(*num),
            _ => None,
        }
    }
}
