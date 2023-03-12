mod tidy;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "TidyUp", author = "Ali Koca. <kinetixcicocuk@gmail.com>", version = "0.1", about = "Tidying up your folders")]
struct Args{
    #[arg(short, long)]
    folder: String,
}

fn main() {
    let args = Args::parse();
    tidy::tidy_up(&args.folder);
}