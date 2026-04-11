use clap::Parser;
use ravm_args::Arg;

fn main() {
    let arg = Arg::parse();
    for file in arg.files {
        ravm_reader::reader(file.clone());
    }
}
