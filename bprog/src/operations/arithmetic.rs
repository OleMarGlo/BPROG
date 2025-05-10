
use crate::{types::Value, stack::Stack};

pub fn add(stack: &mut Stack) -> Result<(), String> {
    let b = stack.pop().unwrap();
    let a = stack.pop().unwrap();
    stack.push((a+b).unwrap());
    Ok(())
}

pub fn sub(stack: &mut Stack) -> Result<(), String> {
    let b = stack.pop().unwrap();
    let a = stack.pop().unwrap();
    stack.push((a-b).unwrap());
    Ok(())
}

pub fn mul(stack: &mut Stack) -> Result<(), String> {
    let b = stack.pop().unwrap();
    let a = stack.pop().unwrap();
    stack.push((a*b).unwrap());
    Ok(())
}

pub fn div(stack: &mut Stack) -> Result<(), String> {
    let b = stack.pop().unwrap();
    let a = stack.pop().unwrap();
    stack.push((a/b).unwrap());
    Ok(())
}

pub fn lt(stack: &mut Stack) -> Result<(), String> {
    let b = stack.pop().unwrap();
    let a = stack.pop().unwrap();
    stack.push(Value::Boolean(a<b));
    Ok(())
}

pub fn gt(stack: &mut Stack) -> Result<(), String> {
    let b = stack.pop().unwrap();
    let a = stack.pop().unwrap();
    stack.push(Value::Boolean(a>b));
    Ok(())
}

pub fn eq(stack: &mut Stack) -> Result<(), String> {
    let b = stack.pop().unwrap();
    let a = stack.pop().unwrap();
    stack.push(Value::Boolean(a==b));
    Ok(())
}

pub fn int_div(stack: &mut Stack) -> Result<(), String> {
    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();
    stack.push(Value::Int((a/b).unwrap().into()));
    Ok(())
}

