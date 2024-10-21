use super::{bib, citations::Citations, latex};

fn unused_citations(citations: Citations, bib_entries: Citations) -> Citations {
    bib_entries.difference(citations)
}

pub fn unused_citations_from_sources(latex_file: &str, bib_file: &str) {
    let citations = latex::gather_citations(latex_file);
    let bib_entries = bib::gather_bib_entries(bib_file);
    let unused = unused_citations(citations, bib_entries);

    for citation in unused.list_sorted() {
        println!("{citation}");
    }
}
