use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Result, anyhow};
use ra_file::{OpCode, Value};
use regex::Regex;

pub fn read_cmds(lines: Vec<&str>, cmds: &mut Vec<OpCode>, line_number: &mut usize) -> Result<()> {
    match *lines.first().ok_or(anyhow!("?"))? {
        "push" => {
            let mut value = String::new();
            for str in lines.iter().skip(1) {
                value.push_str(str);
                value.push(' ');
            }
            value.pop();
            let value = if let Some(value) = unescape::unescape(&value) {
                value
            } else {
                value
            };
            let mut values = vec![];
            let re = Regex::new(r#""([^"\\]*(?:\\.[^"\\]*)*)"|(true|false)|([^\s]+)"#)?;
            for cap in re.captures_iter(&value) {
                if let Some(mat) = cap.get(1) {
                    values.push(Value::String(mat.as_str().to_string()));
                } else if let Some(mat) = cap.get(2) {
                    let b = mat.as_str() == "true";
                    values.push(Value::Bool(b));
                } else if let Some(mat) = cap.get(3) {
                    let s = mat.as_str();
                    if let Ok(num) = s.parse::<i32>() {
                        values.push(Value::Number(num));
                    } else {
                        values.push(Value::String(s.to_string()));
                    }
                }
            }
            for value in values {
                cmds.push(OpCode::Push(value));
            }
        }
        "import" => {
            let file = File::open(lines.get(1).ok_or(anyhow!("No file, at {}", line_number))?)?;
            let reader = BufReader::new(file);
            let mut cmds_ = vec![];
            for line in reader.lines() {
                let line = line?;
                let lines: Vec<&str> = line.split(' ').collect();
                read_cmds(lines, &mut cmds_, line_number)?;
            }
            cmds.splice(cmds.len()..cmds.len(), cmds_);
        }
        "syscall" => cmds.push(OpCode::Syscall),
        "add" => cmds.push(OpCode::Add),
        _ => cmds.push(OpCode::Null),
    }
    Ok(())
}
