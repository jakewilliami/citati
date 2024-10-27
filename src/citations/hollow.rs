use super::gather::{CitationSource, GatherCitations};
use crate::{
    bib, latex,
    sources::{Abstract, Bib, LaTeX, Source},
};
use std::{collections::HashSet, marker::PhantomData};

/// Collection of citation keys from some specified source
///
/// This is "hollow" or "shallow" because we don't need to store any more information beyond the citation keys.  Note that `_source` is required so that we use the generic type `S: Source` to allow type inference etc. to work (I don't fully understand it but if I remove it things stop working)
pub struct HollowCitations<S: Source> {
    data: HashSet<String>,
    _source: PhantomData<S>,
}

/// Convenient implementation of construction of `HollowCitations`
impl<S: Source> HollowCitations<S> {
    /// Constructor method for `HollowCitations`
    pub fn new() -> Self {
        Self {
            data: HashSet::new(),
            _source: PhantomData,
        }
    }
}

/// Conveninent implementation of construction of `HollowCitations` from an iterable
impl<I, S: Source> From<I> for HollowCitations<S>
where
    I: IntoIterator<Item = String>,
{
    /// Constructor method for `HollowCitations` from iterable
    fn from(iter: I) -> Self {
        let data = iter.into_iter().collect();
        Self {
            data,
            _source: PhantomData,
        }
    }
}

/// Implementations on the `HollowCitations` struct for convenience
///
/// These implementations typically access the underlying data but provide a useful/convenient API for the `HollowCitations` struct
impl<S: Source> HollowCitations<S> {
    pub fn insert(&mut self, citation: String) -> bool {
        self.data.insert(citation)
    }

    // NOTE: the following can be uncommented if/when needed
    // pub fn count(&self) -> usize {
    //     self.data.len()
    // }

    /// Create an `Abstract` `HollowCitations` struct from the set difference of two `HollowCitations` objects
    pub fn difference<R: Source>(&self, other: HollowCitations<R>) -> HollowCitations<Abstract> {
        let data = self.data.difference(&other.data).cloned();
        HollowCitations::<Abstract>::from(data)
    }

    pub fn list_sorted(&self) -> Vec<String> {
        let mut citations: Vec<String> = self.data.iter().cloned().collect();
        citations.sort();
        citations
    }
}

/// Implement the `gather` function for `HollowCitations` for LaTeX source code
impl GatherCitations for HollowCitations<LaTeX> {
    fn gather(src: &CitationSource) -> Self {
        // TODO: instead of unwrapping source, write impl on CitationSource for validating the field
        let mut lexer = latex::Lexer::from_str(src.latex_file.unwrap());
        let mut citations = HollowCitations::<LaTeX>::new();
        while let Some(token) = lexer.next_token() {
            if let latex::Token::Citation(citation) = token {
                citations.insert(citation.key);
            }
        }
        citations
    }
}

/// Implement the `gather` function for `HollowCitations` for a bibliography
impl GatherCitations for HollowCitations<Bib> {
    fn gather(src: &CitationSource) -> Self {
        // TODO: instead of unwrapping source, write impl on CitationSource for validating the field
        let bib = bib::parse_bib_from_file(src.bib_file.unwrap());
        Self::from(bib.keys().map(|s| s.to_string()))
    }
}
