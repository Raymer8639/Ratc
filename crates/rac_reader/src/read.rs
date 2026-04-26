use anyhow::Result;
use ra_file::OpCode;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read(
    file: String,
    cmds: &mut Vec<OpCode>,
    line_number: &mut usize,
    skip_buffer: &mut String,
    is_skiping: &mut bool,
) -> Result<()> {
    let mut root_line = String::new();
    let file = File::open(file)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        *line_number += 1;
        root_line = line?;
        let line = root_line.clone();
        if line.ends_with(';') || *is_skiping {
            skip_buffer.push_str(line.as_str());
            *line_number -= 1;

            *is_skiping = !line.ends_with(';');

            continue;
        }

        let line = skip_buffer.clone() + line.as_str();
        skip_buffer.clear();
        let commands_vec: Vec<&str> = line.split(';').collect();

        crate::read_cmds(commands_vec.clone(), cmds, line_number)?;
    }
    if !skip_buffer.is_empty() {
        let line = root_line.clone() + (*skip_buffer).as_str();
        skip_buffer.clear();
        let commands_vec: Vec<&str> = line.split(';').collect();
        crate::read_cmds(commands_vec.clone(), cmds, line_number)?;
    }

    Ok(())
}
