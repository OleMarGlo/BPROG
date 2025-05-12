use std::fmt::Display;

use crate::types::Value;

/**
 * Since rust does not have a built-in stack data structure, we need to implement our own.
 * The Stack struct is a wrapper around a vector of Value.
 */
#[derive(Debug)]
pub struct Stack {
    stack: Vec<Value>,
}

impl Stack { 
    pub fn new() -> Self {
        Stack {
            stack: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Result<Value, String> {
        self.stack.pop().ok_or("Stack is empty".to_string())
    }


    /**
     * This function swaps the top two elements on the stack.
     * If the stack is empty, it will return an error.
     */
    pub fn swap(&mut self) -> Result<(), String> {
        let a = self.pop()?;
        let b = self.pop()?;
        self.stack.push(a);
        self.stack.push(b);
        Ok(())
    }

    /**
     * This function duplicates the top element on the stack.
     * If the stack is empty, it will return an error.
     */
    pub fn dup(&mut self) -> Result<(), String> {
        let top = self.pop()?;
        self.stack.push(top.clone());
        self.stack.push(top);
        Ok(())
    }
}

// Implement Display trait for Stack to print the stack
impl Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner = self.stack.iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        write!(f, "[{}]", inner)
    }
}