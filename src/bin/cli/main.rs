
use std::{io::stdin, process::exit};
use clap::Parser;
use core::{
    interpreter::Interpreter,
    debugger::{Debugger, Message}, 
    utils::io::{
        read_line, read_lines_from_file
    },
};


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The file you want to run
    file: String,

    /// Open debug mode
    #[arg(short, long)]
    debug: bool,
}

#[inline]
fn print_run_result(result: Result<usize, Message>) {
    match result {
        Ok(count) => print_over(count),
        Err(message) => eprint!("{}", message)
    }
}

#[inline]
fn print_over(count: usize) {
    eprintln!("\nProgram has exited successfully!");
    eprintln!("Total instructions = {}", count);
}

fn main() {
    let read_func = Box::new(||{
        let mut input = String::new();
        let _ = stdin().read_line(&mut input).unwrap();
        input
    });

    let write_func = Box::new(|text: String|{
        print!("{}", text)
    });

    // define cli i/o function
    let Args {file, debug} = Args::parse();

    // read lines
    let lines = read_lines_from_file(&file);
    let ref_liens = &lines.iter().map(|s| s as &str).collect();

    let interpreter = match Interpreter::from_lines(ref_liens, read_func, write_func) {
        Ok(i) => i,
        Err(err) => { dbg!(err); exit(1) }
    };


    let debugger = Debugger::new(interpreter);

    if !debug {
        let result = debugger.run();
        print_run_result(result);
        return;
    }

    loop {
        eprint!("> ");
        let cmd = read_line();
        
        match cmd.as_str() {
            "exit" => return,
            "run" => {
                let result = debugger.run();
                print_run_result(result);
            },
            "step" => match debugger.step() {
                Err(msg) => eprintln!("{}", msg),
                Ok(result) => if let Some(count) = result {
                    print_over(count);
                } 
            },
            "stop" => {
                let msg = debugger.stop();
                eprintln!("{}", msg);
            },
            _ => eprintln!("Input Error: Command not found!")
        };
    }

    
}
