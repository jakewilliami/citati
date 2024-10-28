//! Check presence of required fields in article entries
//!
//! Check all bibliography entries of type `article` have required fields.  This module contains logic for this functionality, which can be accessed using the [`--article`](`crate::Group::article`) option.  Unfortunately, some journals will have non-standard article reporting, so this will produce some false positives.

use crate::{
    citations::{gather_citations, Citations},
    source::{Bib, BibCitation, CitationSource},
};
use biblatex::EntryType;
use lazy_static::lazy_static;

lazy_static! {
    static ref REQUIRED_ARTICLE_FIELDS: &'static [&'static str] =
        &["volume", "number", "pages", "doi"];
}

/// Construct user-friendly string to report missing article fields
fn report_article(citation: &BibCitation) -> String {
    let missing_fields: Vec<&str> = REQUIRED_ARTICLE_FIELDS
        .iter()
        .filter(|f| !citation.has_field(f))
        .cloned()
        .collect();

    if missing_fields.is_empty() {
        eprintln!("[ERROR] Cannot report missing article fields when none are missing");
        std::process::exit(1);
    }

    format!("{} (missing: {})", citation.key, missing_fields.join(", "))
}

/// Check for missing article fields
pub fn check_article_fields(bib_file: &str) {
    let src = CitationSource::from_bib(bib_file);
    let bib_entries = gather_citations::<Citations<Bib>>(&src);
    let articles_with_missing_fields = bib_entries.filter(|c| {
        c.entry_type() == EntryType::Article && !c.has_fields(&REQUIRED_ARTICLE_FIELDS)
    });

    for citation in articles_with_missing_fields.list_sorted() {
        println!("{}", report_article(citation));
    }
}
