
#[derive(Debug, PartialEq)]
pub enum Operator {
    Plus, Sub, Mul, Div, 
    Equal, Greater, Less, GreaterEqual, LessEqual
}

#[derive(Debug, PartialEq)]
pub enum Variable<'a> {
    Number(u32),
    Pointer(&'a str),
    Deref(&'a str),
    Id(&'a str)
}

impl <'a>Variable<'a> {
    pub fn get_id(&self) -> Option<&'a str> {
        if let Variable::Pointer(id)
            | Variable::Deref(id)
            | Variable::Id(id) = self 
        {
            Some(id)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Sentence<'a> {
    Label(&'a str),
    Func(&'a str),

    Assign{
        target: Variable<'a>, 
        var: Variable<'a>,
    },
    Arth{
        l: Variable<'a>,
        r: Variable<'a>,
        opt: Operator,
        target: Variable<'a>,
    },
    Goto(&'a str),
    IfGoto{
        l: Variable<'a>,
        r: Variable<'a>,
        opt: Operator,
        label: &'a str
    },
    Return(Variable<'a>),
    Dec{
        target: Variable<'a>,
        size: u32,
    },
    Arg(Variable<'a>),
    Call{
        target: Variable<'a>,
        func: &'a str
    },
    Param(Variable<'a>),
    Read(Variable<'a>),
    Write(Variable<'a>)
}