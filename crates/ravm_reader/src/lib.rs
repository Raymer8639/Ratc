use std::{fs::File, io::Read};

use anyhow::{Result, anyhow};
use ra_file::{ArchivedBytecodeFile, BytecodeFile, OpCode};

pub fn reader(file: String) -> Result<Vec<OpCode>> {
    let mut file = File::open(file)?;
    let mut bytes = vec![];
    file.read_to_end(&mut bytes)?;
    let file = rkyv::access::<ArchivedBytecodeFile, rkyv::rancor::Error>(&bytes)?;
    let file: BytecodeFile = rkyv::deserialize::<BytecodeFile, rkyv::rancor::Error>(file)?;

    if ra_version::RUNNNER_VERSION != file.version {
        return Err(anyhow!(
            "The VM's version is too old or too new! The exe's version is {}",
            file.version.clone()
        ));
    }
    Ok(file.lines)
}
