use super::gather::{CitationSource, GatherCitations};
use crate::source::{
    bib::{self, BibCitation},
    latex::{self, LaTeXCitation},
    sources::{Bib, LaTeX, Source},
};
use std::collections::HashMap;

// NOTE: the following can be uncommented if/when needed
// /// Struct containing composite information from both LaTeX and bib sources
// pub struct Citation {
//     pub key: String,
//     entry: Option<Entry>,
//     cite_cmds: Vec<String>,
// }
//
// impl Citation {
//     /// Was the citation actually cited in the LaTeX source?
//     pub fn cited(&self) -> bool {
//         !self.cite_cmds.is_empty()
//     }
//
//     /// Was the citation found in a bibliography file?
//     pub fn in_bib(&self) -> bool {
//         self.entry.is_some()
//     }
//
//     /// Get a field from the bib entry of the citation, if present
//     pub fn get(&self, field: &str) -> Option<String> {
//         self.entry
//             .as_ref()
//             .and_then(|e| e.get_as::<String>(field).ok())
//     }
// }

/// Struct containing a collection of citations
///
/// The type of citations in the collection depends on the source of the citations
pub struct Citations<S: Source> {
    data: HashMap<String, S::CitationType>,
}

/// Implementations on the `Citations` struct for convenience
///
/// These implementations typically access the underlying data but provide a useful/convenient API for the `Citations` struct
impl<S: Source> Citations<S> {
    // NOTE: the following can be uncommented if/when needed
    // pub fn iter(&self) -> impl Iterator<Item = &S::CitationType> {
    //     self.data.values()
    // }
    //
    // pub fn keys(&self) -> impl Iterator<Item = &String> {
    //     self.data.keys()
    // }
    //
    // pub fn get(&self, key: &String) -> Option<&S::CitationType> {
    //     self.data.get(key)
    // }

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
        let data: HashMap<String, S::CitationType> = self
            .data
            .iter()
            .filter(|(_, citation)| predicate(citation))
            .map(|(key, citation)| (key.clone(), citation.clone()))
            .collect();

        Citations { data }
    }
}

/// Implement the `gather` function for `Citations` for LaTeX source code
impl GatherCitations for Citations<LaTeX> {
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
                    _key: key.to_owned(),
                    _cite_cmds: cite_cmds,
                },
            );
        }

        Self { data }
    }
}

/// Implement the `gather` function for `Citations` for a bibliography
impl GatherCitations for Citations<Bib> {
    fn gather(src: &CitationSource) -> Self {
        // TODO: instead of unwrapping source, write impl on CitationSource for validating the field
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

        Self { data }
    }
}

// NOTE: the following can be uncommented if/when needed
// /// Collect citations from multiple sources and aggregate/compose them
// impl GatherCitations for Citations<Composite> {
//     fn gather(src: &CitationSource) -> Self {
//         let citations = gather_citations::<Citations<LaTeX>>(&src);
//         let bib_entries = gather_citations::<Citations<Bib>>(&src);
//
//         let mut data = HashMap::new();
//         let keys: HashSet<String> = citations
//             .keys()
//             .chain(bib_entries.keys())
//             .cloned()
//             .collect();
//
//         for key in keys {
//             let entry = bib_entries.get(&key).and_then(|c| Some(c.entry.clone()));
//             let cite_cmds = citations
//                 .get(&key)
//                 .map_or_else(Vec::new, |c| c.cite_cmds.clone());
//             data.insert(
//                 key.clone(),
//                 Citation {
//                     key,
//                     entry,
//                     cite_cmds,
//                 },
//             );
//         }
//
//         Self { data }
//     }
// }
