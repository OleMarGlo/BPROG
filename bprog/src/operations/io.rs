use std::io;

use crate::{stack::Stack, types::Value, variables};

pub fn print(stack: &mut Stack, variables: &mut variables::Variables) -> Result<(), String> {
    let value = stack.pop().unwrap();
    print!("{}", value.to_string_with_variables(variables));
    Ok(())
}

pub fn println(stack: &mut Stack, variables: &mut variables::Variables) -> Result<(), String> {
    let value = stack.pop().unwrap();
    println!("{}", value.to_string_with_variables(variables));
    Ok(())
}

pub fn read(stack: &mut Stack) -> Result<(), String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    stack.push(Value::String(input.trim().to_string()));
    Ok(())
}