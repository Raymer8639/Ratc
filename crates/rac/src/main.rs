use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;
use clap::Parser;
use rac_args::Arg;
use rustyline::DefaultEditor;

fn main() -> Result<()> {
    let arg = Arg::parse();

    let output_name = arg.output_name.unwrap_or_default();

    let mut line_number: usize = 0;

    if arg.files.is_empty() {
        let mut rl = DefaultEditor::new()?;
        let mut cmds = vec![];

        loop {
            line_number += 1;
            let readline = rl.readline(" >> ");
            match readline {
                Ok(line) => {
                    if line == "____exit____" {
                        break;
                    }
                    let lines: Vec<&str> = line.split(' ').collect();
                    rac_reader::read_cmds(lines.clone(), &mut cmds, &mut line_number)?;
                }
                Err(rustyline::error::ReadlineError::Interrupted) => break,
                Err(rustyline::error::ReadlineError::Eof) => break,
                Err(err) => return Err(err.into()),
            }
        }

        rac_writer::writer(&cmds, output_name)?;
    } else {
        for file in arg.files {
            let file = File::open(file)?;
            let reader = BufReader::new(file);
            let mut cmds = vec![];
            for line in reader.lines() {
                line_number += 1;
                let line = line?;
                let line: Vec<&str> = line.split(' ').collect();
                rac_reader::read_cmds(line.clone(), &mut cmds, &mut line_number)?;
            }
            rac_writer::writer(&cmds, output_name.clone())?;
        }
    }
    Ok(())
}
