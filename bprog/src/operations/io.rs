use std::io;

use crate::{types::Value, stack::Stack};

pub fn print(stack: &mut Stack) -> Result<(), String> {
    let value = stack.pop().unwrap();
    println!("{}", value);
    Ok(())
}

pub fn read(stack: &mut Stack) -> Result<(), String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    stack.push(Value::String(input.trim().to_string()));
    Ok(())
}