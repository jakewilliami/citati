use clap::{crate_authors, crate_name, crate_version, ArgAction, Args, Parser};

mod citations;
mod fields;
mod pages;
mod source;
mod unused;

// TODO:
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
//   - Check that inside parentheses we are using `nptextcite`
//   - Make citations an enum type from string

#[derive(Parser)]
#[command(
    name = crate_name!(),
    author = crate_authors!(", "),
    version = crate_version!(),
    arg_required_else_help = true,
)]
/// Citation helper for BibTex
///
/// Look through citations in LaTeX/bibliography source and perform various checks for correctness.  Name derived from цитаты (_tsitaty_): quotes/citations.
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

/// Group containing individual functional units for the program.
///
/// We only want to allow one functional check at a time.  The following group, which is flattened in the main Cli struct, should provide such functionality.
///
/// <https://stackoverflow.com/a/76315811>
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

    /// Show bib keys of citations in bib file that do not use proper formatting for pages
    #[arg(
        short = 'p',
        long = "pages",
        action = ArgAction::SetTrue,
        num_args = 0,
        default_value_t = false,
    )]
    pages: bool,

    /// Show bib keys of article citations in bib file that do not contain required fields
    #[arg(
        short = 'a',
        long = "article",
        action = ArgAction::SetTrue,
        num_args = 0,
        default_value_t = false,
    )]
    article: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.group.unused {
        unused::unused_citations(&cli.latex_file, &cli.bib_file);
    } else if cli.group.pages {
        pages::check_bib_pages(&cli.bib_file);
    } else if cli.group.article {
        fields::article::check_article_fields(&cli.bib_file);
    }

    std::process::exit(0);
}
