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
}
