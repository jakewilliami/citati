use clap::{crate_authors, crate_name, crate_version, ArgAction, Args, Parser};
use std::process;

mod bib;
mod citations;
mod latex;
mod unused;

// TODO:
//   - Add --pages functionality
//   - Add --article functionality
//   - Add --collection functionality
//   - Add --book functionality
//   - Add --count functionality
//   - Add checker for journals to be capitalised appropriately
//   - Port to Rust
//   - Check no duplicate IDs
//   - TODO: check no "and others" in the authors
//   - Check that "and" in publisher is expected
//   - Check correct capitalisation of journal
//   - FIX UNUSED COMMAND
//   - check no . at end of title

#[derive(Parser)]
#[command(
    name = crate_name!(),
    author = crate_authors!(", "),
    version = crate_version!(),
    arg_required_else_help = true,
)]
/// Extract archive in memory and get its contents' hash(es)
struct Cli {
    /// LaTeX file
    #[arg(
        short = 'f',
        long = "file",
        action = ArgAction::Set,
        num_args = 0..=1,
        value_name = "latex file",
        default_value = "document.tex",
    )]
    latex_file: String,

    /// BibTeX file
    #[arg(
        short = 'b',
        long = "bibliography",
        action = ArgAction::Set,
        num_args = 0..=1,
        value_name = "bib file",
        default_value = "references.bib",
    )]
    bib_file: String,

    #[clap(flatten)]
    group: Group,
}

// We only want to allow one functional check at a time.  The following group,
// which is flattened in the main Cli struct, should provide such functionality
//
//   https://stackoverflow.com/a/76315811
#[derive(Args)]
#[group(required = true, multiple = false)]
pub struct Group {
    /// Show bib keys of citations in bib file that are not used in LaTeX source
    #[arg(
        short = 'u',
        long = "unused",
        action = ArgAction::SetTrue,
        num_args = 0,
        default_value_t = false,
    )]
    unused: bool,
}

fn main() {
    let cli = Cli::parse();
    let citations = citations::gather_citations(&cli.latex_file, &cli.bib_file);

    if cli.group.unused {
        unused::unused_citations(&citations);
    }

    process::exit(0);
}
