use clap::{crate_authors, crate_name, crate_version, ArgAction, Parser};
use std::process;

mod latex;

#[derive(Parser)]
#[command(
    name = crate_name!(),
    author = crate_authors!(", "),
    version = crate_version!(),
)]
/// Extract archive in memory and get its contents' hash(es)
struct Cli {
    /// LaTeX file
    #[arg(
        action = ArgAction::Set,
        num_args = 0..=1,
        value_name = "latex file",
        default_value = "document.tex",
    )]
    latex_file: String,
}

fn main() {
    let cli = Cli::parse();
    let latex_file = &cli.latex_file;

    for i in latex::gather_citations(latex_file) {
        println!("{i:?}");
    }

    process::exit(0);
}
