use std::rc::Rc;

use super::value::RuntimeValue;

#[derive(Debug)]
pub enum RuntimeSignal {
    LoopBreak,
    LoopContinue,
    FunctionReturn(Rc<RuntimeValue>),
}
