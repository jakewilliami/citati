use super::gather::{gather_citations, CitationSource, GatherCitations};
use crate::{
    bib::{self, BibCitation},
    latex::{self, LaTeXCitation},
    sources::{Bib, Composite, LaTeX, Source},
};
use biblatex::Entry;
use std::{
    collections::{HashMap, HashSet},
    marker::PhantomData,
};

#[derive(Clone)]
pub struct Citation {
    pub key: String,
    entry: Option<Entry>,
    cite_cmds: Vec<String>,
}

impl Citation {
    /// Was the citation actually cited in the LaTeX source?
    pub fn cited(&self) -> bool {
        !self.cite_cmds.is_empty()
    }

    /// Was the citation found in a bibliography file?
    pub fn in_bib(&self) -> bool {
        self.entry.is_some()
    }

    // TODO: document
    pub fn get(&self, field: &str) -> Option<String> {
        //TODO: use chunks or whatever
        self.entry
            .as_ref()
            .and_then(|e| e.get_as::<String>(field).ok())
    }
}

pub struct Citations<S: Source> {
    data: HashMap<String, S::CitationType>,
    // TODO: do we need phantom data?
    _source: PhantomData<S>,
}

impl<S: Source> Citations<S> {
    pub fn iter(&self) -> impl Iterator<Item = &S::CitationType> {
        self.data.values()
    }

    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.data.keys()
    }

    pub fn get(&self, key: &String) -> Option<&S::CitationType> {
        self.data.get(key)
    }

    pub fn list_sorted(&self) -> Vec<&S::CitationType> {
        let mut citations: Vec<(&String, &S::CitationType)> = self.data.iter().collect();
        citations.sort_by_key(|(key, _)| *key);
        citations
            .into_iter()
            .map(|(_, citation)| citation)
            .collect()
    }

    pub fn filter<F>(&self, predicate: F) -> Citations<S>
    where
        F: Fn(&S::CitationType) -> bool,
        S::CitationType: Clone,
    {
        let filtered_data: HashMap<String, S::CitationType> = self
            .data
            .iter()
            .filter(|(_, citation)| predicate(citation))
            .map(|(key, citation)| (key.clone(), citation.clone()))
            .collect();

        Citations {
            data: filtered_data,
            _source: PhantomData,
        }
    }
}

// TODO: document
impl GatherCitations for Citations<LaTeX> {
    // impl GatherCitations for LaTeX {
    fn gather(src: &CitationSource) -> Self {
        // TODO: instead of unwrapping source, write impl on CitationSource for validating both fields
        let mut lexer = latex::Lexer::from_str(src.latex_file.unwrap());
        let mut citations = HashMap::<String, Vec<latex::CitationToken>>::new();
        while let Some(token) = lexer.next_token() {
            if let latex::Token::Citation(citation) = token {
                citations
                    .entry(citation.key.to_owned())
                    .or_default()
                    .push(citation);
            }
        }
        let mut data = HashMap::new();
        for key in citations.keys() {
            let cite_cmds = citations
                .get(key)
                .map_or_else(Vec::new, |v| v.iter().map(|s| s.cite_cmd.clone()).collect());
            data.insert(
                key.clone(),
                LaTeXCitation {
                    key: key.to_owned(),
                    cite_cmds,
                },
            );
        }

        Self {
            data,
            _source: PhantomData,
        }
    }
}

// TODO: document
impl GatherCitations for Citations<Bib> {
    fn gather(src: &CitationSource) -> Self {
        let bib = bib::parse_bib_from_file(src.bib_file.unwrap());
        let mut data = HashMap::new();
        for entry in bib.iter() {
            data.insert(
                entry.key.clone(),
                BibCitation {
                    key: entry.key.clone(),
                    entry: entry.clone(),
                },
            );
        }

        Self {
            data,
            _source: PhantomData,
        }
    }
}

// TODO: Implement the trait for Citations
impl GatherCitations for Citations<Composite> {
    fn gather(src: &CitationSource) -> Self {
        let citations = gather_citations::<Citations<LaTeX>>(&src);
        let bib_entries = gather_citations::<Citations<Bib>>(&src);

        let mut data = HashMap::new();
        let keys: HashSet<String> = citations
            .keys()
            .chain(bib_entries.keys())
            .cloned()
            .collect();

        for key in keys {
            let entry = bib_entries.get(&key).and_then(|c| Some(c.entry.clone()));
            let cite_cmds = citations
                .get(&key)
                .map_or_else(Vec::new, |c| c.cite_cmds.clone());
            data.insert(
                key.clone(),
                Citation {
                    key,
                    entry,
                    cite_cmds,
                },
            );
        }

        Self {
            data,
            _source: PhantomData,
        }
    }
}
