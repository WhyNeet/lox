pub mod error;
pub mod runtime;

use std::{cell::RefCell, rc::Rc};

use ::error::InterpreterError;
use ast::{expression::Expression, literal::Literal, operator::Operator, statement::Statement};
use error::{RuntimeError, RuntimeErrorKind, RuntimeResult};
use runtime::{environment::Environment, signal::RuntimeSignal, value::RuntimeValue};

pub struct Runtime {
    environment: RefCell<Rc<Environment>>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            environment: RefCell::new(Rc::new(Environment::new())),
        }
    }

    fn environment(&self) -> Rc<Environment> {
        Rc::clone(&self.environment.borrow())
    }
}

impl Runtime {
    pub fn run(&self, program: &Vec<Rc<Statement>>) -> RuntimeResult<()> {
        if let Some(signal) = self._run(program)? {
            Err(InterpreterError::new(RuntimeError::new(match signal {
                RuntimeSignal::LoopBreak => RuntimeErrorKind::BreakNotWithinLoop,
                RuntimeSignal::LoopContinue => RuntimeErrorKind::ContinueNotWithinLoop,
                RuntimeSignal::FunctionReturn(_) => RuntimeErrorKind::ReturnNotWithinFunction,
            })))
        } else {
            Ok(())
        }
    }

    fn _run(&self, program: &Vec<Rc<Statement>>) -> RuntimeResult<Option<RuntimeSignal>> {
        for stmt in program {
            if let Some(signal) = self.statement(stmt)? {
                return Ok(Some(signal));
            }
        }

        Ok(None)
    }

    fn statement(&self, stmt: &Statement) -> RuntimeResult<Option<RuntimeSignal>> {
        match stmt {
            Statement::Expression(expr) => self.expr_stmt(&expr).map(|_| None),
            Statement::Print(expr) => self.print_stmt(&expr).map(|_| None),
            Statement::VariableDeclaration {
                identifier,
                expression,
            } => self
                .var_stmt(identifier.to_string(), &expression)
                .map(|_| None),
            Statement::FunctionDeclaration {
                identifier,
                parameters,
                execute,
            } => self.fun_stmt(identifier.to_string(), parameters.clone(), execute),
            Statement::Block(statements) => self.block(statements),
            Statement::Conditional {
                condition,
                then,
                alternative,
            } => self.conditional_stmt(condition, then, alternative.as_ref()),
            Statement::While { condition, block } => self.loop_stmt(condition, block),
            Statement::Break => Ok(Some(RuntimeSignal::LoopBreak)),
            Statement::Continue => Ok(Some(RuntimeSignal::LoopContinue)),
            Statement::Return(expression) => Ok(Some(RuntimeSignal::FunctionReturn(
                self.evaluate(expression)?,
            ))),
        }
    }

    fn fun_stmt(
        &self,
        identifier: String,
        parameters: Vec<String>,
        execute: &Statement,
    ) -> RuntimeResult<Option<RuntimeSignal>> {
        let closure = Rc::clone(&self.environment.borrow());

        let execute = match execute {
            Statement::Block(statements) => statements,
            _ => unreachable!(),
        };

        let function = RuntimeValue::callable(parameters, execute.to_vec(), closure);
        self.environment().define(identifier, Rc::new(function))?;

        Ok(None)
    }

    fn loop_stmt(
        &self,
        condition: &Expression,
        block: &Statement,
    ) -> RuntimeResult<Option<RuntimeSignal>> {
        while self.evaluate(condition)?.as_ref().into() {
            if let Some(signal) = self.statement(block)? {
                match signal {
                    RuntimeSignal::LoopBreak => break,
                    RuntimeSignal::LoopContinue => continue,
                    other => return Ok(Some(other)),
                }
            }
        }

        Ok(None)
    }

    fn conditional_stmt(
        &self,
        condition: &Expression,
        then: &Statement,
        alternative: Option<&Box<Statement>>,
    ) -> RuntimeResult<Option<RuntimeSignal>> {
        let condition_result = self.evaluate(&condition)?;

        // if negated runtime value is false
        let signal = if !<_ as TryInto<bool>>::try_into(&(!&*condition_result).unwrap()).unwrap() {
            self.statement(then)?
        } else if let Some(alternative) = alternative {
            self.statement(alternative)?
        } else {
            None
        };

        Ok(signal)
    }

    fn block(&self, statements: &Vec<Rc<Statement>>) -> RuntimeResult<Option<RuntimeSignal>> {
        let environment = Rc::new(Environment::with_enclosing(Rc::clone(
            &self.environment.borrow(),
        )));

        let prev_environment = self.environment.replace(environment);

        let signal = self._run(statements)?;

        self.environment.replace(prev_environment);

        Ok(signal)
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
            Expression::FunctionInvokation { callee, arguments } => {
                self.function_invokation(callee, arguments)
            }
        }
    }

    fn function_invokation(
        &self,
        callee: &Expression,
        arguments: &Vec<Expression>,
    ) -> RuntimeResult<Rc<RuntimeValue>> {
        let callee_expr = self.evaluate(callee)?;

        match callee_expr.as_ref() {
            RuntimeValue::Callable {
                execute,
                closure,
                parameters,
            } => {
                if arguments.len() != parameters.len() {
                    return Err(InterpreterError::new(RuntimeError::new(
                        RuntimeErrorKind::InvalidArgumentCount(arguments.len(), parameters.len()),
                    )));
                }

                let environment = Environment::with_enclosing(Rc::clone(closure));
                for (idx, argument) in arguments.iter().enumerate() {
                    let name = parameters[idx].to_string();
                    let argument_value = self.evaluate(argument)?;
                    environment.define(name, argument_value)?;
                }

                let prev_environment = self.environment.replace(Rc::new(environment));

                let signal = self.block(execute)?;

                self.environment.replace(prev_environment);

                let return_value = if let Some(signal) = signal {
                    match signal {
                        RuntimeSignal::FunctionReturn(value) => value,
                        RuntimeSignal::LoopBreak => {
                            return Err(InterpreterError::new(RuntimeError::new(
                                RuntimeErrorKind::BreakNotWithinLoop,
                            )))
                        }
                        RuntimeSignal::LoopContinue => {
                            return Err(InterpreterError::new(RuntimeError::new(
                                RuntimeErrorKind::ContinueNotWithinLoop,
                            )))
                        }
                    }
                } else {
                    Rc::new(RuntimeValue::Nil)
                };

                Ok(return_value)
            }
            _ => Err(InterpreterError::new(RuntimeError::new(
                RuntimeErrorKind::ExpressionNotCallable,
            ))),
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
            Operator::Greater => Some(RuntimeValue::boolean(left.gt(&right.unwrap()))),
            Operator::GreaterOrEqual => Some(RuntimeValue::boolean(left.ge(&right.unwrap()))),
            Operator::Less => Some(RuntimeValue::boolean(left.lt(&right.unwrap()))),
            Operator::LessOrEqual => Some(RuntimeValue::boolean(left.le(&right.unwrap()))),
            Operator::Equal => Some(RuntimeValue::boolean(left.eq(&right.unwrap()))),
            Operator::NotEqual => Some(RuntimeValue::boolean(left.ne(&right.unwrap()))),
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
