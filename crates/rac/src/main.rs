use anyhow::Result;
use clap::Parser;
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
                    if line == "____exit____" {
                        break;
                    }
                    let lines: Vec<&str> = line.split(' ').collect();
                    rac_reader::read_cmds(lines.clone(), &mut cmds)?;
                }
                Err(rustyline::error::ReadlineError::Interrupted) => break,
                Err(rustyline::error::ReadlineError::Eof) => break,
                Err(err) => return Err(err.into()),
            }
        }

        rac_writer::writer(&cmds)?;
    } else {
        todo!();
    }
    Ok(())
}
