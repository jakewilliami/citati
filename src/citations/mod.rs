//! A module to contain different types of collections of citations
//!
//! This module describes how to gather citations from various sources into collections.  Depending on the amount of information required downstream, we can gather citations with varying depths of information.  See release notes for [v0.2.0](https://github.com/jakewilliami/citati/releases/tag/v0.2.0), where this functionality was first developed.

mod full;
mod gather;
mod hollow;

pub use full::Citations;
pub use gather::{gather_citations, GatherCitations};
pub use hollow::HollowCitations;
