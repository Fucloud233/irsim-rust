use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use lalrpop_util::lalrpop_mod;

use crate::ast::{Sentence, Variable};
use crate::error::{
    InterpreterError as IError, 
    InterpreterErrorKind::*
};

lalrpop_mod!(pub parser);

struct Symbol {
    offset: usize,
    size: usize,
    is_array: bool
}

impl Symbol {
    fn new_number(offset: usize) -> Self {
        Symbol { offset, size:4, is_array: false }
    }

    fn new_array(offset: usize, length: usize) -> Self {
        Symbol { offset, size: 4 * length, is_array: true }
    }
}


// in the origin project
// all the symbol are recorded in a same table
// so the dictionary of function must be recorded
pub struct Interpreter<'a> {
    codes: Vec<Sentence<'a>>,
    label_table: RefCell<BTreeMap<&'a str, usize>>,
    func_table: RefCell<BTreeMap<&'a str, usize>>,

    symbol_table_stack: RefCell<Vec<Symbol>>,
    ip: RefCell<usize>,
    entrance_ip: RefCell<Option<usize>>
}

impl <'a>Interpreter<'a> {
    pub fn from_lines(lines: &Vec<&'a str>) -> Result<Interpreter<'a>, IError> {
        let sent_parser = parser::SentenceParser::new();
        
        let codes = lines.iter()
            .map(|line|sent_parser.parse(line).unwrap())
            .collect::<Vec<Sentence>>();
        
        let interpreter = Self { 
            codes: codes,
            label_table: RefCell::new(BTreeMap::new()),
            func_table: RefCell::new(BTreeMap::new()),

            symbol_table_stack: RefCell::new(Vec::new()),

            ip: RefCell::new(0),
            entrance_ip: RefCell::new(None)
        };
        interpreter.check()?;

        Ok(interpreter)
    }   

    // it is difficult to decouple checking and loading
    pub fn check(& self) -> Result<(), IError> {
        let mut cur_func: Option<&str> = None;
        let mut symbol_table: BTreeSet<&str> = BTreeSet::new(); 

        let (mut goto_labels, mut call_funcs) = (Vec::new(), Vec::new());
        
        // check label and variable
        for (i, code) in self.codes.iter().enumerate() {
            // 1. check label
            if self.check_label(code, i, &mut cur_func)? {
                // if current function change, symbol variable should be cleared 
                symbol_table.clear()
            } else if cur_func.is_none() {
                IError::new_err(CurrentFuncNoneError, i)?
            }

            // 2. check variable
            self.check_var(code, i, &mut symbol_table, &mut goto_labels, &mut call_funcs)?;
        }
        
        // check goto and function
        for (item, i) in goto_labels {
            if self.label_table.borrow().get(item).is_none() {
                IError::new_err(UndefinedLabelError, i)?
            }
        }
        for (item, i) in call_funcs {
            if self.func_table.borrow().get(item).is_none() {
                IError::new_err(UndefinedFuncError, i)?
            }
        }

        // main function not found
        match self.entrance_ip.borrow().as_ref() {
            Some(i) => *self.ip.borrow_mut() = *i,
            None => IError::new_err(IRSyntaxError, self.codes.len())?
        };
    
        Ok(())
    }

    #[inline]
    fn check_var(
        &self,
        code: &Sentence<'a>, 
        i: usize,
        symbol_table: &mut BTreeSet<&'a str>,
        goto_labels: &mut Vec<(&'a str, usize)>,
        call_funcs: &mut Vec<(&'a str, usize)> ) -> Result<(), IError>  
    {

        let check_variable_exist = |var: &Variable| {
            match var.get_id() {
                Some(id) => symbol_table.get(id).is_some(),
                None => false,
            }
        };

        // 2. check variable
        match code {
            Sentence::Read(var) | Sentence::Param(var)  => {
                // code `a = read()` will be converted to `READ b; a := b`
                // so b always be variable
                match var {
                    Variable::Id(id) => { symbol_table.insert(id); }
                    _ => IError::new_err(IRSyntaxError, i)?
                };
            },
            Sentence::Write(var) 
                | Sentence::Arg(var) 
                | Sentence::Return(var) => 
            {
                if !check_variable_exist(var) {
                    IError::new_err(UndefinedVariableError, i)?
                }
            }
            Sentence::Dec { var, size } => {
                // the size to allocate must be the number of 4
                if size % 4 != 0 {
                    IError::new_err(IRSyntaxError, i)?
                } else if check_variable_exist(var) {
                    IError::new_err(DuplicatedVariableError, i)?
                }
                symbol_table.insert(var.get_id().unwrap());

            },
            // don't distinguish between declaration and assignment
            Sentence::Assign { l, r } => {
                if !check_variable_exist(r) {
                    IError::new_err(UndefinedVariableError, i)?
                } else if let Some(id) = l.get_id() {
                    symbol_table.insert(id);
                };
            },
            Sentence::Arth { l, r, target, .. } => {
                if !(check_variable_exist(l) && check_variable_exist(r)) {
                    IError::new_err(UndefinedVariableError, i)?
                } else if let Some(id) = target.get_id() {
                    symbol_table.insert(id);
                };
            },
            Sentence::Call { var, func } => {
                if let Some(id) = var.get_id() {
                    symbol_table.insert(id);
                };
                call_funcs.push((*func, i));
            },
            Sentence::IfGoto { target, l, r,  .. } => {
                if !(check_variable_exist(l) && check_variable_exist(r)) {
                    IError::new_err(UndefinedVariableError, i)?
                }
                goto_labels.push((*target, i))
            },
            Sentence::Goto(target) => goto_labels.push((*target, i)),
            _ => todo!()
        };

        Ok(())
    }

    /// check label  and return 
    #[inline]
    fn check_label(
        &self, 
        code: &Sentence<'a>, 
        i: usize, 
        cur_func: &mut Option<&'a str>) -> Result<bool, IError>  
    {
        Ok(if let Sentence::Label(label) = code {
            let mut label_table = self.label_table.borrow_mut();

            if label_table.get(label).is_some() {
                IError::new_err(DuplicatedLabelError, i)?
            } else if *label == "main" {
                IError::new_err(IRSyntaxError, i)?
            }
            label_table.insert(label, i);

            false
        } else if let Sentence::Func(label) = code {
            let mut func_table = self.func_table.borrow_mut();
            
            if func_table.get(label).is_some() {
                IError::new_err(DuplicatedFuncError, i)?
            } else if *label == "main" {
                self.entrance_ip.borrow_mut().get_or_insert(i);
            }
            // record function name and line no
            func_table.insert(&label, i);
            // modify the current function
            cur_func.get_or_insert(label);

            true
        }else {
            false
        }) 
    }
    
    fn exec_code() {

    }
}

    fn initialize() {

    }