use anyhow::{Result, anyhow};
use ra_file::OpCode;

mod import;
mod push;

pub fn read_cmds(lines: Vec<&str>, cmds: &mut Vec<OpCode>, line_number: &mut usize) -> Result<()> {
    match *lines.first().ok_or(anyhow!("?"))? {
        "push" => {
            push::push(lines, cmds)?;
        }
        "import" => {
            import::import(lines, cmds, line_number)?;
        }
        "syscall" => cmds.push(OpCode::Syscall),
        "add" => cmds.push(OpCode::Add),
        _ => cmds.push(OpCode::Null),
    }
    Ok(())
}
