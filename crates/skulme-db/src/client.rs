use std::sync::Arc;
use surrealdb::engine::any::Any;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use thiserror::Error;

use crate::schema::SCHEMA_SURREALQL;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("SurrealDB error: {0}")]
    Surreal(#[from] surrealdb::Error),

    #[error("Migration error: {0}")]
    Migration(String),

    #[error("Record not found: {0}")]
    NotFound(String),
}

/// Managed SurrealDB client supporting memory and distributed backends
#[derive(Clone)]
pub struct SkulmeDb {
    db: Arc<Surreal<Any>>,
}

impl SkulmeDb {
    /// Connect to SurrealDB instance (e.g. "ws://localhost:8000" or "mem://")
    pub async fn connect(endpoint: &str) -> Result<Self, DbError> {
        let db = surrealdb::engine::any::connect(endpoint).await?;
        
        if endpoint.starts_with("ws://") || endpoint.starts_with("http://") {
            let _ = db.signin(Root {
                username: "root",
                password: "rootpassword",
            }).await;
        }

        db.use_ns("skulme").use_db("skulme_core").await?;

        Ok(Self { db: Arc::new(db) })
    }

    /// Connect to an in-memory database for testing and local verification
    pub async fn connect_memory() -> Result<Self, DbError> {
        Self::connect("mem://").await
    }

    /// Execute the schema DDL migrations
    pub async fn migrate(&self) -> Result<(), DbError> {
        self.db.query(SCHEMA_SURREALQL).await?;
        Ok(())
    }

    /// Access the raw SurrealDB handle for custom graph/vector queries
    pub fn raw(&self) -> &Surreal<Any> {
        &self.db
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_in_memory_connection_and_migration() {
        let db = SkulmeDb::connect_memory().await.expect("Failed to connect to in-memory SurrealDB");
        db.migrate().await.expect("Failed to execute SurrealDB schema migrations");
    }

    #[tokio::test]
    async fn test_create_and_query_concept() {
        let db = SkulmeDb::connect_memory().await.expect("Failed to connect");
        db.migrate().await.expect("Failed to migrate");

        let query_str = format!(
            "CREATE type::thing('concept', $slug) SET slug = $slug, title = $title, category = $category, description = $desc, status = 'published';
             CREATE type::thing('implementation', $impl_id) SET language = 'rust', source_code = 'fn main(){{}}', ast_hash = 'h1', commit_sha = 'c1', original_authors = ['test'], keywords = ['k1'], license_spdx = 'MIT', origin_repo = 'repo:rust', origin_path = 'src/test.rs';
             RELATE implementation:binary_search_rust->IMPLEMENTS->concept:binary_search;"
        );
        let mut res = db.raw().query(&query_str)
        .bind(("slug", "binary_search"))
        .bind(("title", "Binary Search"))
        .bind(("category", "searching"))
        .bind(("desc", "Search algorithm"))
        .bind(("impl_id", "binary_search_rust"))
        .await
        .expect("Failed to create concept and implementation");

        let errors = res.take_errors();
        assert!(errors.is_empty(), "SurrealDB query errors: {:?}", errors);
    }
}
