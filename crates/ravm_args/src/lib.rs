use clap::Parser;

#[derive(Parser)]
#[command(version, name = "Ravm", about = "Ra's frontend")]
pub struct Arg {
    pub files: Vec<String>,
}
