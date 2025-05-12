use std::{env, fs, io};
use types::Value;


mod operations;
mod types;
mod stack;
mod variables;
mod functions;

fn main() -> Result<(), String> {
    let args = env::args().collect::<Vec<_>>();


    let mut variables = variables::Variables::new();
    let mut functions = functions::Functions::new();
    let mut stack = stack::Stack::new();

    if args.len() > 1 {
        if let Ok(text) = fs::read_to_string(&args[1]) {
            let replaced = text.replace("\n", " ");
            let block = Value::Block(replaced.split_whitespace().map(|s| s.to_string()).collect());
            if let Err(e) = block.exec(&mut stack, &mut variables, &mut functions) {
                panic!("{}", e);
            }
        }
    } else {
        loop {
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).unwrap();
            let block = Value::Block(buf.split_whitespace().map(|s| s.to_string()).collect());
            match block.exec(&mut stack, &mut variables, &mut functions) {
                Ok(_) => {println!{"Stack: {}", stack}},
                Err(e) => println!("Error: {}", e),
            }
        }
    }
    Ok(())
}