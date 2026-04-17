use anyhow::Result;
use ra_file::{OpCode, Value};

mod double_pop;
mod systemcall;

pub fn runner(cmds: Vec<OpCode>) -> Result<()> {
    let mut stack: Vec<Value> = vec![];
    let mut cmd_number: u32 = 0;

    for cmd in cmds {
        cmd_number += 1;
        match cmd {
            OpCode::Push(value) => {
                stack.push(value.clone());
            }
            OpCode::Syscall => {
                let (first_value, second_value) =
                    double_pop::double_pop_number(&mut stack, cmd_number)?;
                systemcall::systemcall(&mut stack, first_value, second_value, cmd_number)?;
            }
            OpCode::Add => {
                if let Ok((value_1, value_2)) =
                    double_pop::double_pop_number(&mut stack, cmd_number)
                {
                    stack.push(Value::Number(value_1 + value_2));
                } else {
                    let (value1, value2) = double_pop::double_pop_string(&mut stack, cmd_number)?;
                    stack.push(Value::String(value1 + value2.as_str()));
                };
            }
            OpCode::Null => (),
        }
    }
    Ok(())
}
