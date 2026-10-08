use std::fs;
use std::path::Path;
use std::time::Instant;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::info;
use skulme_db::SkulmeDb;

#[derive(Debug, Error)]
pub enum IngestionError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Database error: {0}")]
    Db(String),

    #[error("Path not found: {0}")]
    NotFound(String),
}

/// Delta from an upstream repository push or poll cycle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitCommitDelta {
    pub repo_name: String,
    pub commit_sha: String,
    pub modified_files: Vec<String>,
    pub added_files: Vec<String>,
}

/// Scanned problem representation extracted dynamically from disk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScannedProblem {
    pub slug: String,
    pub category: String,
    pub title: String,
    pub difficulty: String,
    pub description: String,
    pub implementations: Vec<ScannedImplementation>,
}

/// Scanned language implementation extracted dynamically from disk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScannedImplementation {
    pub language: String,
    pub relative_path: String,
    pub source_code: String,
    pub ast_hash: String,
}

/// Metadata item from seed_problems.json if present
#[derive(Debug, Clone, Deserialize)]
struct ProblemIndexEntry {
    pub slug: String,
    pub category: String,
    pub title: String,
    pub difficulty: String,
    pub path: String,
}

/// Telemetry metrics returned after a dynamic ingestion run
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IngestionStats {
    pub repos_registered: usize,
    pub concepts_imported: usize,
    pub implementations_imported: usize,
    pub relationships_created: usize,
    pub duration_ms: u64,
}

/// Dynamic repository scanner and importer for Skul.me polyglot problem sets
#[derive(Debug, Clone)]
pub struct DynamicRepoImporter {
    pub commit_sha: String,
    pub license_spdx: String,
}

impl Default for DynamicRepoImporter {
    fn default() -> Self {
        Self {
            commit_sha: "HEAD".to_string(),
            license_spdx: "MIT".to_string(),
        }
    }
}

impl DynamicRepoImporter {
    pub fn new() -> Self {
        Self::default()
    }

    /// Dynamically scans problems and implementations from the given root repository directory
    pub fn scan_directory(&self, root_path: &Path) -> Result<Vec<ScannedProblem>, IngestionError> {
        let problems_base = if root_path.join("problems").is_dir() {
            root_path.join("problems")
        } else if root_path.is_dir() && root_path.file_name().map_or(false, |n| n == "problems") {
            root_path.to_path_buf()
        } else {
            root_path.to_path_buf()
        };

        let seed_json_path = root_path.join("database/seed_problems.json");

        if seed_json_path.exists() {
            self.scan_from_seed_json(&seed_json_path, root_path)
        } else {
            self.scan_by_walking(&problems_base)
        }
    }

    /// Scans problems guided by seed_problems.json index
    fn scan_from_seed_json(&self, json_path: &Path, root_path: &Path) -> Result<Vec<ScannedProblem>, IngestionError> {
        let content = fs::read_to_string(json_path)?;
        let entries: Vec<ProblemIndexEntry> = serde_json::from_str(&content)?;

        let mut results = Vec::with_capacity(entries.len());

        for entry in entries {
            let prob_dir = root_path.join("problems").join(&entry.path);
            let fallback_dir = root_path.join(&entry.path);
            let dir = if prob_dir.exists() {
                prob_dir
            } else if fallback_dir.exists() {
                fallback_dir
            } else {
                continue;
            };

            let readme_file = dir.join("README.md");
            let description = if readme_file.exists() {
                fs::read_to_string(&readme_file).unwrap_or_default()
            } else {
                entry.title.clone()
            };

            let implementations = self.scan_implementations(&dir, &entry.path);
            let full_slug = format!("{}_{}", entry.category, entry.slug);

            results.push(ScannedProblem {
                slug: full_slug,
                category: entry.category,
                title: entry.title,
                difficulty: entry.difficulty,
                description,
                implementations,
            });
        }

        Ok(results)
    }

    /// Dynamically walks directory structure if seed_problems.json is absent
    fn scan_by_walking(&self, problems_dir: &Path) -> Result<Vec<ScannedProblem>, IngestionError> {
        if !problems_dir.exists() {
            return Err(IngestionError::NotFound(problems_dir.to_string_lossy().to_string()));
        }

        let mut results = Vec::new();
        let cat_entries = fs::read_dir(problems_dir)?;

        for cat_res in cat_entries {
            let cat_entry = cat_res?;
            let cat_path = cat_entry.path();
            if !cat_path.is_dir() {
                continue;
            }

            let cat_name = cat_entry.file_name().to_string_lossy().to_string();
            let prob_entries = fs::read_dir(&cat_path)?;

            for prob_res in prob_entries {
                let prob_entry = prob_res?;
                let prob_path = prob_entry.path();
                if !prob_path.is_dir() {
                    continue;
                }

                let prob_name = prob_entry.file_name().to_string_lossy().to_string();
                let readme_path = prob_path.join("README.md");
                if !readme_path.exists() {
                    continue;
                }

                let description = fs::read_to_string(&readme_path).unwrap_or_default();
                let title = parse_title_from_readme(&description).unwrap_or_else(|| {
                    prob_name.replace('_', " ").chars().fold(String::new(), |mut acc, c| {
                        if acc.is_empty() || acc.ends_with(' ') {
                            acc.push(c.to_ascii_uppercase());
                        } else {
                            acc.push(c);
                        }
                        acc
                    })
                });

                let rel_base = format!("{cat_name}/{prob_name}");
                let implementations = self.scan_implementations(&prob_path, &rel_base);
                let slug = format!("{cat_name}_{prob_name}");

                results.push(ScannedProblem {
                    slug,
                    category: cat_name.clone(),
                    title,
                    difficulty: "Medium".to_string(),
                    description,
                    implementations,
                });
            }
        }

        results.sort_by(|a, b| a.slug.cmp(&b.slug));
        Ok(results)
    }

    /// Discovers language implementation files in a problem directory
    fn scan_implementations(&self, prob_dir: &Path, rel_base: &str) -> Vec<ScannedImplementation> {
        let candidates: &[(&str, &[&str])] = &[
            ("rust", &["solution.rs", "src/solution.rs"]),
            ("python", &["solution.py"]),
            ("java", &["Solution.java"]),
            ("typescript", &["solution.ts"]),
            ("cpp", &["solution.cpp"]),
            ("go", &["solution.go"]),
            ("sql", &["solution.sql"]),
        ];

        let mut impls = Vec::new();

        for &(lang, filenames) in candidates {
            for &filename in filenames {
                let file_path = prob_dir.join(filename);
                if file_path.exists() && file_path.is_file() {
                    if let Ok(code) = fs::read_to_string(&file_path) {
                        let hash = blake3::hash(code.as_bytes()).to_hex().to_string();
                        let rel_path = format!("problems/{rel_base}/{filename}");
                        impls.push(ScannedImplementation {
                            language: lang.to_string(),
                            relative_path: rel_path,
                            source_code: code,
                            ast_hash: hash,
                        });
                        break;
                    }
                }
            }
        }

        impls
    }

    /// Dynamically imports all discovered repositories, concepts, and implementations into SurrealDB
    pub async fn import_repository(&self, db: &SkulmeDb, repo_root: &Path) -> Result<IngestionStats, IngestionError> {
        let start = Instant::now();
        let mut stats = IngestionStats::default();

        // 1. Dynamically register upstream repositories
        let repos = [
            ("rust", "Rust", "https://github.com/TheAlgorithms/Rust"),
            ("python", "Python", "https://github.com/TheAlgorithms/Python"),
            ("java", "Java", "https://github.com/TheAlgorithms/Java"),
            ("typescript", "TypeScript", "https://github.com/TheAlgorithms/TypeScript"),
            ("cpp", "C-Plus-Plus", "https://github.com/TheAlgorithms/C-Plus-Plus"),
            ("go", "Go", "https://github.com/TheAlgorithms/Go"),
        ];

        for (id, name, url) in repos {
            let mut res = db.raw().query(
                "UPSERT type::thing('repo', $id) MERGE {
                    name: $name,
                    url: $url,
                    default_branch: 'master',
                    last_commit_sha: $sha,
                    license_spdx: $license,
                    updated_at: time::now()
                };"
            )
            .bind(("id", id.to_string()))
            .bind(("name", name.to_string()))
            .bind(("url", url.to_string()))
            .bind(("sha", self.commit_sha.clone()))
            .bind(("license", self.license_spdx.clone()))
            .await
            .map_err(|e| IngestionError::Db(e.to_string()))?;

            let errors = res.take_errors();
            if !errors.is_empty() {
                return Err(IngestionError::Db(format!("Repo registration failed: {errors:?}")));
            }
            stats.repos_registered += 1;
        }

        // 2. Dynamically scan problems from disk
        let problems = self.scan_directory(repo_root)?;
        info!("Dynamic scanner discovered {} problems", problems.len());

        // 3. Batch import concepts, implementations, and relation edges
        for prob in &problems {
            // Upsert Concept
            let mut res = db.raw().query(
                "UPSERT type::thing('concept', $slug) MERGE {
                    slug: $slug,
                    title: $title,
                    category: $category,
                    description: $desc,
                    status: 'published'
                };"
            )
            .bind(("slug", prob.slug.clone()))
            .bind(("title", prob.title.clone()))
            .bind(("category", prob.category.clone()))
            .bind(("desc", prob.description.clone()))
            .await
            .map_err(|e| IngestionError::Db(e.to_string()))?;

            let errors = res.take_errors();
            if !errors.is_empty() {
                return Err(IngestionError::Db(format!("Concept insert failed: {errors:?}")));
            }
            stats.concepts_imported += 1;

            // Upsert Implementations & Relations
            for imp in &prob.implementations {
                let impl_id = format!("{}_{}", prob.slug, imp.language);
                let origin_repo = format!("repo:{}", imp.language);
                let keywords = vec![prob.category.clone(), prob.slug.clone(), imp.language.clone()];

                let relate_stmt = format!("RELATE implementation:{impl_id}->IMPLEMENTS->concept:{};", prob.slug);
                let query = format!(
                    "UPSERT type::thing('implementation', $impl_id) MERGE {{
                        language: $lang,
                        source_code: $code,
                        ast_hash: $hash,
                        commit_sha: $sha,
                        original_authors: ['TheAlgorithms Community'],
                        keywords: $keywords,
                        license_spdx: $license,
                        origin_repo: $repo,
                        origin_path: $path
                    }};
                    {relate_stmt}"
                );

                let mut imp_res = db.raw().query(&query)
                    .bind(("impl_id", impl_id))
                    .bind(("lang", imp.language.clone()))
                    .bind(("code", imp.source_code.clone()))
                    .bind(("hash", imp.ast_hash.clone()))
                    .bind(("sha", self.commit_sha.clone()))
                    .bind(("keywords", keywords))
                    .bind(("license", self.license_spdx.clone()))
                    .bind(("repo", origin_repo))
                    .bind(("path", imp.relative_path.clone()))
                    .await
                    .map_err(|e| IngestionError::Db(e.to_string()))?;

                let imp_errors = imp_res.take_errors();
                if !imp_errors.is_empty() {
                    return Err(IngestionError::Db(format!("Implementation insert failed: {imp_errors:?}")));
                }

                stats.implementations_imported += 1;
                stats.relationships_created += 1;
            }
        }

        stats.duration_ms = start.elapsed().as_millis() as u64;
        info!(
            "Dynamic ingestion completed in {} ms: {} concepts, {} implementations",
            stats.duration_ms, stats.concepts_imported, stats.implementations_imported
        );

        Ok(stats)
    }
}

/// Helper function to parse first markdown title from README.md
fn parse_title_from_readme(readme: &str) -> Option<String> {
    for line in readme.lines() {
        let trimmed = line.trim();
        if let Some(stripped) = trimmed.strip_prefix('#') {
            let title = stripped.trim_start_matches('#').trim();
            if !title.is_empty() {
                return Some(title.to_string());
            }
        }
    }
    None
}

/// Hybrid Git syncer supporting multi-repository scanning and dynamic ingestion
pub struct MultiRepoSyncer {
    pub repositories: Vec<String>,
    pub importer: DynamicRepoImporter,
}

impl MultiRepoSyncer {
    pub fn new(repositories: Vec<String>) -> Self {
        Self {
            repositories,
            importer: DynamicRepoImporter::default(),
        }
    }

    pub async fn sync_all(&self, db: &SkulmeDb, workspace_root: &Path) -> Result<IngestionStats, IngestionError> {
        self.importer.import_repository(db, workspace_root).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_parse_title_from_readme() {
        let md = "# The All-Star Squad Selection\nSome text...";
        assert_eq!(parse_title_from_readme(md).as_deref(), Some("The All-Star Squad Selection"));

        let md2 = "## Secondary\n### Tertiary";
        assert_eq!(parse_title_from_readme(md2).as_deref(), Some("Secondary"));
    }

    #[test]
    fn test_dynamic_scan_existing_problem() {
        let importer = DynamicRepoImporter::new();
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let problems = importer.scan_directory(&root).expect("Failed to scan problems directory");
        assert!(!problems.is_empty(), "Scanner should find problems in repository");

        // Verify that backtracking problem has multi-language implementations
        let sample = problems.iter().find(|p| p.slug.contains("all_combination_of_size_k"));
        assert!(sample.is_some(), "Should find all_combination_of_size_k problem");
        let sample = sample.unwrap();
        assert!(!sample.implementations.is_empty(), "Should discover implementations");
        assert!(sample.implementations.iter().any(|i| i.language == "python" || i.language == "java" || i.language == "rust"));
    }

    #[tokio::test]
    async fn test_dynamic_import_sample_into_memory_db() {
        let db = SkulmeDb::connect_memory().await.expect("Failed to connect");
        db.migrate().await.expect("Failed to migrate");

        let importer = DynamicRepoImporter::new();
        // Create a temporary simulated problem directory structure to test ingestion without importing all 394 in unit test
        let temp_dir = std::env::temp_dir().join(format!("skulme_test_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        let prob_dir = temp_dir.join("problems/math/fibonacci");
        fs::create_dir_all(&prob_dir).unwrap();
        fs::write(prob_dir.join("README.md"), "# Fibonacci Numbers\nCalculate nth fibonacci").unwrap();
        fs::write(prob_dir.join("solution.rs"), "pub fn fib(n: u32) -> u32 { 0 }").unwrap();
        fs::write(prob_dir.join("solution.py"), "def fib(n: int) -> int: return 0").unwrap();

        let stats = importer.import_repository(&db, &temp_dir).await.expect("Dynamic import failed");
        assert_eq!(stats.concepts_imported, 1);
        assert_eq!(stats.implementations_imported, 2);
        assert_eq!(stats.relationships_created, 2);

        // Verify SurrealDB query returns the dynamically imported concept
        let mut query = db.raw().query("SELECT title, slug FROM concept WHERE slug = 'math_fibonacci'").await.unwrap();
        let concepts: Vec<serde_json::Value> = query.take(0).unwrap();
        assert_eq!(concepts.len(), 1);
        assert_eq!(concepts[0]["title"], "Fibonacci Numbers");

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
