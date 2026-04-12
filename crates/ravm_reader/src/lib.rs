use std::{fs::File, io::BufReader};

use anyhow::{Result, anyhow};
use ra_file::BytecodeFile;

pub fn reader(file: String) -> Result<()> {
    let file = File::open(file)?;
    let file_reader = BufReader::new(file);
    let file: BytecodeFile = bincode::deserialize_from(file_reader)?;
    if ra_version::RUNNNER_VERSION != file.version {
        return Err(anyhow!(
            "The VM's version is too old or too new! The exe's version is {}",
            file.version.clone()
        ));
    }
    dbg!(file);

    Ok(())
}
