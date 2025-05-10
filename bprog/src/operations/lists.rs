use crate::{stack::Stack, types::Value, variables};

use super::flow::read_block;

pub fn head(stack: &mut Stack) -> Result<(), String> {
    let top = stack.pop().unwrap();
    let value = Value::head(&top).unwrap();
    stack.push(value);
    Ok(())
}

pub fn tail(stack: &mut Stack) -> Result<(), String> {
    let top = stack.pop().unwrap();
    let value = Value::tail(&top).unwrap();
    stack.push(value);
    Ok(())
}

pub fn empty(stack: &mut Stack) -> Result<(), String> {
    let top = stack.pop().unwrap();
    let value = Value::empty(&top).unwrap();
    stack.push(value);
    Ok(())
}

pub fn lenght(stack: &mut Stack) -> Result<(), String> {
    let top = stack.pop().unwrap();
    let value = Value::lenght(&top).unwrap();
    stack.push(value);
    Ok(())
}

pub fn cons(stack: &mut Stack) -> Result<(), String> {
    let b = stack.pop().unwrap();
    let value = stack.pop().unwrap();
    match b {
        Value::List(mut list) => {
            Value::cons(&mut list, value)?;
            stack.push(Value::List(list));
        },
        _ => return Err(format!("Invalid operation")),
    }
    Ok(())
}

pub fn append(stack: &mut Stack) -> Result<(), String> {
    let b = stack.pop().unwrap();
    let a = stack.pop().unwrap();
    match (a, b) {
        (Value::List(mut a), Value::List(b)) => {
            Value::append(&mut a, b)?;
            stack.push(Value::List(a));
        },
        _ => return Err(format!("Invalid operation")),
    }
    Ok(())
}

pub fn each<'a, I>(iter: &mut I, stack: &mut Stack, variables: &mut variables::Variables) -> Result<(), String>
where
    I: Iterator<Item = &'a String>,
{
    let list = stack.pop().unwrap();
    match list {
        Value::List(_) => {
            if let Some(token) = iter.next() {
                if token == "{" {
                    let block = read_block(iter)?;
                    list.each(stack, block, variables)?;
                    Ok(())
                } else {
                    list.each(stack, Value::Block(vec!(token.to_string())), variables)?;
                    Ok(())
                }
            } else {
                Err(format!("Invalid operation"))
            }
        }
        _ => return Err(format!("Invalid operation")),
    }    
}

pub fn map<'a, I>(iter: &mut I, stack: &mut Stack, variables: &mut variables::Variables) -> Result<(), String>
where
    I: Iterator<Item = &'a String>,
{
    let list = stack.pop().unwrap();
    match list {
        Value::List(_) => {
            if let Some(token) = iter.next() {
                if token == "{" {
                    let block = read_block(iter)?;
                    list.map(stack, block, variables)?;
                    Ok(())
                } else {
                    list.map(stack, Value::Block(vec!(token.to_string())), variables)?;
                    Ok(())
                }
            } else {
                Err(format!("Invalid operation"))
            }
        }
        _ => return Err(format!("Invalid operation")),
    }    
}

pub fn foldl<'a, I>(iter: &mut I, stack: &mut Stack, variables: &mut variables::Variables) -> Result<(), String>
where
    I: Iterator<Item = &'a String>,
{
    let start_value = stack.pop().unwrap();
    let list = stack.pop().unwrap();
    match list {
        Value::List(_) => {
            if let Some(token) = iter.next() {
                if token == "{" {
                    let block = read_block(iter)?;
                    list.foldl(stack, block, start_value, variables)?;
                    Ok(())
                } else {
                    list.foldl(stack, Value::Block(vec!(token.to_string())), start_value, variables)?;
                    Ok(())
                }
            } else {
                Err(format!("Invalid operation"))
            }
        }
        _ => return Err(format!("Invalid operation")),
    }    
}