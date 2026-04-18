use anyhow::{Result, anyhow};
use ra_file::Value;

pub fn double_pop_number(stack: &mut Vec<Value>, cmd_number: u32) -> Result<(i32, i32)> {
    let (mut first_value, mut second_value) = (-1, -1);
    if let Some(top) = stack.pop() {
        let _top = top.clone();
        let top = if let Value::Number(value) = _top {
            value
        } else {
            stack.push(top);
            return Err(anyhow!("Value type error! At: {cmd_number}"));
        };

        second_value = top;
    }
    if second_value == -1 {
        return Err(anyhow!("No value! At: {cmd_number}"));
    }
    if let Some(top) = stack.pop() {
        let _top = top.clone();
        let top = if let Value::Number(value) = _top {
            value
        } else {
            stack.push(top);
            return Err(anyhow!("Value type error!. At: {cmd_number}"));
        };
        first_value = top;
    }
    Ok((first_value, second_value))
}

pub fn double_pop_string(stack: &mut Vec<Value>, line_number: u32) -> Result<(String, String)> {
    let (mut first_value, mut second_value) = (String::new(), String::new());
    if let Some(top) = stack.pop() {
        let _top = top.clone();
        let top = if let Value::String(value) = _top {
            value
        } else {
            stack.push(top);
            return Err(anyhow!("Value type error! At: {line_number}"));
        };

        second_value = top;
    }
    if second_value.is_empty() {
        return Err(anyhow!("No value! At: {line_number}"));
    }
    if let Some(top) = stack.pop() {
        let _top = top.clone();
        let top = if let Value::String(value) = _top {
            value
        } else {
            stack.push(top);
            return Err(anyhow!("Value type error! At: {line_number}"));
        };
        first_value = top;
    }
    Ok((first_value, second_value))
}
