use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::verification::ExitReason;

/// Supported programming languages across TheAlgorithms ecosystem
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    Rust,
    Python,
    Java,
    Cpp,
    Go,
    TypeScript,
}

impl Language {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Rust => "rust",
            Self::Python => "python",
            Self::Java => "java",
            Self::Cpp => "cpp",
            Self::Go => "go",
            Self::TypeScript => "typescript",
        }
    }

    pub fn file_extension(&self) -> &'static str {
        match self {
            Self::Rust => "rs",
            Self::Python => "py",
            Self::Java => "java",
            Self::Cpp => "cpp",
            Self::Go => "go",
            Self::TypeScript => "ts",
        }
    }
}

/// Upstream repository metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repo {
    pub id: Option<String>,
    pub name: String,
    pub url: String,
    pub default_branch: String,
    pub last_commit_sha: String,
    pub etag: Option<String>,
    pub license_spdx: String,
    pub updated_at: DateTime<Utc>,
}

/// Upstream commit audit record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    pub id: Option<String>,
    pub repo_id: String,
    pub commit_sha: String,
    pub processed: bool,
    pub file_deltas: Vec<String>,
    pub committed_at: DateTime<Utc>,
}

/// User profile and attestation reputation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Option<String>,
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub reputation_score: f64,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Canonical algorithm or data structure concept
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Concept {
    pub id: Option<String>,
    pub slug: String,
    pub title: String,
    pub category: String,
    pub time_complexity: Option<String>,
    pub space_complexity: Option<String>,
    pub description: String,
    pub story_metaphor: Option<String>,
    pub guided_inquiry: Option<String>,
    pub status: String, // 'staged' | 'published'
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Snippet kind for specialized vector indexing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SnippetKind {
    Docstring,
    AstBody,
    Signature,
}

impl SnippetKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Docstring => "docstring",
            Self::AstBody => "ast_body",
            Self::Signature => "signature",
        }
    }
}

/// Dedicated snippet vector embedding record (namespaced by model_id)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnippetEmbedding {
    pub id: Option<String>,
    pub concept_id: String,
    pub model_id: String,
    pub language: Language,
    pub snippet_kind: SnippetKind,
    pub embedding: Vec<f32>,
    pub created_at: DateTime<Utc>,
}

/// Canonical language implementation from upstream repositories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Implementation {
    pub id: Option<String>,
    pub language: Language,
    pub source_code: String,
    pub ast_hash: String,
    pub commit_sha: String,
    pub original_authors: Vec<String>,
    pub keywords: Vec<String>,
    pub license_spdx: String,
    pub origin_repo: String,
    pub origin_path: String,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Immutable test execution record signed by system_runner
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestRun {
    pub id: Option<String>,
    pub implementation_id: String,
    pub runner_id: String,
    pub runner_public_key: String,
    pub runner_signature: String,
    pub nonce: String,
    pub exit_reason: ExitReason,
    pub stdout_hash: String,
    pub stderr_hash: String,
    pub cpu_time_ms: u64,
    pub wall_time_ms: u64,
    pub peak_memory_bytes: u64,
    pub test_count: usize,
    pub executed_at: DateTime<Utc>,
}

/// User submission and pedagogical learning session attempt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Submission {
    pub id: Option<String>,
    pub user_id: String,
    pub concept_id: String,
    pub test_run_id: Option<String>,
    pub solution_code: String,
    pub language: Language,
    pub socratic_score: f64,
    pub hints_consumed: u32,
    pub passed: bool,
    pub created_at: DateTime<Utc>,
}
