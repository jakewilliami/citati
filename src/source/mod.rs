//! Module pertaining to data sources
//!
//! The data that we work with in citati have varying sources, from LaTeX source code to bibliography files.  This module contains submodules that help to handle these required data from various sources.

pub mod bib;
pub mod latex;
mod sources;

pub use bib::BibCitation;
pub use latex::LaTeXCitation;
pub use sources::{Abstract, Bib, CitationSource, LaTeX, Source};
