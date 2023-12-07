use std::{cell::RefCell, ops::BitXorAssign, fmt::{Display, self}};
use crate::interpreter::Interpreter;

pub struct Message {
    msg: String,
    kind: MessageKind
}

enum MessageKind {
    Warn, Info, Error
}

impl Message {
    fn new(msg: String, kind: MessageKind) -> Self {
        Message { msg, kind }
    }

    fn from_str(msg: &'static str, kind: MessageKind) -> Self {
        Message { msg: msg.to_string(), kind }
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind = match self.kind {
            MessageKind::Warn => "warn",
            MessageKind::Info => "info",
            MessageKind::Error => "error",
        };
        write!(f, "[{}] {}", kind, self.msg)
    }
}


pub struct Debugger<'a> {
    interpreter: Interpreter<'a>,

    is_running: RefCell<bool>,
    check_stop: RefCell<bool>,
}

impl<'a> Debugger<'a> {
    pub fn new(interpreter: Interpreter<'a>) -> Self {
        Debugger { 
            interpreter,
            is_running: RefCell::new(false),
            check_stop: RefCell::new(false)
        }
    } 

    pub fn run(&self) -> Result<usize, Message> {
        loop {
            let result = self.interpreter.execute().or_else(
                |e| Err(Message::new(e.message(), MessageKind::Error))
            )?;
            if let Some(count) = result {
                break Ok(count)
            }
        }
    }

    pub fn step(&self) -> Result<Option<usize>, Message> {
        let mut is_running = self.is_running.borrow_mut();
        if ! *is_running {
            is_running.bitxor_assign(true);
        } 
        Ok(self.interpreter.execute()
            .or_else(|e| Err(Message::new(e.message(), MessageKind::Error)))?
        )
    }

    pub fn stop(&self) -> Message {
        if ! *self.is_running.borrow() {
            return Message::from_str("program not start", MessageKind::Warn);
        }

        let mut flag = self.check_stop.borrow_mut();
        if !*flag {
            flag.bitxor_assign(true);
            Message::from_str("program is stopped", MessageKind::Info)
        } else {
            flag.bitxor_assign(true);
            Message::from_str("program is running, please type 'stop' again to stop", MessageKind::Warn)
        }
    }
}