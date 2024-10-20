use clap::{crate_authors, crate_name, crate_version, ArgAction, Parser};
use std::process;

mod bib;
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

    /// BibTeX file
    #[arg(
        action = ArgAction::Set,
        num_args = 0..=1,
        value_name = "bib file",
        default_value = "references.bib",
    )]
    bib_file: String,
}

fn main() {
    let cli = Cli::parse();
    let latex_file = &cli.latex_file;
    let bib_file = &cli.bib_file;

    // TODO: specify how many are unique?  Have some kind of container for citations?
    println!(
        "Found {} citations in {latex_file:?}",
        latex::gather_citations(latex_file).len()
    );
    println!(
        "Found {} entries in {bib_file:?}",
        bib::gather_bib_entries(bib_file).len()
    );

    process::exit(0);
}
