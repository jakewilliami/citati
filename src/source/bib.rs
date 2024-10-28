//! Read/parse bibliography source
//!
//! Lightly wrapping the [`biblatex`]'s [`Bibliography`], we parse the given bibliography file, with some additional error handling to improve help messages.

use biblatex::{Bibliography, Entry, EntryType};
use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};
use std::fs;

lazy_static! {
    static ref BIB_RE: Regex = RegexBuilder::new(r"^@(?<type>\w+)\{(?<id>\w+),(?<rest>[^@]*)\}")
        .multi_line(true)
        .dot_matches_new_line(true)
        .build()
        .unwrap();
    static ref UNESCAPED_COMMENT_RE: Regex = Regex::new(r"[^\\]%").unwrap();
}

#[derive(Clone)]
pub struct BibCitation {
    pub key: String,
    pub entry: Entry,
}

impl BibCitation {
    pub fn get(&self, field: &str) -> Option<String> {
        self.entry.get_as::<String>(field).ok()
    }

    pub fn has_field(&self, field: &str) -> bool {
        self.entry.get(field).is_some()
    }

    pub fn has_fields(&self, fields: &[&str]) -> bool {
        fields.iter().all(|f| self.has_field(f))
    }

    pub fn entry_type(&self) -> EntryType {
        self.entry.entry_type.clone()
    }
}

fn comments_in_citation_blocks(src: &str) -> Vec<usize> {
    let mut violating_lines = Vec::new();
    let mut line_number;

    for capture in BIB_RE.captures_iter(src) {
        let full_match = capture.get(0).unwrap().as_str();

        // Calculate the line number in the file for the start of this match
        let match_start = capture.get(0).unwrap().start();
        line_number = src[..match_start].lines().count();

        // Split the entire match into individual lines and iterate over them,
        // checking for violating lines
        for (i, line) in full_match.lines().enumerate() {
            let current_line_number = line_number + i + 1;

            // Check for unescaped '%' in the line
            if UNESCAPED_COMMENT_RE.is_match(line) {
                violating_lines.push(current_line_number)
            }
        }
    }

    violating_lines
}

pub fn parse_bib_from_file(bib_file: &str) -> Bibliography {
    let src = fs::read_to_string(bib_file).unwrap();

    // Check for lines that are malformatted due to comments
    let violating_lines = comments_in_citation_blocks(&src);
    if !violating_lines.is_empty() {
        eprintln!("[WARN] You have comments inside , which Typyst's BibLaTeX does not currently support (typyst/biblatex#64).\n  We will try to parse the bibliography file anyway, but it will likely fail.\n  Violating lines are:\n    {}", violating_lines.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "))
    }

    Bibliography::parse(&src).unwrap()
}
