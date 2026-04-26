use crate::read;
use anyhow::{Result, anyhow};
use ra_file::OpCode;

pub fn import(lines: Vec<&str>, cmds: &mut Vec<OpCode>, line_number: &mut usize) -> Result<()> {
    // let file = File::open(lines.get(1).ok_or(anyhow!("No file, at {}", line_number))?)?;
    // let reader = BufReader::new(file);
    // let mut cmds_ = vec![];
    // for line in reader.lines() {
    //     let line = line?;
    //     let lines: Vec<&str> = line.split(' ').collect();
    //     read_cmds(lines, &mut cmds_, line_number)?;
    // }
    let mut cmds_ = vec![];
    let mut skip_buffer = String::new();
    let mut is_skiping = false;
    read::read(
        lines
            .get(1)
            .ok_or(anyhow!("No file, at {}", line_number))?
            .to_string(),
        &mut cmds_,
        line_number,
        &mut skip_buffer,
        &mut is_skiping,
    )?;
    cmds.splice(cmds.len()..cmds.len(), cmds_);
    Ok(())
}
