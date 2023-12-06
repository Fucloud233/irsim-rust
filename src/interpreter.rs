use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::ops::AddAssign;
use std::str::FromStr; 
use lalrpop_util::lalrpop_mod;

use crate::{
    ast::{Sentence, Variable},
    computer::Computer,
    error::{
        InterpreterError as IError, 
        InterpreterErrorKind::*
    },
};

lalrpop_mod!(pub parser);

struct Symbol {
    addr: u32,
    size: u32,
    is_array: bool
}

impl Symbol {
    fn new_number(addr: u32) -> Self {
        Symbol { addr, size:4, is_array: false }
    }

    fn new_array(addr: u32, length: u32) -> Self {
        Symbol { addr, size: 4 * length, is_array: true }
    }
}

struct Call<'a> {
    ip: usize, 
    var: Variable<'a>,
}

impl<'a> Call<'a> {
    fn new(ip: usize, var: Variable<'a>) -> Self  {
        Call {ip, var}
    }
}


// in the origin project
// all the symbol are recorded in a same table
// so the dictionary of function must be recorded
pub struct Interpreter<'a> {
    codes: Vec<Sentence<'a>>,

    read: Box<dyn Fn() -> String>,
    write: Box<dyn Fn(String) -> ()>,
    
    label_table: RefCell<BTreeMap<&'a str, usize>>,
    func_table: RefCell<BTreeMap<&'a str, usize>>,

    count: RefCell<u32>,
    symbol_table_stack: RefCell<Vec<BTreeMap<&'a str, Symbol>>>,
    call_stack: RefCell<Vec<Call<'a>>>,
    computer: RefCell<Computer>,
    ip: RefCell<usize>,
    entrance_ip: RefCell<Option<usize>>
}

impl <'a>Interpreter<'a> {
    pub fn from_lines(
        lines: &Vec<&'a str>,
        read: Box<dyn Fn() -> String>,
        write: Box<dyn Fn(String) -> ()>) 
        -> Result<Interpreter<'a>, IError> 
    {
        let sent_parser = parser::SentenceParser::new();
        
        let codes = lines.iter()
            .map(|line|sent_parser.parse(line).unwrap())
            .collect::<Vec<Sentence>>();
        
        let interpreter = Self { 
            codes: codes,
            label_table: RefCell::new(BTreeMap::new()),
            func_table: RefCell::new(BTreeMap::new()),
            
            read, write,

            symbol_table_stack: RefCell::new(Vec::new()),
            call_stack: RefCell::new(Vec::new()),
            count: RefCell::new(0),
            computer: RefCell::new(Computer::new()),
            ip: RefCell::new(0),
            entrance_ip: RefCell::new(None)
        };
        interpreter.check()?;

        Ok(interpreter)
    }   

    // it is difficult to decouple checking and loading
    fn check(& self) -> Result<(), IError> {
        let mut cur_func: Option<&str> = None;
        let mut symbol_table: BTreeSet<&str> = BTreeSet::new(); 

        let (mut goto_labels, mut call_funcs) = (Vec::new(), Vec::new());
        
        // check label and variable
        for (i, code) in self.codes.iter().enumerate() {
            // TODO: when check label, it need to continue
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
            None => IError::new_err(IRSyntaxError, self.codes.len())?,
            Some(i) => {
                *self.ip.borrow_mut() = *i;
                self.symbol_table_stack.borrow_mut().push(BTreeMap::new());
            },
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
        let check_var_not_exist = |var: &Variable, symbol_table: &BTreeSet<&str>|{
            match var.get_id() {
                Some(id) => symbol_table.get(id).is_none(),
                None => false,
            }
        };

        if let Sentence::Assign { target,.. } 
            | Sentence::Arith { target, .. }
            | Sentence::Call {target, ..} = code 
        {
            // check left value
            match target {
                Variable::Number(_) | Variable::Pointer(_)
                    => IError::new_err(LeftValueError, i)?,
                Variable::Id(id) => {
                    if symbol_table.get(id).is_some() {
                        IError::new_err(DuplicatedVariableError, i)?
                    }
                    symbol_table.insert(id);
                },
                Variable::Deref(id) 
                    => if symbol_table.get(id).is_none() {
                        IError::new_err(UndefinedVariableError, i)?
                    }
            };

            // check right value
            match code {
                Sentence::Assign { var, .. } => if check_var_not_exist(var, &symbol_table) {
                    IError::new_err(UndefinedVariableError, i)?
                },
                Sentence::Arith { l, r, .. } => if check_var_not_exist(l, &symbol_table) {
                    IError::new_err(UndefinedVariableError, i)?
                } else if check_var_not_exist(r, &symbol_table) {
                    IError::new_err(UndefinedVariableError, i)?
                },
                Sentence::Call { func, ..} => call_funcs.push((*func, i)),
                _ => unreachable!()
            };
            
            return Ok(())
        }
        
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
                if !check_var_not_exist(var, &symbol_table) {
                    IError::new_err(UndefinedVariableError, i)?
                }
            }
            Sentence::Dec { target, size } => {
                // the size to allocate must be the number of 4
                if size % 4 != 0 {
                    IError::new_err(IRSyntaxError, i)?
                }

                match target {
                    Variable::Number(_) | Variable::Pointer(_) | Variable::Deref(_)
                        => IError::new_err(LeftValueError, i)?,
                    Variable::Id(id) => if symbol_table.get(id).is_some() {
                        IError::new_err(DuplicatedFuncError, i)?
                    }
                };

                symbol_table.insert(target.get_id().unwrap());

            },
            Sentence::IfGoto { label, l, r,  .. } => {
                if check_var_not_exist(l, &symbol_table) || check_var_not_exist(r, &symbol_table) {
                    IError::new_err(UndefinedVariableError, i)?
                }
                goto_labels.push((*label, i))
            },
            Sentence::Goto(label) => goto_labels.push((*label, i)),
            _ => unreachable!()
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
    
    pub fn execute(&self) -> Result<bool, IError> {
        
        let ip = *self.ip.borrow();
        let code = match self.codes.get(ip) {
            Some(c) => c,
            None => {println!("{}", ip); todo!()}
        };

        match code {
            Sentence::Read(var) => {
                let input = match u32::from_str((*self.read)().as_str().trim()) {
                    Ok(i) => i,
                    Err(_) => return IError::new_err::<bool>(InputError, ip)
                };
                self.assign_number(var, input);
                
            }
            Sentence::Write(var) => {
                let output = self.get_var(var);
                (*self.write)(format!("{}\n", output));
            },
            Sentence::Goto(label) => self.goto(label),
            Sentence::IfGoto { l, r, opt, label } => {
                let (l_value, r_value) = (self.get_var(l), self.get_var(r));
                if opt.calculate(l_value, r_value) >=1  {
                    self.goto(label);
                }   
            },
            Sentence::Assign { target, var } => self.assign(target, var),
            Sentence::Arith { l, r, opt, target } => {
                let result = opt.calculate(self.get_var(l), self.get_var(r));
                self.assign_number(target, result)
            },
            Sentence::Return(var) => {
                let mut symbol_table_stack = self.symbol_table_stack.borrow_mut();
                if symbol_table_stack.len() == 0 {
                    return Ok(true)
                }
                // TODO: how to recycle the memory
                let return_value = self.get_var(var);
                symbol_table_stack.pop();
                

                // symbol_table_stack.last().unwrap()
            }
            Sentence::Call { target, func } => {
                // let call = Call::new(self.ip.borrow(), self)
                
            }
            _ => ()
        };

        self.count.borrow_mut().add_assign(1);
        self.ip.borrow_mut().add_assign(1);
        Ok(false)
    }

    fn goto(&self, label: &str) {
        let binding = self.label_table.borrow();
        let new_ip = binding.get(label).unwrap();
        *self.ip.borrow_mut() = *new_ip;
    }

    fn get_var(&self, var: &Variable) -> u32{
        if let Variable::Number(number) = var {
            return *number;
        };
        
        let computer = self.computer.borrow();
        // the address always valid
        match var {
            Variable::Pointer(id) => {
                self.get_addr(id)
            },
            Variable::Deref(id) => {
                let addr = self.get_addr(id);
                let new_addr = computer.load(addr);
                computer.load(new_addr)
            },
            Variable::Id(id) => {
                let addr = self.get_addr(id);
                computer.load(addr)
            },
            _ => unreachable!()
        }
    }
    
    fn assign(&self, target: &Variable, var: &Variable) {
        self.assign_number(target, self.get_var(var))
    }

    fn assign_number(&self, target: &Variable, number: u32) {
        let mut computer = self.computer.borrow_mut();

        let addr = match target {
            Variable::Id(id) => self.get_addr(id),
            Variable::Deref(id) => {
                let addr = self.get_addr(id);
                computer.load(addr)
            }
            _ => unreachable!()
        };

        computer.save(addr, number);
    }

    // it must be left value here  
    fn get_addr(&self, id: &str) -> u32 {
        let binding = self.symbol_table_stack.borrow();
        let symbol_table = binding.last().unwrap();
        match symbol_table.get(id) {
            Some(symbol) => symbol.addr,
            // when id isn't in the symbol table
            // we should allocate memory for it
            None => {
                let mut computer = self.computer.borrow_mut();
                computer.allocate(1)
            }
        }
    }
}