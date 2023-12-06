pub mod ast; 
pub mod interpreter;
pub mod error;
mod computer;
#[cfg(test)]
mod test {
    mod lexer;
    mod parser;
}

