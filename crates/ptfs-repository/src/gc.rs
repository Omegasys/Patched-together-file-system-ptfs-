use std::collections::HashSet;
use std::collections::HashMap;

/// Garbage collector for unreachable commits
pub struct GarbageCollector;

impl GarbageCollector {
    pub fn new() -> Self {
        Self
    }

    /// Mark-and-sweep GC for commit graph
    pub fn collect(
        commits: &HashMap<String, Vec<String>>,
        roots: &[String],
    ) -> HashSet<String> {
        let mut reachable = HashSet::new();
        let mut stack = roots.to_vec();

        while let Some(current) = stack.pop() {
            if reachable.insert(current.clone()) {
                if let Some(children) = commits.get(&current) {
                    for c in children {
                        stack.push(c.clone());
                    }
                }
            }
        }

        reachable
    }

    pub fn sweep(
        all_commits: HashSet<String>,
        reachable: HashSet<String>,
    ) -> Vec<String> {
        all_commits
            .difference(&reachable)
            .cloned()
            .collect()
    }
}
