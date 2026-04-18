use clap::Parser;

#[derive(Parser)]
#[command(version, name = "Rac", about = "Ra's backend")]
pub struct Arg {
    pub files: Vec<String>,
    #[arg(short, long)]
    pub output_name: Option<String>,
}
