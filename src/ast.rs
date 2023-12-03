
pub enum Operator {
    Plus, Sub, Mul, Div, 
    Equal, Greater, Less, GreaterEqual, LessEqual
}

#[derive(Debug, PartialEq)]
pub enum Variable {
    Number(i64),
    Pointer(String),
    Deref(String),
    Id(String)
}

pub enum Sentence {
    Label(String),
    FUNCTION(String),

    Assign(Variable, Variable),
    Arth{
        l: Variable,
        r: Variable,
        opt: Operator
    },
    Goto(String),
    IfGoto{
        l: Variable,
        r: Variable,
        opt: Operator,
        target: String
    },
    Return(Variable),
    Dec(Variable, usize),
    Arg(Variable),
    Call{
        l: Variable,
        func: String
    },
    Param(Variable),
    Read(Variable),
    Write(Variable)
}