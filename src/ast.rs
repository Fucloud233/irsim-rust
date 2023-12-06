
#[derive(Debug, PartialEq)]
pub enum Operator {
    Plus, Sub, Mul, Div, 
    Equal, Greater, Less, GreaterEqual, LessEqual
}

impl Operator {
    pub fn calculate(&self, l: i32, r: i32) -> i32 {
        match self {
            Operator::Plus => l + r,
            Operator::Sub => l - r,
            Operator::Mul => l * r,
            Operator::Div => l / r,
            _ => {
                let flag = match self {
                    Operator::Equal => l == r,
                    Operator::Greater => l > r,
                    Operator::Less => l < r,
                    Operator::GreaterEqual => l >= r,
                    Operator::LessEqual => l <= r,
                    _ => unreachable!()
                };
                if flag {1} else {0}
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Variable<'a> {
    Number(i32),
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
    Arith{
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
        size: i32,
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