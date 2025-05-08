use std::fs;

use types::Value;

mod interpreter;
mod operations;
mod types;
mod stack;

fn main() -> Result<(), String> {
    if let Ok(text) = fs::read_to_string("./file.txt") {
        let replaced = text.replace("\n", " ");
        let block = Value::Block(replaced.split_whitespace().map(|s| s.to_string()).collect());
        block.exec()?;
    }
    Ok(())
}