use crate::{literal::Literal, operator::Operator};

#[derive(Debug)]
pub enum Expression {
    Binary {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>,
    },
    Unary {
        operator: Operator,
        right: Box<Expression>,
    },
    Literal(Literal),
    Grouping(Box<Expression>),
    Conditional {
        condition: Box<Expression>,
        then: Box<Expression>,
        alternative: Box<Expression>,
    },
    Identifier(String),
}
