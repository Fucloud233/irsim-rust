use std::fmt::{Display, self};

use lalrpop_util::{ParseError, lexer::Token};

trait BaseError: Display {
    fn msg(&self) -> &'static str;
}

#[derive(Debug)]
pub struct InterpreterError<'a> {
    kind: InterpreterErrorKind<'a>,
    i: usize
}

impl <'a>InterpreterError<'a> {
    // pub fn new(kind: InterpreterErrorKind<'a>, i: usize) -> Self {
    //     InterpreterError{kind, i}
    // }

    pub fn new_err<T>(kind: InterpreterErrorKind<'a>, i: usize) -> Result<T, Self> {
        Err(InterpreterError{kind, i})
    }
}

#[derive(Debug)]
pub enum InterpreterErrorKind<'a> {
    ParseError(ParseError<usize, Token<'a>, &'static str>),
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
    UndefinedFuncError,

    // input
    LeftValueError,
}

#[derive(Debug)]
pub struct RuntimeError {
    kind: RuntimeErrorKind,
    i: usize
}

#[derive(Debug)]
pub enum RuntimeErrorKind {
    InputError,
}

impl RuntimeError {
    pub fn new_err<T>(kind: RuntimeErrorKind, i: usize) -> Result<T, RuntimeError>{
        Err(RuntimeError {kind, i})
    }

    pub fn message(&self) -> String {
        let msg = match self.kind {
            RuntimeErrorKind::InputError => "input must be number",
        };

        format!("Runtime error at line {}: {}", self.i, msg)
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self.kind {
            RuntimeErrorKind::InputError => "input must be number",
        };
        write!(f, "Runtime error at line {}: {}", self.i, msg)
    }
}