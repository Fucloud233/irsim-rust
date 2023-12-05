mod ast; 
mod computer;
mod interpreter;
mod error;
#[cfg(test)]
mod test {
    mod lexer;
    mod parser;
}


use interpreter::Interpreter;

fn main() {
    println!("Hello, world!");

    let lines = vec![];
    let interpreter = Interpreter::from_lines(&lines);
}
