use clap::{crate_authors, crate_name, crate_version, Parser};
use std::process;

#[derive(Parser)]
#[command(
    name = crate_name!(),
    author = crate_authors!(", "),
    version = crate_version!(),
)]
/// Extract archive in memory and get its contents' hash(es)
struct Cli {}

fn main() {
    let cli = Cli::parse();

    process::exit(0);
}
