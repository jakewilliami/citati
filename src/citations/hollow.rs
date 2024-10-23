use super::gather::{CitationSource, GatherCitations};
use crate::{
    bib, latex,
    sources::{Abstract, Bib, LaTeX, Source},
};
use std::{collections::HashSet, marker::PhantomData};

pub struct HollowCitations<S: Source> {
    data: HashSet<String>,
    _source: PhantomData<S>,
}

impl<S: Source> HollowCitations<S> {
    pub fn new() -> Self {
        Self {
            data: HashSet::new(),
            _source: PhantomData,
        }
    }
}

impl<I, S: Source> From<I> for HollowCitations<S>
where
    I: IntoIterator<Item = String>,
{
    fn from(iter: I) -> Self {
        let data = iter.into_iter().collect();
        Self {
            data,
            _source: PhantomData,
        }
    }
}

impl<S: Source> HollowCitations<S> {
    pub fn insert(&mut self, citation: String) -> bool {
        self.data.insert(citation)
    }

    pub fn count(&self) -> usize {
        self.data.len()
    }

    // TODO: consider changing unspecified
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

// TODO: Implement for HollowCitations
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
// TODO: Implement for HollowCitations
impl GatherCitations for HollowCitations<Bib> {
    fn gather(src: &CitationSource) -> Self {
        // TODO: instead of unwrapping source, write impl on CitationSource for validating the field
        let bib = bib::parse_bib_from_file(src.bib_file.unwrap());
        Self::from(bib.keys().map(|s| s.to_string()))
    }
}
