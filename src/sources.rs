use super::bib::BibCitation;
use super::citations::full::Citation;
use super::latex::LaTeXCitation;

// https://stackoverflow.com/a/72438660
pub struct LaTeX;
pub struct Bib;
pub struct Composite;
pub struct Abstract;

// https://stackoverflow.com/a/72450472
pub trait Source {
    type CitationType;
}

impl Source for LaTeX {
    type CitationType = LaTeXCitation;
}

impl Source for Bib {
    type CitationType = BibCitation;
}

impl Source for Composite {
    type CitationType = Citation;
}

impl Source for Abstract {
    type CitationType = ();
}
