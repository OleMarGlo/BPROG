/**
 * This module contains functions that perform logical operations on boolean values.
 * It contains functions for logical AND, OR, and NOT.
 */

use crate::stack::Stack;

/**
 * This function performs logical AND on two boolean values.
 * This is what is called when the && operator is used.
 * If errors occur it will pass it to the caller using ? operator.
 */
pub fn and(stack: &mut Stack) -> Result<(), String> {
    let b = stack.pop()?;
    let a = stack.pop()?;
    stack.push(a.and(b)?);
    Ok(())
}

/**
 * This function performs logical OR on two boolean values.
 * This is what is called when the || operator is used.
 * If errors occur it will pass it to the caller using ? operator.
 */
pub fn or(stack: &mut Stack) -> Result<(), String> {
    let b = stack.pop()?;
    let a = stack.pop()?;
    stack.push(a.or(b)?);
    Ok(())
}

/**
 * This function performs logical NOT on a boolean value.
 * This is what is called when the not operator is used.
 * If errors occur it will pass it to the caller using ? operator.  
 */
pub fn not(stack: &mut Stack) -> Result<(), String> {
    let a = stack.pop()?;
    stack.push(!a);
    Ok(())
}