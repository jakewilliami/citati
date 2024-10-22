use super::{bib, latex};
use biblatex::Entry;
use std::collections::{HashMap, HashSet};

pub struct Citation {
    pub key: String,
    entry: Option<Entry>,
    cite_cmd: Option<String>,
}

impl Citation {
    /// Was the citation actually cited in the LaTeX source?
    pub fn cited(&self) -> bool {
        self.cite_cmd.is_some()
    }

    /// Was the citation found in a bibliography file?
    pub fn in_bib(&self) -> bool {
        self.entry.is_some()
    }
}

pub struct Citations {
    data: HashMap<String, Citation>,
}

impl Citations {
    pub fn iter(&self) -> impl Iterator<Item = &Citation> {
        self.data.values()
    }
}

pub fn gather_citations(latex_file: &str, bib_file: &str) -> Citations {
    let citations = latex::gather_citations(latex_file);
    let bib_entries = bib::gather_bib_entries(bib_file);
    let mut data = HashMap::new();
    let keys: HashSet<String> = citations
        .keys()
        .chain(bib_entries.keys())
        .cloned()
        .collect();

    for key in keys {
        let entry = bib_entries.get(&key).cloned();
        let cite_cmd = citations.get(&key).map(|c| c.cite_cmd.clone());
        data.insert(
            key.clone(),
            Citation {
                key,
                entry,
                cite_cmd,
            },
        );
    }

    Citations { data }
}
