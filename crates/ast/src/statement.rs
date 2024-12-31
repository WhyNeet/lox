use crate::expression::Expression;

#[derive(Debug)]
pub enum Statement {
    Print(Expression),
    Expression(Expression),
    VariableDeclaration {
        identifier: String,
        expression: Expression,
    },
    Block(Vec<Statement>),
    Conditional {
        condition: Expression,
        then: Box<Statement>,
        alternative: Option<Box<Statement>>,
    },
}
