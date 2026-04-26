use anyhow::Result;
use clap::Parser;
use rac_args::Arg;
use rustyline::DefaultEditor;

fn main() -> Result<()> {
    let arg = Arg::parse();

    let output_name = arg.output_name.unwrap_or_default();

    let mut skip_buffer = String::new();
    let mut is_skiping = false;

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

                    if !line.ends_with(';') || is_skiping {
                        skip_buffer.push_str(line.as_str());
                        line_number -= 1;

                        is_skiping = !line.ends_with(';');

                        continue;
                    }
                    let line = skip_buffer.clone() + line.as_str();
                    skip_buffer.clear();
                    let commands_vec: Vec<&str> = line.split(';').collect();
                    rac_reader::read_cmds(commands_vec.clone(), &mut cmds, &mut line_number)?;
                }
                Err(rustyline::error::ReadlineError::Interrupted) => break,
                Err(rustyline::error::ReadlineError::Eof) => break,
                Err(err) => return Err(err.into()),
            }
        }

        rac_writer::writer(&cmds, output_name)?;
    } else {
        for file in arg.files {
            let mut cmds = vec![];
            rac_reader::read::read(
                file,
                &mut cmds,
                &mut line_number,
                &mut skip_buffer,
                &mut is_skiping,
            )?;
            rac_writer::writer(&cmds, output_name.clone())?;
        }
    }
    Ok(())
}
