/// Source of citations being used or defined
///
/// Either from bibliography file (.bib) or LaTeX file(s) (.tex)
pub struct CitationSource<'a> {
    pub latex_file: Option<&'a str>,
    pub bib_file: Option<&'a str>,
}

/// Convenient implementations for construction of `CitationSource`
impl<'a> CitationSource<'a> {
    /// Create a new `CitationSource` object from both source files
    pub fn new(latex_file: &'a str, bib_file: &'a str) -> Self {
        Self {
            latex_file: Some(latex_file),
            bib_file: Some(bib_file),
        }
    }

    // NOTE: the following can be uncommented if/when needed
    // /// Create a new `CitationSource` object from just LaTeX file
    // pub fn from_latex(latex_file: &'a str) -> Self {
    //     Self {
    //         latex_file: Some(latex_file),
    //         bib_file: None,
    //     }
    // }

    /// Create a new `CitationSource` object from just bib file
    pub fn from_bib(bib_file: &'a str) -> Self {
        Self {
            latex_file: None,
            bib_file: Some(bib_file),
        }
    }
}

/// Trait that takes a `CitationSource` and returns some collection of citations
///
/// From a `CitationSource`, we read the source and gather citations into `Self`
pub trait GatherCitations {
    fn gather(src: &CitationSource) -> Self;
}

/// Gather citations into an of type `T` from the source
///
/// Given a citation source, we call to the `gather` function on a type that implements the GatherCitations` trait, and returns an object of that type
pub fn gather_citations<T: GatherCitations>(src: &CitationSource) -> T {
    T::gather(src)
}
