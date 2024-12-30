use std::{cell::RefCell, collections::HashMap, rc::Rc};

use error::InterpreterError;

use crate::error::{RuntimeError, RuntimeErrorKind, RuntimeResult};

use super::value::RuntimeValue;

#[derive(Default)]
pub struct Environment {
    values: RefCell<HashMap<String, Rc<RuntimeValue>>>,
    enclosing: Option<Rc<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_enclosing(enclosing: Rc<Environment>) -> Self {
        Self {
            enclosing: Some(enclosing),
            ..Default::default()
        }
    }

    pub fn define(&self, identifier: String, value: Rc<RuntimeValue>) -> RuntimeResult<()> {
        if self.values.borrow().contains_key(&identifier) {
            Err(InterpreterError::new(RuntimeError::new(
                RuntimeErrorKind::VariableAlreadyDefined(identifier),
            )))
        } else {
            self.values.borrow_mut().insert(identifier, value);
            Ok(())
        }
    }

    pub fn get(&self, identifier: &str) -> Option<Rc<RuntimeValue>> {
        let values = self.values.borrow();
        let value = values.get(identifier).map(Rc::clone);

        if value.is_none() && self.enclosing.is_some() {
            self.enclosing.as_ref().unwrap().get(identifier)
        } else {
            value
        }
    }

    pub fn assign(&self, identifier: String, value: Rc<RuntimeValue>) -> RuntimeResult<()> {
        if !self.values.borrow().contains_key(&identifier) {
            if self.enclosing.is_some() {
                self.enclosing.as_ref().unwrap().assign(identifier, value)
            } else {
                Err(InterpreterError::new(RuntimeError::new(
                    RuntimeErrorKind::VariableNotDefined(identifier),
                )))
            }
        } else {
            self.values.borrow_mut().insert(identifier, value);
            Ok(())
        }
    }
}
