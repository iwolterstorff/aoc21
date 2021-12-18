mod days;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about, version)]
struct Args {
    #[clap(short, long)]
    day: u32,

    #[clap(short, long)]
    puzzle: u32,
}

fn main() {
    let args = Args::parse();
}
