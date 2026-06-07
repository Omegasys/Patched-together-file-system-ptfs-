use crate::trait_::StorageBackend;

/// GitHub-backed storage (snapshots stored as repo commits/blobs)
pub struct GitHubBackend {
    pub repo: String,
    pub token: String,
}

impl GitHubBackend {
    pub fn new(repo: String, token: String) -> Self {
        Self { repo, token }
    }
}

impl StorageBackend for GitHubBackend {
    fn name(&self) -> &str {
        "github"
    }

    fn upload(&self, key: &str, data: &[u8]) -> anyhow::Result<()> {
        println!(
            "[PTFS-GITHUB] commit blob {} to repo {} ({} bytes)",
            key,
            self.repo,
            data.len()
        );

        // Real system mapping:
        // - blob → Git object
        // - tree → snapshot structure
        // - commit → PTFS snapshot
        // - push via GitHub API or git CLI

        Ok(())
    }

    fn download(&self, key: &str) -> anyhow::Result<Vec<u8>> {
        println!(
            "[PTFS-GITHUB] fetch blob {} from repo {}",
            key,
            self.repo
        );

        Ok(vec![4, 5, 6]) // placeholder
    }

    fn health_check(&self) -> bool {
        !self.repo.is_empty()
    }
}
