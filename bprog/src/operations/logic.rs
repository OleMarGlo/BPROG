use crate::stack::Stack;

pub fn and(stack: &mut Stack) -> Result<(), String> {
    let b = stack.pop()?;
    let a = stack.pop()?;
    stack.push(a.and(b)?);
    Ok(())
}

pub fn or(stack: &mut Stack) -> Result<(), String> {
    let b = stack.pop()?;
    let a = stack.pop()?;
    stack.push(a.or(b)?);
    Ok(())
}

pub fn not(stack: &mut Stack) -> Result<(), String> {
    let a = stack.pop()?;
    stack.push(!a);
    Ok(())
}