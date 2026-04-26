use anyhow::{Result, anyhow};
use ra_file::OpCode;

mod import;
mod push;
pub mod read;

pub fn read_cmds(
    mut commands_vec: Vec<&str>,
    cmds: &mut Vec<OpCode>,
    line_number: &mut usize,
) -> Result<()> {
    commands_vec.pop();
    for command in commands_vec {
        let command_vec: Vec<&str> = command.split(' ').collect();

        match *command_vec.first().ok_or(anyhow!("?"))? {
            "push" => {
                push::push(command_vec.clone(), cmds)?;
            }
            "import" => {
                import::import(command_vec.clone(), cmds, line_number)?;
            }
            "{" => {}
            "}" => {}
            "syscall" => cmds.push(OpCode::Syscall),
            "add" => cmds.push(OpCode::Add),
            "rm" => cmds.push(OpCode::Rm),
            _ => cmds.push(OpCode::Null),
        }
    }

    Ok(())
}
