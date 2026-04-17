use clap::Parser;

#[derive(Parser)]
pub struct Arg {
    pub files: Vec<String>,
    #[arg(short, long)]
    pub output_name: Option<String>,
}
