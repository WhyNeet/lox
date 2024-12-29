pub mod error;
pub mod runtime;

use ::error::InterpreterError;
use ast::{expression::Expression, literal::Literal, operator::Operator};
use error::{RuntimeError, RuntimeErrorKind, RuntimeResult};
use runtime::value::RuntimeValue;

pub struct Runtime {}

impl Runtime {
    pub fn new() -> Self {
        Self {}
    }
}

impl Runtime {
    pub fn evaluate(&self, expr: &Expression) -> RuntimeResult<RuntimeValue> {
        match expr {
            Expression::Binary {
                left,
                operator,
                right,
            } => self.binary(&left, operator, &right),
            Expression::Unary { operator, right } => self.unary(operator, right),
            Expression::Literal(literal) => self.literal(literal),
            Expression::Conditional {
                condition,
                then,
                alternative,
            } => self.conditional(condition, then, alternative),
            Expression::Grouping(expr) => self.grouping(expr),
        }
    }

    fn literal(&self, literal: &Literal) -> RuntimeResult<RuntimeValue> {
        Ok(match literal {
            Literal::Boolean(value) => RuntimeValue::Boolean(*value),
            Literal::String(value) => RuntimeValue::String(value.clone()),
            Literal::Number(value) => {
                if value.fract() == 0.0 {
                    RuntimeValue::Integer(*value as i64)
                } else {
                    RuntimeValue::Float(*value)
                }
            }
            Literal::Nil => RuntimeValue::Nil,
        })
    }

    fn grouping(&self, expr: &Expression) -> RuntimeResult<RuntimeValue> {
        self.evaluate(expr)
    }

    fn unary(&self, operator: &Operator, expr: &Expression) -> RuntimeResult<RuntimeValue> {
        let right = self.evaluate(&expr)?;

        match operator {
            Operator::Subtraction => -&right,
            Operator::Addition => Some(right),
            Operator::Negation => !&right,
            _ => None,
        }
        .ok_or(InterpreterError::new(RuntimeError::new(
            RuntimeErrorKind::ExpectedNumberOperand,
        )))
    }

    fn binary(
        &self,
        left: &Expression,
        operator: &Operator,
        right: &Expression,
    ) -> RuntimeResult<RuntimeValue> {
        let left = self.evaluate(&left)?;
        let right = self.evaluate(&right)?;

        match operator {
            Operator::Addition => &left + &right,
            Operator::Subtraction => &left - &right,
            Operator::Multiplication => &left * &right,
            Operator::Division => &left / &right,
            Operator::Greater => Some(RuntimeValue::boolean(
                left.partial_cmp(&right)
                    .map(|ord| ord.is_gt())
                    .unwrap_or(false),
            )),
            Operator::GreaterOrEqual => Some(RuntimeValue::boolean(
                left.partial_cmp(&right)
                    .map(|ord| ord.is_ge())
                    .unwrap_or(false),
            )),
            Operator::Less => Some(RuntimeValue::boolean(
                left.partial_cmp(&right)
                    .map(|ord| ord.is_lt())
                    .unwrap_or(false),
            )),
            Operator::LessOrEqual => Some(RuntimeValue::boolean(
                left.partial_cmp(&right)
                    .map(|ord| ord.is_ge())
                    .unwrap_or(false),
            )),
            Operator::Equal => Some(RuntimeValue::boolean(
                left.partial_cmp(&right)
                    .map(|ord| ord.is_eq())
                    .unwrap_or(false),
            )),
            Operator::NotEqual => Some(RuntimeValue::boolean(
                left.partial_cmp(&right)
                    .map(|ord| ord.is_ne())
                    .unwrap_or(false),
            )),
            _ => unreachable!(),
        }
        .ok_or(InterpreterError::new(RuntimeError::new(
            RuntimeErrorKind::ExpectedNumberOperand,
        )))
    }

    fn conditional(
        &self,
        condition: &Expression,
        then: &Expression,
        alternative: &Expression,
    ) -> RuntimeResult<RuntimeValue> {
        let condition_result = self.evaluate(condition)?;

        if condition_result == RuntimeValue::boolean(true) {
            self.evaluate(then)
        } else {
            self.evaluate(alternative)
        }
    }
}
