use {chrome_idl_parser::generate, clap::Parser, std::path::PathBuf};

fn main() {
    let args = Args::parse();

    let froms = args.from.iter().map(PathBuf::from).collect::<Vec<_>>();

    let to = PathBuf::from(&args.to);

    generate(&froms, &to).unwrap();
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(help = "The path to the input directory with chrome json and idl files")]
    from: Vec<String>,
    #[arg(short, long, help = "The file path to the src directory")]
    to: String,
}
