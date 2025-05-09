use std::fmt::Display;

use crate::types::Value;

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

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Result<Value, String> {
        self.stack.pop().ok_or("Stack is empty".to_string())
    }

    pub fn swap(&mut self) -> Result<(), String> {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push(a);
        self.stack.push(b);
        Ok(())
    }
    pub fn dup(&mut self) -> Result<(), String> {
        let top = self.stack.pop().unwrap();
        self.stack.push(top.clone());
        self.stack.push(top);
        Ok(())
    }
}

impl Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner = self.stack.iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        write!(f, "[{}]", inner)
    }
}