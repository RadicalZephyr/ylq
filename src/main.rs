use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "ylq",
    about = "A data query and process tool supporting multiple data serializiation formats."
)]
struct Options {
    #[structopt(short, long)]
    in_file: Option<PathBuf>,

    #[structopt(short, long)]
    out_file: Option<PathBuf>,

    #[structopt(short, long)]
    read_format: Option<String>,

    #[structopt(short, long)]
    write_format: Option<String>,

    #[structopt]
    program: String,
}

fn main() {
    let options = Options::from_args();
    println!("{:?}", options);
}
