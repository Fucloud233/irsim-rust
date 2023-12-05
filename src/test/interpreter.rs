use std::io::stdin;

use crate::interpreter::Interpreter;



#[test]
fn test_io() {
    let lines = vec![ "FUNCTION main :", "READ a", "RETURN #0" ];

    let read_func = Box::new(||{
        let mut input = String::new();
        let _ = stdin().read_line(&mut input).unwrap();
        input
    });

    let write_func = Box::new(|text: &str|{});

    let interpreter = Interpreter::from_lines(&lines, read_func, write_func).unwrap();
    loop {
        let result = interpreter.exec_code().unwrap();
        if result {
            break
        }
    }
}