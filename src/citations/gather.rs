//! Specify gather trait for citation
//!
//! The [`GatherCitations`] trait is herein defined.  Types can implement if by taking a [`CitationSource`] and returning an instance of themselves.

use crate::source::CitationSource;

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
