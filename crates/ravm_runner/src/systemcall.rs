use anyhow::{Result, anyhow};
use ra_file::Value;

pub fn systemcall(
    stack: &mut Vec<Value>,
    first_value: i32,
    second_value: i32,
    cmd_number: u32,
) -> Result<()> {
    match (first_value, second_value) {
        (0, 0) => {
            let value = if let Some(value) = stack.pop() {
                value.to_string()
            } else {
                return Err(anyhow!(
                    "No value! In the {cmd_number}th(st, nd) instruction"
                ));
            };
            print!("{value}");
        }
        (_, _) => {
            return Err(anyhow!(
                "Invaild systemcall! In the {cmd_number}th(st, nd) instruction"
            ));
        }
    }
    Ok(())
}
