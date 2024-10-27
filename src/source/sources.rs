use super::bib::BibCitation;
use super::latex::LaTeXCitation;

/// Singleton types to denote the source of some citations
///
/// <https://stackoverflow.com/a/72438660>

pub struct LaTeX;
pub struct Bib;
pub struct Abstract;

// NOTE: the following can be uncommented if/when needed
// pub struct Composite;

/// Source implementations for defined source types
///
/// <https://stackoverflow.com/a/72450472>

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
