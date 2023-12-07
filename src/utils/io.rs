use std::{fs, io::stdin};

pub fn read_lines_from_file (filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .expect("Should have been able to read the file")
        .lines()
        .map(|str| str.trim().into())
        .collect()
}

pub fn read_line() -> String {
    let mut input = String::new();
    let _ = stdin().read_line(&mut input);
    input.trim().into()
}