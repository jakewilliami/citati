//! Define source types (as singletons) and [`Source`] trait
//!
//! Implement singleton types (i.e., [`LaTeX`] and [`Bib`]) that pertain to different data sources.  We also define the `Abstract` type, which is useful downstream (see [`GatherCitations`](`crate::citations::GatherCitations`))

use super::bib::BibCitation;
use super::latex::LaTeXCitation;

/*
  Singleton types to denote the source of some citations

  <https://stackoverflow.com/a/72438660>
*/

/// Singleton LaTeX source type
pub struct LaTeX;
/// Singleton bib source type
pub struct Bib;
/// Singleton abstract source type
pub struct Abstract;

// NOTE: the following can be uncommented if/when needed
// /// Singleton composite source type
// pub struct Composite;

/*
  Source implementations for defined source types

  <https://stackoverflow.com/a/72450472>
*/

/// Trait to denote that a type that implements it is a source
pub trait Source {
    type CitationType;
}

/// The source is LaTex source code (.tex)
impl Source for LaTeX {
    type CitationType = LaTeXCitation;
}

/// The source is a bib(la)tex bibliography (.bib)
impl Source for Bib {
    type CitationType = BibCitation;
}

/// Abstract source, where the source is technically unspecified or indeterminate
impl Source for Abstract {
    type CitationType = ();
}

// NOTE: the following can be uncommented if/when needed
// /// Composite/aggregated from multiple sources
// impl Source for Composite {
//     type CitationType = Citation;
// }

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
