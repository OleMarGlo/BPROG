use crate::types::{convert, Value};

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
    while let Some(token) = iter.next() {
        if token == "}" {
            break;
        }
        input.push(token.to_string());
    }
    Ok(Value::Block(input))
}
