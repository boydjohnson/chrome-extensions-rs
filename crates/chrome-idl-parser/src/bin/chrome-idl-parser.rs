use {
    chrome_idl_parser::{generate, json::ChromeApi},
    clap::Parser,
    std::{
        io::{stdin, stdout},
        path::{Path, PathBuf},
    },
};

fn main() {
    let args = Args::parse();

    let from = PathBuf::from(&args.from);
    let to = PathBuf::from(&args.to);

    generate(&from, &to).unwrap();
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(help = "The path to the input directory with chrome json and idl files")]
    from: String,
    #[arg(help = "The file path to the output file.")]
    to: String,
}
