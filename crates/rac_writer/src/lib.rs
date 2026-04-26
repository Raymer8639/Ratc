use anyhow::Result;
use ra_file::{BytecodeFile, OpCode};
use std::{
    fs::File,
    io::{BufWriter, Write},
};

pub fn writer(commands: &[OpCode], output_name: String) -> Result<()> {
    let output_name = match output_name.as_str() {
        "" => "___output___",
        _ => output_name.as_str(),
    };
    let bin = BytecodeFile {
        lines: commands.to_vec(),
        version: ra_version::COMPILER_VERSION.to_string(),
    };
    let file = File::create(output_name)?;
    let bytes = postcard::to_allocvec(&bin)?;
    let mut writer = BufWriter::new(file);
    writer.write_all(&bytes)?;
    writer.flush()?;
    Ok(())
}
