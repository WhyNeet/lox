use crate::expression::Expression;

#[derive(Debug)]
pub enum Statement {
    Print(Expression),
    Expression(Expression),
}
