use std::collections::HashMap;

/// Branch in PTFS version graph
#[derive(Debug, Clone)]
pub struct Branch {
    pub name: String,
    pub head_commit: Option<String>,
}

pub struct BranchManager {
    pub branches: HashMap<String, Branch>,
}

impl BranchManager {
    pub fn new() -> Self {
        Self {
            branches: HashMap::new(),
        }
    }

    pub fn create_branch(&mut self, name: &str, head: Option<String>) {
        self.branches.insert(
            name.to_string(),
            Branch {
                name: name.to_string(),
                head_commit: head,
            },
        );
    }

    pub fn update_head(&mut self, name: &str, commit_id: String) {
        if let Some(branch) = self.branches.get_mut(name) {
            branch.head_commit = Some(commit_id);
        }
    }
}
