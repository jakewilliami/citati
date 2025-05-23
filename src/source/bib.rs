//! Read/parse bibliography source
//!
//! Lightly wrapping the [`biblatex`]'s [`Bibliography`], we parse the given bibliography file, with some additional error handling to improve help messages.

use biblatex::{Bibliography, Entry, EntryType};
use std::fs;

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

// A private, helper trait to determine whether a character at which the
// cursor is pointing at in some buffer is escaped using backslashes.
trait LaTeXCharEscaped {
    fn is_escaped(&self) -> bool;
}

impl LaTeXCharEscaped for str {
    fn is_escaped(&self) -> bool {
        // Count the number of consecutive backslashes before the character
        // in order to determine whether the character has been escaped or not
        let mut n = 0;
        for c in self.chars().rev() {
            if c == '\\' {
                n += 1;
            } else {
                break;
            }
        }

        // If the number of consecutive backslashes immediately preceeding
        // the character is odd, then it has been escaped; otherwise, these
        // are literal backslashes that have each been escaped an even number
        // of times and so they are all treated as character literals
        n % 2 == 1
    }
}

fn strip_comments(src: &str) -> String {
    let mut out = String::new();

    // We want to strip comments (indicated by %) from each line of source.
    // If we encounter a % character, we can skip to the next line.
    for line in src.lines() {
        let mut buf = String::new();

        'chars: for ch in line.chars() {
            if ch == '%' && !buf.is_escaped() {
                // In this case, we have encountered a LaTeX-style comment.
                // The other comment that we could have encountered is a legacy
                // BibTeX-style comment (@Comment {}), however, I don't want to
                // re-write the lexer that Typst provides and I don't typically
                // use this style of comment, so I am happy to ignore it until
                // or unless it becomes a problem...

                // Trim superfluous whitespace from end of string preceeding
                // comment if needed.  We do this in-place by truncating the
                // string until we no longer find any whitespace.  This should
                // handle unicode as we truncate (pop) character by character
                // rather than using truncate with indices.
                //
                // See earlier versions in ec65e24, 6e70f50, and 783fa42
                while buf.ends_with(char::is_whitespace) {
                    buf.pop();
                }

                // Continue to the next line as a comment has been encountered
                // and handled.  Technically we continue to the end of the `chars`
                // loop as we still need to push the current line buffer to the
                // source file, which happens at the end of each `chars` loop.
                break 'chars;
            } else {
                // If no comment has been encountered, then we can push the present
                // character to the line buffer, carefree.
                buf.push(ch);
            }
        }

        // Add the current line buffer to the end of the modified source file
        if !buf.is_empty() {
            out.push_str(&buf);
            out.push('\n');
        }
    }

    out
}

pub fn parse_bib_from_file(bib_file: &str) -> Bibliography {
    let mut src = fs::read_to_string(bib_file).unwrap();

    // As of v0.3.3, we no longer warn the user about comments in their bibliography src
    src = strip_comments(&src);

    // Parse the file into a bibliography
    Bibliography::parse(&src).unwrap()
}
