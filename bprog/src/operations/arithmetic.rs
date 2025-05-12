/**
 * This module contains functions that perform arithmetic operations on values.
 * It contains functions for addition, subtraction, multiplication, division, and integer division.
 */

use crate::{types::Value, stack::Stack};

pub fn add(stack: &mut Stack) -> Result<(), String> {
    let b = stack.pop()?;
    let a = stack.pop()?;
    stack.push((a+b)?);
    Ok(())
}

pub fn sub(stack: &mut Stack) -> Result<(), String> {
    let b = stack.pop()?;
    let a = stack.pop()?;
    stack.push((a-b)?);
    Ok(())
}

pub fn mul(stack: &mut Stack) -> Result<(), String> {
    let b = stack.pop()?;
    let a = stack.pop()?;
    stack.push((a*b)?);
    Ok(())
}

pub fn div(stack: &mut Stack) -> Result<(), String> {
    let b = stack.pop()?;
    let a = stack.pop()?;
    stack.push((a/b)?);
    Ok(())
}

pub fn lt(stack: &mut Stack) -> Result<(), String> {
    let b = stack.pop()?;
    let a = stack.pop()?;
    stack.push(Value::Boolean(a<b));
    Ok(())
}

pub fn gt(stack: &mut Stack) -> Result<(), String> {
    let b = stack.pop()?;
    let a = stack.pop()?;
    stack.push(Value::Boolean(a>b));
    Ok(())
}

pub fn eq(stack: &mut Stack) -> Result<(), String> {
    let b = stack.pop()?;
    let a = stack.pop()?;
    stack.push(Value::Boolean(a==b));
    Ok(())
}

pub fn int_div(stack: &mut Stack) -> Result<(), String> {
    let a = stack.pop()?;
    let b = stack.pop()?;
    stack.push(Value::Int((a/b)?.into()));
    Ok(())
}

