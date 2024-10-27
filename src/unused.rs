use super::{
    citations::{
        gather::{gather_citations, CitationSource},
        hollow::HollowCitations,
    },
    source::sources::{Bib, LaTeX},
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
