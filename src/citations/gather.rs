pub struct CitationSource<'a> {
    pub bib_file: Option<&'a str>,
    pub latex_file: Option<&'a str>,
}

impl<'a> CitationSource<'a> {
    pub fn new(latex_file: &'a str, bib_file: &'a str) -> Self {
        Self {
            bib_file: Some(bib_file),
            latex_file: Some(latex_file),
        }
    }

    pub fn from_latex(latex_file: &'a str) -> Self {
        Self {
            bib_file: None,
            latex_file: Some(latex_file),
        }
    }

    pub fn from_bib(bib_file: &'a str) -> Self {
        Self {
            bib_file: Some(bib_file),
            latex_file: None,
        }
    }
}

pub trait GatherCitations {
    fn gather(src: &CitationSource) -> Self;
}

// TODO: Generic function to gather citations
pub fn gather_citations<T: GatherCitations>(src: &CitationSource) -> T {
    T::gather(src)
}
