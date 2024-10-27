use lazy_static::lazy_static;
use regex::Regex;
use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

lazy_static! {
    static ref CITATION_REGEX: Regex = Regex::new(r"\\(\w*cite)\{([^}]+)\}").unwrap();
}

/// Struct containing information about a citation from LaTeX
pub struct LaTeXCitation {
    pub _key: String,

    // TODO: make this `Vec<InTexCite>` or `Vec<CiteLoc>` so that we have more information about the position and span of the citation within LaTeX
    // TODO: use an enum for difference cite commands?
    // NOTE: the above TODO notes are only useful if we need that information somewhere downstream.  We barely need `cite_cmds`
    pub _cite_cmds: Vec<String>,
}

/// Token struct containing information about a citation found in LaTeX source
pub struct CitationToken {
    pub key: String,
    pub cite_cmd: String,
}

/// Possible token types from LaTeX source, output by `Lexer`
///
/// We only really care about the `Citation` token; everything else can be `Other`
pub enum Token {
    Citation(CitationToken),
    Other,
}

/// Custom Lexer for LaTeX source code that will find citations
///
/// Lexer will also recurse into `\input{}`s and add them to the `stack`
pub struct Lexer<R: BufRead> {
    reader: R,

    /// Stack of lexers for handling nested inputs
    stack: Vec<Lexer<R>>,

    /// Store the base path of the file we are lexing
    base_path: PathBuf,
}

/// Convenient implementation of construction of `Lexer`
impl<R: BufRead> Lexer<R> {
    /// Constructor method for `Lexer`
    ///
    /// Requires a `base_path` to be constructed so that we can handle recursion into `\input{}`s from the relative source path
    fn new(reader: R, base_path: PathBuf) -> Self {
        Lexer {
            reader,
            stack: Vec::new(),
            base_path,
        }
    }
}

/// Convenient implementation of construction of `Lexer` with concrete reader type from files
impl Lexer<BufReader<File>> {
    /// Construct `Lexer` from `Path`
    fn from_path(latex_file: &Path) -> Self {
        // TODO: better error handling if failed to open file or get absolute path
        // TODO: move path logic into file.rs or path.rs (tuck the logic behind some other module)
        let file = File::open(latex_file).unwrap();
        let reader = BufReader::new(file);
        let abs_path = fs::canonicalize(latex_file).unwrap();
        let base_path = abs_path.parent().unwrap();
        let path_buf = base_path.to_path_buf();
        Self::new(reader, path_buf)
    }

    /// Construct `Lexer` from path `&str`
    pub fn from_str(latex_file: &str) -> Self {
        let path = Path::new(latex_file);
        Self::from_path(path)
    }
}

/// Implements primary `Lexer` functionality to get next token
///
/// Currently only works for `R: BufReader<File>` but in future we should make this more generic (if required)
impl Lexer<BufReader<File>> {
    /// Get the next token from a `Lexer`
    pub fn next_token(&mut self) -> Option<Token> {
        // Check the stack first
        if let Some(top_lexer) = self.stack.last_mut() {
            if let Some(token) = top_lexer.next_token() {
                return Some(token);
            } else {
                // Pop the lexer if it's exhausted
                self.stack.pop();
            }
        }

        // Continue reading from the main lexer
        let mut buffer = String::new();
        if self.reader.read_line(&mut buffer).unwrap() == 0 {
            return None; // EOF
        }

        // Remove comments from the line
        if let Some(comment_pos) = buffer.find('%') {
            // Keep only the part before the comment
            buffer.truncate(comment_pos);
        }

        let line = buffer.trim();

        // Check for citation commands
        if let Some(caps) = CITATION_REGEX.captures(line) {
            let cite_cmd = caps.get(1).map(|m| m.as_str().to_owned()).unwrap();
            let citations = caps.get(2).map_or("", |m| m.as_str());

            // Split citations by comma and return each as a separate token
            for citation in citations.split(',') {
                let citation = citation.trim();
                if !citation.is_empty() {
                    let key = citation.to_string();
                    return Some(Token::Citation(CitationToken { key, cite_cmd }));
                }
            }
        }

        // Recurse into \input
        // TODO: recurse immediately rather than adding them to the stack, as this would be more like how the LaTeX compiler does it
        if let Some(input_pos) = line.find("\\input") {
            // Offset starting position to get the file name; the length of "\input{" is 7
            let start = input_pos + 7;
            if let Some(end) = line[start..].find('}') {
                let filename_str = line[start..start + end].trim().to_string();
                let mut filename = PathBuf::from(filename_str);

                // Prepend the base path to the filename
                filename = self.base_path.join(&filename);

                // Check if the filename has an extension; if not, add ".tex"
                if filename.extension().is_none() {
                    filename.set_extension("tex");
                }

                // Process the input file
                let lexer = Self::from_path(&filename);

                // Push the new lexer onto the stack
                self.stack.push(lexer);

                // Return the next token from the newly added lexer
                return self.stack.last_mut().unwrap().next_token();
            }
        }

        // If no citation command is found, treat it as 'Other'.
        Some(Token::Other)
    }
}
