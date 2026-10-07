use serde::{Deserialize, Serialize};

/// Delta from an upstream repository push or poll cycle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitCommitDelta {
    pub repo_name: String,
    pub commit_sha: String,
    pub modified_files: Vec<String>,
    pub added_files: Vec<String>,
}

/// Hybrid Git syncer supporting HMAC webhooks and conditional GraphQL ETag polling
pub struct MultiRepoSyncer {
    pub repositories: Vec<String>,
}

impl MultiRepoSyncer {
    pub fn new(repositories: Vec<String>) -> Self {
        Self { repositories }
    }
}
