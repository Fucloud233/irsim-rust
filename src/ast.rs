
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
    Func(String),

    Assign{
        l: Variable, 
        r: Variable
    },
    Arth{
        l: Variable,
        r: Variable,
        opt: Operator,
        target: Variable
    },
    Goto(String),
    IfGoto{
        l: Variable,
        r: Variable,
        opt: Operator,
        target: String
    },
    Return(Variable),
    Dec{
        var: Variable,
        size: usize,
    },
    Arg(Variable),
    Call{
        var: Variable,
        func: String
    },
    Param(Variable),
    Read(Variable),
    Write(Variable)
}