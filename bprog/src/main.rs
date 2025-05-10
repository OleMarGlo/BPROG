use std::{fs, env};
use types::Value;


mod operations;
mod types;
mod stack;
mod variables;
mod functions;

fn main() -> Result<(), String> {
    let args = env::args().collect::<Vec<_>>();
    let file = args.get(1).unwrap();


    let mut variables = variables::Variables::new();
    let mut functions = functions::Functions::new();
    let mut stack = stack::Stack::new();

    if let Ok(text) = fs::read_to_string(file) {
        let replaced = text.replace("\n", " ");
        let block = Value::Block(replaced.split_whitespace().map(|s| s.to_string()).collect());
        block.exec(&mut stack, &mut variables, &mut functions)?;
    }
    Ok(())
}