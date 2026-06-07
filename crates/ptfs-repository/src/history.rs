use std::collections::HashMap;

/// Commit graph (DAG structure)
#[derive(Default)]
pub struct History {
    pub commits: HashMap<String, Vec<String>>, // parent → children
}

impl History {
    pub fn new() -> Self {
        Self {
            commits: HashMap::new(),
        }
    }

    pub fn add_commit(&mut self, commit_id: String, parent: Option<String>) {
        if let Some(p) = parent {
            self.commits
                .entry(p)
                .or_default()
                .push(commit_id);
        }
    }

    pub fn children_of(&self, commit_id: &str) -> Option<&Vec<String>> {
        self.commits.get(commit_id)
    }
}
