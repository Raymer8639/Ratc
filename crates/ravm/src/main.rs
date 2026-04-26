use anyhow::Result;
use clap::Parser;
use ravm_args::Arg;

fn main() -> Result<()> {
    let arg = Arg::parse();
    for file in arg.files {
        let cmds = ravm_reader::reader(file.clone())?;
        ravm_runner::runner(cmds)?;
    }
    Ok(())
}
