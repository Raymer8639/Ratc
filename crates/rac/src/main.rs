use anyhow::{Result, anyhow};
use clap::Parser;
use ra_file::OpCode;
use rac_args::Arg;
use rustyline::DefaultEditor;

fn main() -> Result<()> {
    let arg = Arg::parse();
    if arg.files.is_empty() {
        let mut rl = DefaultEditor::new()?;
        let mut cmds = vec![];
        loop {
            let readline = rl.readline(" >> ");
            match readline {
                Ok(line) => {
                    let lines: Vec<&str> = line.split(' ').collect();
                    match *lines.first().ok_or(anyhow!("?"))? {
                        "push" => cmds.push(OpCode::Push(
                            lines.get(1).ok_or(anyhow!("No args!"))?.parse()?,
                        )),
                        "add" => cmds.push(OpCode::Add),
                        _ => {}
                    }
                }
                Err(rustyline::error::ReadlineError::Interrupted) => break,
                Err(rustyline::error::ReadlineError::Eof) => break,
                Err(err) => return Err(err.into()),
            }
        }
        dbg!(cmds.clone());
        rac_writer::writer(&cmds)?;
    } else {
    }
    Ok(())
}
