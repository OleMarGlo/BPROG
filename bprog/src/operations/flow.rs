/**
 * This module contains functions that are used to execute or read code blocks.
 * It contains functions for reading strings, lists, and blocks.
 */

use crate::{functions, stack, types::{convert, Value}, variables};

/**
 * This function reads a string from an iterator.
 * It takes an iterator and returns a Value.
 * It reads a string until it encounters a double quote.
 */
pub fn read_string<'a, I>(iter: &mut I) -> Value
where
    I: Iterator<Item = &'a String>,
{
    let mut input = String::new();
    while let Some(token) = iter.next() {
        if token == "\"" {
            break;
        }
        input.push_str(token);
        input.push(' ');
    }
    Value::String(input.trim().to_string())
}

/**
 * This function reads a list from an iterator.
 * It takes an iterator and returns a Value.
 * It reads a list until it encounters a closing square bracket.
 * It can read strings, lists and blocks inside the list. 
 */
pub fn read_list<'a, I>(iter: &mut I) -> Result<Value, String>
where
    I: Iterator<Item = &'a String>,
{
    let mut input: Vec<Value> = Vec::new();
    while let Some(token) = iter.next() {
        match token.as_str() {
            "]" => break,
            "\"" => {
                input.push(read_string(iter));
            },
            "[" => {
                input.push(read_list(iter)?);
            },
            "{" => {
                input.push(read_block(iter)?);
            },
            _ => {
                input.push(convert(token));
            }
        }

    }
    Ok(Value::from(input))
}

/**
 * This function reads a block from an iterator.
 * It takes an iterator and returns a Value.
 * It reads a block until it encounters a closing curly bracket.
 * It can read double blocks inside the block. this is useful if an if block is inside an if block
 */
pub fn read_block<'a, I>(iter: &mut I) -> Result<Value, String>
where
    I: Iterator<Item = &'a String>,     // I is an iterator that yields &str continues from exec
{
    let mut input = Vec::new();
    let mut depth = 1;           // depth is used to keep track of the number of curly brackets, it has alredy read the first one
    while let Some(token) = iter.next() {
        match token.as_str() {
            "{" => {
                depth += 1;
                input.push(token.to_string());
            },
            "}" => {
                depth -= 1;
                if depth == 0 {
                    break;
                } else {
                    input.push(token.to_string());
                }
            },
            _ => {
                input.push(token.to_string());
            }
        }
    }
    Ok(Value::Block(input))
}


pub fn if_block<'a, I>(iter: &mut I, stack: &mut stack::Stack, variables: &mut variables::Variables, functions: &mut functions::Functions) -> Result<(), String>
where
    I: Iterator<Item = &'a String>,
{
    let val = stack.pop()?;
    let block_true = match iter.next(){     // read the true block
        Some(token) if token == "{" => read_block(iter)?,
        Some(token) => Value::Block(vec!(token.to_string())),
        None => Value::Block(Vec::new()),
    };

    let block_false = match iter.next(){    // read the false block
        Some(token) if token == "{" => read_block(iter)?,
        Some(token) => Value::Block(vec!(token.to_string())),
        None => Value::Block(Vec::new()),
    };

    let condition = match val {             // make sure the value is a boolean
        Value::Boolean(value) => value,
        _ => return Err(format!("Invalid syntax not a boolean")),
    };

    // if the condition is true run the true block, otherwise run the false block
    let block_to_run = if condition { block_true } else { block_false };    
    block_to_run.exec(stack, variables, functions)?;
    Ok(())
}

/**
 * This function executes a block a specified number of times.
 * It takes a stack, a block, a variables object, and a functions object as arguments.
 * It reads the number of times to execute the block from the iterator.
 * If the value is not a list or block, it will return an error.
 */
pub fn times<'a, I>(iter: &mut I, stack: &mut stack::Stack, variables: &mut variables::Variables, functions: &mut functions::Functions) -> Result<(), String>
where
    I: Iterator<Item = &'a String>,
{
    let mut block = Value::Block(Vec::new());
    if let Some(token) = iter.next() {
        if token == "{" {
            block = read_block(iter)?;
        } else {
            block = Value::Block(vec!(token.to_string()));
        }
        let times = stack.pop()?;
        let mut times = match times {
            Value::Int(value) => value,
            _ => return Err(format!("Invalid syntax not an integer")),
        };
        loop {
            block.exec(stack, variables, functions)?;
            times -= 1;
            if times == 0 {
                break;
            }
        }
    } else {
        return Err(format!("Invalid syntax"));
    }
    Ok(())
}

/**
 * This function executes a block until a condition is met.
 * It takes a stack, a block, a variables object, and a functions object as arguments.
 * It reads the condition from the iterator.
 * If the value is not a list or block, it will return an error.
 */
pub fn r#loop<'a, I>(iter: &mut I, stack: &mut stack::Stack, variables: &mut variables::Variables, functions: &mut functions::Functions) -> Result<(), String>
where
    I: Iterator<Item = &'a String>,
{
    let check: Value;
    let block: Value;
    if let Some(token) = iter.next() {  // read the check block
        if token == "{" {
            check = read_block(iter)?;
        } else {
            check = Value::Block(vec!(token.to_string()));
        }
    } else {
        return Err(format!("Invalid syntax"));
    }
    if let Some(token) = iter.next() {  // read the block to execute if the condition is met
        if token == "{" {
            block = read_block(iter)?;
        } else {
            block = Value::Block(vec!(token.to_string()));
        }
    } else {
        return Err(format!("Invalid syntax"));
    }
    loop {
        check.exec(stack, variables, functions)?;    // execute the check block
        match stack.pop()? {                         // check if the result is true or false
            Value::Boolean(true) => { break },       // if true break out of the loop
            Value::Boolean(false) => {
                block.exec(stack, variables, functions)?;    // if false execute the block
            },
            _ => return Err(format!("Invalid syntax")),
        }
    }
    Ok(())
}

/**
 * This function assigns a value to a variable.
 * It takes a stack, a variables object, and a functions object as arguments.
 * It reads the variable name from the stack.
 * If the value is not a symbol, it will return an error.
 */
pub fn assign(stack: &mut stack::Stack, variables: &mut variables::Variables) -> Result<(), String> {
    let value = stack.pop()?;
    let name = stack.pop()?;
    match name {
        Value::Symbol(name) => {
            variables.set(&name, value);
        },
        _ => return Err(format!("Invalid syntax not a symbol")),
    }
    Ok(())
}

/**
 * This function creates a new function.
 * It takes a stack, a variables object, and a functions object as arguments.
 * It reads the function name from the stack.
 * If the value is not a symbol, it will return an error.
 */
pub fn new_function(stack: &mut stack::Stack, functions: &mut functions::Functions) -> Result<(), String> {
    let block = stack.pop()?;
    let name = stack.pop()?;
    match name {
        Value::Symbol(name) => {
            functions.set(&name, block);
        },
        _ => return Err(format!("Invalid syntax not a symbol")),
    }
    Ok(())
}