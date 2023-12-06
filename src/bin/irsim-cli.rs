
use core::interpreter::Interpreter;
use std::{io::stdin, process::exit, fs};

fn read_lines (filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .expect("Should have been able to read the file")
        .lines()
        .map(|str| str.trim().into())
        .collect()
}


fn main() {
    
    let filename = "./test/test_d1.ir";
    let lines = read_lines(filename);

    let read_func = Box::new(||{
        let mut input = String::new();
        let _ = stdin().read_line(&mut input).unwrap();
        input
    });

    let write_func = Box::new(|text: String|{
        print!("{}", text)
    });

    let interpreter = match Interpreter::from_lines(&lines.iter().map(|s| s as &str).collect(), read_func, write_func) {
        Ok(i) => i,
        Err(err) => { dbg!(err); exit(1) }
    };
    loop {
        let result = interpreter.execute().unwrap();
        match result {
            None =>(),
            Some(count) => {
                println!("Count: {}", count); 
                break
            }
        }
    }
}
