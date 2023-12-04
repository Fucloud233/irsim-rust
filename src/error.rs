#[derive(Debug)]
pub struct InterpreterError {
    kind: InterpreterErrorKind,
    i: usize
}

impl InterpreterError {
    pub fn new(kind: InterpreterErrorKind, i: usize) -> Self {
        InterpreterError{kind, i}
    }

    pub fn new_err(kind: InterpreterErrorKind, i: usize) -> Result<(), Self> {
        Err(InterpreterError{kind, i})
    }
}

#[derive(Debug)]
pub enum InterpreterErrorKind {
    IRSyntaxError,
    // label
    DuplicatedLabelError,
    UndefinedLabelError,
    
    // variable
    DuplicatedVariableError,
    UndefinedVariableError,

    // function
    CurrentFuncNoneError,
    DuplicatedFuncError,
}