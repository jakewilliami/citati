use super::citations::Citations;

pub fn unused_citations(citations: &Citations) {
    let mut unused: Vec<String> = citations
        .iter()
        .filter(|c| c.in_bib() && !c.cited())
        .map(|c| c.key.clone())
        .collect();
    unused.sort();

    for citation in unused.iter() {
        println!("{citation}")
    }
}
