use crate::read_cmds;
use anyhow::{Result, anyhow};
use ra_file::OpCode;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};
pub fn import(lines: Vec<&str>, cmds: &mut Vec<OpCode>, line_number: &mut usize) -> Result<()> {
    let file = File::open(lines.get(1).ok_or(anyhow!("No file, at {}", line_number))?)?;
    let reader = BufReader::new(file);
    let mut cmds_ = vec![];
    for line in reader.lines() {
        let line = line?;
        let lines: Vec<&str> = line.split(' ').collect();
        read_cmds(lines, &mut cmds_, line_number)?;
    }
    cmds.splice(cmds.len()..cmds.len(), cmds_);
    Ok(())
}
