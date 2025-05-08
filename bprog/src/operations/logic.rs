use crate::stack::Stack;

pub fn and(stack: &mut Stack) -> Result<(), String> {
    let b = stack.pop().unwrap();
    let a = stack.pop().unwrap();
    stack.push(a.and(b).unwrap());
    Ok(())
}

pub fn or(stack: &mut Stack) -> Result<(), String> {
    let b = stack.pop().unwrap();
    let a = stack.pop().unwrap();
    stack.push(a.or(b).unwrap());
    Ok(())
}

pub fn not(stack: &mut Stack) -> Result<(), String> {
    let a = stack.pop().unwrap();
    stack.push(!a);
    Ok(())
}