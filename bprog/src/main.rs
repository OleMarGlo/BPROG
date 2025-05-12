use std::{env, fs, io};
use types::Value;


mod operations;
mod types;
mod stack;
mod variables;
mod functions;

fn main() -> Result<(), String> {
    let args = env::args().collect::<Vec<_>>();

    // initialize variables, functions, and stack
    let mut variables = variables::Variables::new();
    let mut functions = functions::Functions::new();
    let mut stack = stack::Stack::new();

    // if there is an argument it SHOULD be a file name
    if args.len() == 1 {
        if let Ok(text) = fs::read_to_string(&args[1]) {
            let replaced = text.replace("\n", " ");      // replace newlines with spaces for easier parsing
            let block = Value::Block(replaced.split_whitespace().map(|s| s.to_string()).collect());
            if let Err(e) = block.exec(&mut stack, &mut variables, &mut functions) {        // execute the block untill error
                panic!("{}", e);
            }
            if stack.len() == 1 {        // if the stack has only one value on it, print it
                println!("{}", stack.pop().unwrap());
            } else {                    // if the stack has more or less than one value on it, panic    
                panic!("Invalid stack size, on exit only one value should be on the stack, but {} are on the stack", stack);
            }
        }
    } else if args.len() == 0 {
        loop {      // loop untill program is terminated
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).unwrap();
            let block = Value::Block(buf.split_whitespace().map(|s| s.to_string()).collect());
            match block.exec(&mut stack, &mut variables, &mut functions) {
                Ok(_) => {println!{"Stack: {}", stack}},
                Err(e) => println!("Error: {}", e),
            }
        }
    } else {
        panic!("Invalid number of arguments");
    }
    Ok(())
}