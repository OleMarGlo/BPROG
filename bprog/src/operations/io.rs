/**
 * This module contains functions that perform input and output operations.
 * It contains functions for reading and printing values, as well as reading a line of text.
 */
use std::io;

use crate::{stack::Stack, types::Value, variables};

/**
 * This function prints a value to the standard output.
 * It takes a stack and a variables object as arguments.
 */
pub fn print(stack: &mut Stack, variables: &mut variables::Variables) -> Result<(), String> {
    let value = stack.pop()?;
    print!("{}", value.to_string_with_variables(variables));
    Ok(())
}

/**
 * This function prints a value to the standard output and a newline.
 * It takes a stack and a variables object as arguments.
 */
pub fn println(stack: &mut Stack, variables: &mut variables::Variables) -> Result<(), String> {
    let value = stack.pop()?;
    println!("{}", value.to_string_with_variables(variables));
    Ok(())
}

/**
 * This function reads a line of text from the standard input and pushes it to the stack.
 * It takes a stack and a variables object as arguments.
 */
pub fn read(stack: &mut Stack) -> Result<(), String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    stack.push(Value::String(input.trim().to_string()));
    Ok(())
}