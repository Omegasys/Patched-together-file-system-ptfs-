use std::collections::HashMap;

/// Lightweight tagging system (like Git tags)
pub struct TagManager {
    pub tags: HashMap<String, String>, // tag → commit_id
}

impl TagManager {
    pub fn new() -> Self {
        Self {
            tags: HashMap::new(),
        }
    }

    pub fn create_tag(&mut self, name: &str, commit_id: &str) {
        self.tags.insert(name.to_string(), commit_id.to_string());
    }

    pub fn get(&self, name: &str) -> Option<&String> {
        self.tags.get(name)
    }

    pub fn delete(&mut self, name: &str) {
        self.tags.remove(name);
    }
}
