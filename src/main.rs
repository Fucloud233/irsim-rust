mod ast; 
mod computer;
mod interpreter;
mod error;
#[cfg(test)]
mod test {
    mod lexer;
    mod parser;
}


fn main() {
    println!("Hello, world!");
}
