use clap::{
    Args,
    Parser,
    Subcommand,
};

#[derive(Debug, Parser)]
#[clap(name = "utils")]
#[clap(author, version, about = "Utilities for retro", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Rename(Rename),
}

#[derive(Args, Debug)]
struct Rename {
    #[clap(help = "Substring to rename")]
    rename: String,
    #[clap(help = "rename substring to this")]
    to: String,
    #[clap(long, parse(from_os_str), help = "path of files to be renamed")]
    from_path: Option<std::path::PathBuf>,
}

fn main() {
    let parsed = Cli::parse();
    println!("{parsed:#?}")
}
