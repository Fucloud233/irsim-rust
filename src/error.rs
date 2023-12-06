use lalrpop_util::{ParseError, lexer::Token};

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
    InputError,
    LeftValueError,
}