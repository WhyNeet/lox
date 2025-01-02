use std::rc::Rc;

use crate::expression::Expression;

#[derive(Debug)]
pub enum Statement {
    Print(Expression),
    Expression(Expression),
    VariableDeclaration {
        identifier: String,
        expression: Expression,
    },
    FunctionDeclaration {
        identifier: String,
        parameters: Vec<String>,
        execute: Box<Statement>,
    },
    Block(Vec<Rc<Statement>>),
    Conditional {
        condition: Expression,
        then: Box<Statement>,
        alternative: Option<Box<Statement>>,
    },
    While {
        condition: Expression,
        block: Box<Statement>,
    },
    Break,
    Continue,
}
