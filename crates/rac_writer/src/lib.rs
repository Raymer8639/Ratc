use anyhow::Result;
use ra_file::{BytecodeFile, OpCode};
use std::{
    fs::File,
    io::{BufWriter},
};

pub fn writer(commands: &[OpCode]) -> Result<()> {
    let file = File::create("./___out___")?;
    let mut writer = BufWriter::new(file);
    bincode::serialize_into(
        &mut writer,
        &BytecodeFile {
            lines: commands.to_vec(),
            version: ra_version::COMPILER_VERSION.to_string(),
        },
    )?;
    Ok(())
}
