use std::{cell::RefCell, collections::HashMap, rc::Rc};

use error::InterpreterError;

use crate::error::{RuntimeError, RuntimeErrorKind, RuntimeResult};

use super::value::RuntimeValue;

#[derive(Default)]
pub struct Environment {
    values: RefCell<HashMap<String, Rc<RuntimeValue>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self::default()
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
        values.get(identifier).map(Rc::clone)
    }
}
