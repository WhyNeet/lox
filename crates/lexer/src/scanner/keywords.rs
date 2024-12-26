use std::{collections::HashMap, sync::LazyLock};

use crate::token::token_type::TokenType;

pub static KEYWORDS: LazyLock<HashMap<String, TokenType>> = LazyLock::new(|| {
    let mut map = HashMap::new();

    map.insert("and".to_string(), TokenType::And);
    map.insert("class".to_string(), TokenType::Class);
    map.insert("else".to_string(), TokenType::Else);
    map.insert("false".to_string(), TokenType::False);
    map.insert("for".to_string(), TokenType::For);
    map.insert("fun".to_string(), TokenType::Fun);
    map.insert("while".to_string(), TokenType::While);
    map.insert("if".to_string(), TokenType::If);
    map.insert("nil".to_string(), TokenType::Nil);
    map.insert("or".to_string(), TokenType::Or);
    map.insert("return".to_string(), TokenType::Return);
    map.insert("print".to_string(), TokenType::Print);
    map.insert("super".to_string(), TokenType::Super);
    map.insert("this".to_string(), TokenType::This);
    map.insert("true".to_string(), TokenType::True);
    map.insert("var".to_string(), TokenType::Var);

    map
});
