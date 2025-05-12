/**
 * This module contains functions that perform list operations.
 * It contains functions for getting the head, tail, empty, length, and appending lists.
 */

use crate::{functions, stack::Stack, types::Value, variables};

use super::flow::read_block;

/**
 * This function returns the first element of a list or string.
 * It takes a stack and a variables object as arguments.
 * If the value is not a list or string, it will return an error.
 */
pub fn head(stack: &mut Stack) -> Result<(), String> {
    let top = stack.pop()?;
    let value = Value::head(&top)?;
    stack.push(value);
    Ok(())
}

/**
 * This function returns the tail of a list or string.
 * It takes a stack and a variables object as arguments.
 * If the value is not a list or string, it will return an error.
 */
pub fn tail(stack: &mut Stack) -> Result<(), String> {
    let top = stack.pop()?;
    let value = Value::tail(&top)?;
    stack.push(value);
    Ok(())
}

/**
 * This function returns true if the value is empty, false otherwise.
 * It takes a stack and a variables object as arguments.
 * If the value is not a list or string, it will return an error.
 */
pub fn empty(stack: &mut Stack) -> Result<(), String> {
    let top = stack.pop()?;
    let value = Value::empty(&top)?;
    stack.push(value);
    Ok(())
}

/**
 * This function returns the length of a list or string.
 * It takes a stack and a variables object as arguments.
 * If the value is not a list or string, it will return an error.
 */
pub fn lenght(stack: &mut Stack) -> Result<(), String> {
    let top = stack.pop()?;
    let value = Value::lenght(&top)?;
    stack.push(value);
    Ok(())
}

/**
 * This function adds a value to the beginning of a list.
 * It takes a stack and a variables object as arguments.
 * If the value is not a list, it will return an error.
 */
pub fn cons(stack: &mut Stack) -> Result<(), String> {
    let b = stack.pop()?;
    let value = stack.pop()?;
    match b {
        Value::List(mut list) => {
            Value::cons(&mut list, value)?;
            stack.push(Value::List(list));
        },
        _ => return Err(format!("Invalid operation")),
    }
    Ok(())
}

/**
 * This function appends one list to another.
 * It takes a stack and a variables object as arguments.
 * If the values are not lists, it will return an error.
 */
pub fn append(stack: &mut Stack) -> Result<(), String> {
    let b = stack.pop()?;
    let a = stack.pop()?;
    match (a, b) {
        (Value::List(mut a), Value::List(b)) => {
            Value::append(&mut a, b)?;
            stack.push(Value::List(a));
        },
        _ => return Err(format!("Invalid operation")),
    }
    Ok(())
}

/**
 * This function executes a block for each element in a list.
 * It takes a stack, a block, a variables object, and a functions object as arguments.
 * As the each can be given a code block after itself it has to be called with an iterator.
 * If the value is not a list or block, it will return an error.
 */
pub fn each<'a, I>(iter: &mut I, stack: &mut Stack, variables: &mut variables::Variables, functions: &mut functions::Functions) -> Result<(), String>
where
    I: Iterator<Item = &'a String>,
{
    let list = stack.pop()?;
    match list {
        Value::List(_) => {
            if let Some(token) = iter.next() {
                if token == "{" {
                    let block = read_block(iter)?;
                    list.each(stack, block, variables, functions)?;
                    Ok(())
                } else {
                    list.each(stack, Value::Block(vec!(token.to_string())), variables, functions)?;
                    Ok(())
                }
            } else {
                Err(format!("Invalid operation"))
            }
        }
        _ => return Err(format!("Invalid operation")),
    }    
}

/**
 * This function applies a block to each element in a list and returns a new list with the results.
 * It takes a stack, a block, a variables object, and a functions object as arguments.
 * As the map can be given a code block after itself it has to be called with an iterator.
 * If the value is not a list or block, it will return an error.
 */
pub fn map<'a, I>(iter: &mut I, stack: &mut Stack, variables: &mut variables::Variables, functions: &mut functions::Functions) -> Result<(), String>
where
    I: Iterator<Item = &'a String>,
{
    let list = stack.pop()?;
    match list {
        Value::List(_) => {
            if let Some(token) = iter.next() {
                if token == "{" {
                    let block = read_block(iter)?;
                    list.map(stack, block, variables, functions)?;
                    Ok(())
                } else {
                    list.map(stack, Value::Block(vec!(token.to_string())), variables, functions)?;
                    Ok(())
                }
            } else {
                Err(format!("Invalid operation"))
            }
        }
        _ => return Err(format!("Invalid operation")),
    }    
}

/**
 * This function applies a block to each element in a list and returns a single value by folding the elements from left to right.
 * It takes a stack, a block, a start value, a variables object, and a functions object as arguments.
 * As the foldl can be given a code block after itself it has to be called with an iterator.
 * If the value is not a list or block, it will return an error.
 */
pub fn foldl<'a, I>(iter: &mut I, stack: &mut Stack, variables: &mut variables::Variables, functions: &mut functions::Functions) -> Result<(), String>
where
    I: Iterator<Item = &'a String>,
{
    let start_value = stack.pop()?;
    let list = stack.pop()?;
    match list {
        Value::List(_) => {
            if let Some(token) = iter.next() {
                if token == "{" {
                    let block = read_block(iter)?;
                    list.foldl(stack, block, start_value, variables, functions)?;
                    Ok(())
                } else {
                    list.foldl(stack, Value::Block(vec!(token.to_string())), start_value, variables, functions)?;
                    Ok(())
                }
            } else {
                Err(format!("Invalid operation"))
            }
        }
        _ => return Err(format!("Invalid operation")),
    }    
}