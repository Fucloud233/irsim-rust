pub mod ast; 
pub mod interpreter;
pub mod error;
pub mod debugger;
mod computer;

pub mod utils {
    pub mod io;
}

#[cfg(test)]
mod test {
    mod lexer;
    mod parser;
}

