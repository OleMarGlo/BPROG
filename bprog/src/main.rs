use std::fs;

use types::Value;

mod operations;
mod types;
mod stack;
mod variables;

fn main() -> Result<(), String> {
    let mut variables = variables::Variables::new();
    let mut stack = stack::Stack::new();
    if let Ok(text) = fs::read_to_string("./file.txt") {
        let replaced = text.replace("\n", " ");
        let block = Value::Block(replaced.split_whitespace().map(|s| s.to_string()).collect());
        block.exec(&mut stack, &mut variables)?;
    }
    Ok(())
}