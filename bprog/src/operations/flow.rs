use crate::{stack, types::{convert, Value}, variables};

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

pub fn read_block<'a, I>(iter: &mut I) -> Result<Value, String>
where
    I: Iterator<Item = &'a String>,
{
    let mut input = Vec::new();
    let mut depth = 1;
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


pub fn if_block<'a, I>(iter: &mut I, stack: &mut stack::Stack, variables: &mut variables::Variables) -> Result<(), String>
where
    I: Iterator<Item = &'a String>,
{
    println!("{:?}", stack);
    let val = stack.pop()?;
    let block_true = match iter.next(){
        Some(token) if token == "{" => read_block(iter)?,
        Some(token) => Value::Block(vec!(token.to_string())),
        None => Value::Block(Vec::new()),
    };

    println!("{:?}", block_true);

    let block_false = match iter.next(){
        Some(token) if token == "{" => read_block(iter)?,
        Some(token) => Value::Block(vec!(token.to_string())),
        None => Value::Block(Vec::new()),
    };

    println!("{:?}", block_false);

    let condition = match val {
        Value::Boolean(value) => value,
        _ => return Err(format!("Invalid syntax not a boolean")),
    };

    let block_to_run = if condition { block_true } else { block_false };
    block_to_run.exec(stack, variables).unwrap();
    Ok(())
}

pub fn times<'a, I>(iter: &mut I, stack: &mut stack::Stack, variables: &mut variables::Variables) -> Result<(), String>
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
        let times = stack.pop().unwrap();
        let mut times = match times {
            Value::Int(value) => value,
            _ => return Err(format!("Invalid syntax not an integer")),
        };
        loop {
            block.exec(stack, variables).unwrap();
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

pub fn r#loop<'a, I>(iter: &mut I, stack: &mut stack::Stack, variables: &mut variables::Variables) -> Result<(), String>
where
    I: Iterator<Item = &'a String>,
{
    let check: Value;
    let block: Value;
    if let Some(token) = iter.next() {
        if token == "{" {
            check = read_block(iter)?;
        } else {
            check = Value::Block(vec!(token.to_string()));
        }
    } else {
        return Err(format!("Invalid syntax"));
    }
    if let Some(token) = iter.next() {
        if token == "{" {
            block = read_block(iter)?;
        } else {
            block = Value::Block(vec!(token.to_string()));
        }
    } else {
        return Err(format!("Invalid syntax"));
    }
    loop {
        check.exec(stack, variables).unwrap();
        match stack.pop().unwrap() {
            Value::Boolean(true) => { break },
            Value::Boolean(false) => {
                block.exec(stack, variables).unwrap();
            },
            _ => return Err(format!("Invalid syntax")),
        }
    }
    Ok(())
}

pub fn assign(stack: &mut stack::Stack, variables: &mut variables::Variables) -> Result<(), String> {
    let value = stack.pop().unwrap();
    let name = stack.pop().unwrap();
    match name {
        Value::Symbol(name) => {
            variables.set(&name, value);
        },
        _ => return Err(format!("Invalid syntax not a symbol")),
    }
    Ok(())
}