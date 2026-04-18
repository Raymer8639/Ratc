use anyhow::Result;
use ra_file::{BytecodeFile, OpCode};
use std::{fs::File, io::Write};

pub fn writer(commands: &[OpCode], output_name: String) -> Result<()> {
    let output_name = match output_name.as_str() {
        "" => "___output___",
        _ => output_name.as_str(),
    };

    let mut file = File::create(output_name)?;
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&BytecodeFile {
        lines: commands.to_vec(),
        version: ra_version::COMPILER_VERSION.to_string(),
    })?;
    file.write_all(&bytes)?;

    Ok(())
}
