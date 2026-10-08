use std::path::PathBuf;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use skulme_db::SkulmeDb;
use skulme_ingestion::DynamicRepoImporter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let db_url = std::env::var("SURREALDB_URL").unwrap_or_else(|_| "ws://localhost:8000".to_string());
    let repo_dir = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));

    info!("Connecting to SurrealDB at {db_url}...");
    let db = SkulmeDb::connect(&db_url).await?;
    info!("Running database migrations...");
    db.migrate().await?;

    info!("Starting dynamic repository scan and ingestion from {:?}...", repo_dir);
    let importer = DynamicRepoImporter::new();
    let stats = importer.import_repository(&db, &repo_dir).await?;

    info!(
        "Dynamic import completed successfully in {} ms!\n- Repositories: {}\n- Concepts: {}\n- Implementations: {}\n- Relationships: {}",
        stats.duration_ms,
        stats.repos_registered,
        stats.concepts_imported,
        stats.implementations_imported,
        stats.relationships_created
    );

    Ok(())
}

