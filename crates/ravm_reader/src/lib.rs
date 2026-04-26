use std::{
    fs::File,
    io::{BufReader, Read},
};

use anyhow::{Result, anyhow};
use ra_file::{BytecodeFile, OpCode};

pub fn reader(file: String) -> Result<Vec<OpCode>> {
    let file = File::open(file)?;
    let mut reader = BufReader::new(file);
    let mut bytes = vec![];
    reader.read_to_end(&mut bytes)?;

    let file: BytecodeFile = postcard::from_bytes(&bytes)?;
    if ra_version::RUNNER_VERSION != file.version {
        return Err(anyhow!(
            "The VM's version is too old or too new! The exe's version is {}",
            file.version.clone()
        ));
    }
    Ok(file.lines)
}
