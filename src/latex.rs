use regex::Regex;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

#[derive(Debug)]
enum Token {
    Citation(String),
    Other,
}

struct Lexer<R: BufRead> {
    reader: R,
    citation_regex: Regex,

    // Stack of lexers for handling nested inputs
    stack: Vec<Lexer<R>>,

    // Store the base path of the file we are lexing
    base_path: PathBuf,
}

// Create a new lexer
impl<R: BufRead> Lexer<R> {
    fn new(reader: R, base_path: PathBuf) -> Self {
        let citation_regex = Regex::new(r"\\\w*cite\{([^}]+)\}").unwrap();
        Lexer {
            reader,
            citation_regex,
            stack: Vec::new(),
            base_path,
        }
    }
}

// Construct lexer with concrete reader type from file
impl Lexer<BufReader<File>> {
    // TODO: better error handling if failed to open file or get absolute path
    // TODO: move path logic into file.rs or path.rs (tuck the logic behind some other module)
    fn from_path(latex_file: &Path) -> Self {
        let file = File::open(latex_file).unwrap();
        let reader = BufReader::new(file);
        let abs_path = fs::canonicalize(latex_file).unwrap();
        let base_path = abs_path.parent().unwrap();
        let path_buf = base_path.to_path_buf();
        Self::new(reader, path_buf)
    }

    fn from_str(latex_file: &str) -> Self {
        let path = Path::new(latex_file);
        Self::from_path(path)
    }
}

// Implement primary lexer function to get next token
// Currently only works for R: BufReader<File>, but in future we should make this more generic
impl Lexer<BufReader<File>> {
    fn next_token(&mut self) -> Option<Token> {
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
        if let Some(caps) = self.citation_regex.captures(line) {
            let citations = caps.get(1).map_or("", |m| m.as_str());

            // Split citations by comma and return each as a separate token
            for citation in citations.split(',') {
                let citation = citation.trim();
                if !citation.is_empty() {
                    return Some(Token::Citation(citation.to_string()));
                }
            }
        }

        // Recurse into \input
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

pub fn gather_citations(latex_file: &str) -> Vec<String> {
    let mut lexer = Lexer::from_str(latex_file);

    let mut citations = Vec::new();

    while let Some(token) = lexer.next_token() {
        if let Token::Citation(citation) = token {
            citations.push(citation);
        }
    }

    citations
}
