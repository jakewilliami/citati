use std::collections::HashSet;

pub struct Citations {
    data: HashSet<String>,
}

impl Citations {
    pub fn new() -> Self {
        Citations {
            data: HashSet::new(),
        }
    }
}

impl<I> From<I> for Citations
where
    I: IntoIterator<Item = String>,
{
    fn from(iter: I) -> Self {
        let data = iter.into_iter().collect();
        Citations { data }
    }
}

impl Citations {
    pub fn insert(&mut self, citation: String) -> bool {
        self.data.insert(citation)
    }

    pub fn contains(&self, citation: &String) -> bool {
        self.data.contains(citation)
    }

    pub fn count(&self) -> usize {
        self.data.len()
    }

    fn difference(&self, other: &Self) -> Self {
        let data = self.data.difference(&other.data).cloned();
        Self::from(data)
    }
}
