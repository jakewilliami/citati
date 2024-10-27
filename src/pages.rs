use super::{
    bib::BibCitation,
    citations::{
        full::Citations,
        gather::{gather_citations, CitationSource},
    },
    sources::Bib,
};
use lazy_static::lazy_static;
use regex::Regex;
use std::char;

lazy_static! {
    static ref EM_DASH_CHAR: char = char::from_u32(0x2013).unwrap();
    static ref BIB_PAGES_RE: Regex =
        Regex::new(&format!(r"^\d+(--|{})\d+$", *EM_DASH_CHAR)).unwrap();
}

/// Defines formatting for reporting citations from bib file with malformatted `pages` field
fn report_pages(citation: &BibCitation) -> String {
    format!(
        "{} ({:?})",
        citation.key,
        citation.get("pages").unwrap_or_default()
    )
}

/// List (in alohabetical order) any citations from the bibliography that have malformatted `pages` field
///
/// Pages should typically be separated by an en dash
pub fn check_bib_pages(bib_file: &str) {
    let src = CitationSource::from_bib(bib_file);
    let bib_entries = gather_citations::<Citations<Bib>>(&src);
    let citations_with_bad_bib_pages = bib_entries.filter(|c| {
        c.get("pages")
            .map_or(false, |pages| !BIB_PAGES_RE.is_match(&pages))
    });

    for citation in citations_with_bad_bib_pages.list_sorted() {
        println!("{}", report_pages(citation));
    }
}
