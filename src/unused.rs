//! Check LaTeX source for any unused citations defined in bibliography
//!
//! Check for any bibliography entries that are defined but not used in the LaTeX source.  This module contains the logic for this functionality, which can be accessed using the [`--unused`](`crate::Group::unused`) option.

use super::{
    citations::{gather_citations, HollowCitations},
    source::{Bib, CitationSource, LaTeX},
};

/// List (in alphabetical order) any unused citations from LaTeX and bib sources
pub fn unused_citations(latex_file: &str, bib_file: &str) {
    let src = CitationSource::new(latex_file, bib_file);
    let citations = gather_citations::<HollowCitations<LaTeX>>(&src);
    let bib_entries = gather_citations::<HollowCitations<Bib>>(&src);
    let unused = bib_entries.difference(citations);

    for citation in unused.list_sorted() {
        println!("{citation}");
    }
}
