use clap::Parser;

#[derive(Parser)]
pub struct Arg {
    pub files: Vec<String>,
}
