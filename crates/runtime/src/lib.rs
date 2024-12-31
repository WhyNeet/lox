pub mod error;
pub mod runtime;

use std::{cell::RefCell, rc::Rc};

use ::error::InterpreterError;
use ast::{expression::Expression, literal::Literal, operator::Operator, statement::Statement};
use error::{RuntimeError, RuntimeErrorKind, RuntimeResult};
use runtime::{environment::Environment, value::RuntimeValue};

pub struct Runtime {
    environment: RefCell<Option<Rc<Environment>>>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            environment: RefCell::new(Some(Rc::new(Environment::new()))),
        }
    }

    fn environment(&self) -> Rc<Environment> {
        self.environment.borrow().as_ref().map(Rc::clone).unwrap()
    }
}

impl Runtime {
    pub fn run(&self, program: &Vec<Statement>) -> RuntimeResult<()> {
        for stmt in program {
            self.statement(stmt)?;
        }

        Ok(())
    }

    fn statement(&self, stmt: &Statement) -> RuntimeResult<()> {
        match stmt {
            Statement::Expression(expr) => self.expr_stmt(&expr),
            Statement::Print(expr) => self.print_stmt(&expr),
            Statement::VariableDeclaration {
                identifier,
                expression,
            } => self.var_stmt(identifier.to_string(), &expression),
            Statement::Block(statements) => self.block(statements),
            Statement::Conditional {
                condition,
                then,
                alternative,
            } => self.conditional_stmt(condition, then, alternative.as_ref()),
            Statement::While { condition, block } => self.loop_stmt(condition, block),
        }
    }

    fn loop_stmt(&self, condition: &Expression, block: &Statement) -> RuntimeResult<()> {
        while self.evaluate(condition)?.as_ref().into() {
            self.statement(block)?;
        }

        Ok(())
    }

    fn conditional_stmt(
        &self,
        condition: &Expression,
        then: &Statement,
        alternative: Option<&Box<Statement>>,
    ) -> RuntimeResult<()> {
        let condition_result = self.evaluate(&condition)?;

        // if negated runtime value is false
        if !<_ as TryInto<bool>>::try_into(&(!&*condition_result).unwrap()).unwrap() {
            self.statement(then)?;
        } else if let Some(alternative) = alternative {
            self.statement(alternative)?;
        }

        Ok(())
    }

    fn block(&self, statements: &Vec<Statement>) -> RuntimeResult<()> {
        let prev_environment = self.environment.take().unwrap();

        *self.environment.borrow_mut() = Some(Rc::new(Environment::with_enclosing(Rc::clone(
            &prev_environment,
        ))));

        self.run(statements)?;

        *self.environment.borrow_mut() = Some(prev_environment);

        Ok(())
    }

    fn var_stmt(&self, identifier: String, expr: &Expression) -> RuntimeResult<()> {
        let value = self.evaluate(expr)?;

        self.environment().define(identifier, value)?;

        Ok(())
    }

    fn print_stmt(&self, expr: &Expression) -> RuntimeResult<()> {
        let value = self.evaluate(expr)?;

        println!("{value}");

        Ok(())
    }

    fn expr_stmt(&self, expr: &Expression) -> RuntimeResult<()> {
        self.evaluate(expr)?;

        Ok(())
    }

    fn evaluate(&self, expr: &Expression) -> RuntimeResult<Rc<RuntimeValue>> {
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
            Expression::Identifier(identifier) => {
                self.environment()
                    .get(identifier)
                    .ok_or(InterpreterError::new(RuntimeError::new(
                        RuntimeErrorKind::VariableNotDefined(identifier.to_string()),
                    )))
            }
            Expression::Assignment {
                identifier,
                expression,
            } => self
                .environment()
                .assign(identifier.to_string(), self.evaluate(&expression)?)
                .map(|_| Rc::new(RuntimeValue::nil())),
        }
    }

    fn literal(&self, literal: &Literal) -> RuntimeResult<Rc<RuntimeValue>> {
        Ok(Rc::new(match literal {
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
        }))
    }

    fn grouping(&self, expr: &Expression) -> RuntimeResult<Rc<RuntimeValue>> {
        self.evaluate(expr)
    }

    fn unary(&self, operator: &Operator, expr: &Expression) -> RuntimeResult<Rc<RuntimeValue>> {
        let right = self.evaluate(&expr)?;

        match operator {
            Operator::Subtraction => (-&*right).map(Rc::new),
            Operator::Addition => Some(right),
            Operator::Negation => (!&*right).map(Rc::new),
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
        right_ast: &Expression,
    ) -> RuntimeResult<Rc<RuntimeValue>> {
        let left = self.evaluate(&left)?;
        let right = if *operator != Operator::Conjunction && *operator != Operator::Disjunction {
            Some(self.evaluate(&right_ast)?)
        } else {
            None
        };

        match operator {
            Operator::Addition => &*left + &*right.unwrap(),
            Operator::Subtraction => &*left - &*right.unwrap(),
            Operator::Multiplication => &*left * &*right.unwrap(),
            Operator::Division => {
                if right.as_ref().unwrap().as_ref() == &RuntimeValue::integer(0)
                    || right.as_ref().unwrap().as_ref() == &RuntimeValue::float(0.)
                {
                    return Err(InterpreterError::new(RuntimeError::new(
                        RuntimeErrorKind::ZeroDivision,
                    )));
                } else {
                    &*left / &*right.unwrap()
                }
            }
            Operator::Greater => Some(RuntimeValue::boolean(
                left.partial_cmp(&right.unwrap())
                    .map(|ord| ord.is_gt())
                    .unwrap_or(false),
            )),
            Operator::GreaterOrEqual => Some(RuntimeValue::boolean(
                left.partial_cmp(&right.unwrap())
                    .map(|ord| ord.is_ge())
                    .unwrap_or(false),
            )),
            Operator::Less => Some(RuntimeValue::boolean(
                left.partial_cmp(&right.unwrap())
                    .map(|ord| ord.is_lt())
                    .unwrap_or(false),
            )),
            Operator::LessOrEqual => Some(RuntimeValue::boolean(
                left.partial_cmp(&right.unwrap())
                    .map(|ord| ord.is_le())
                    .unwrap_or(false),
            )),
            Operator::Equal => Some(RuntimeValue::boolean(
                left.partial_cmp(&right.unwrap())
                    .map(|ord| ord.is_eq())
                    .unwrap_or(false),
            )),
            Operator::NotEqual => Some(RuntimeValue::boolean(
                left.partial_cmp(&right.unwrap())
                    .map(|ord| ord.is_ne())
                    .unwrap_or(false),
            )),
            Operator::Conjunction => Some(RuntimeValue::boolean(
                (&*left).into() && (&*self.evaluate(right_ast)?).into(),
            )),
            Operator::Disjunction => Some(RuntimeValue::boolean(
                (&*left).into() || (&*self.evaluate(right_ast)?).into(),
            )),
            _ => unreachable!(),
        }
        .map(Rc::new)
        .ok_or(InterpreterError::new(RuntimeError::new(
            RuntimeErrorKind::ExpectedNumberOperand,
        )))
    }

    fn conditional(
        &self,
        condition: &Expression,
        then: &Expression,
        alternative: &Expression,
    ) -> RuntimeResult<Rc<RuntimeValue>> {
        let condition_result = self.evaluate(condition)?;

        if *condition_result == RuntimeValue::boolean(true) {
            self.evaluate(then)
        } else {
            self.evaluate(alternative)
        }
    }
}
