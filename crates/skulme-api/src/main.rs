use std::net::SocketAddr;
use std::sync::Arc;
use axum::{
    extract::{Path, State, WebSocketUpgrade},
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use skulme_core::models::Language;
use skulme_core::traits::{ExecutionJob, SandboxExecutor};
use skulme_db::SkulmeDb;
use skulme_sandbox::SupervisorSandbox;

#[derive(Clone)]
struct AppState {
    #[allow(dead_code)]
    db: SkulmeDb,
    supervisor: Arc<SupervisorSandbox>,
}

#[derive(Debug, Deserialize)]
struct ExecuteRequest {
    implementation_id: String,
    language: String,
    source_code: String,
    test_code: String,
}

#[derive(Debug, Serialize)]
struct ExecuteResponse {
    passed: bool,
    exit_reason: String,
    stdout: String,
    stderr: String,
    execution_time_ms: u64,
    payload_digest: String,
    runner_public_key: String,
    runner_signature: String,
    signature_verified: bool,
}

async fn health_check() -> &'static str {
    "skulme-api ok"
}

async fn execute_code(
    State(state): State<AppState>,
    Json(payload): Json<ExecuteRequest>,
) -> Result<Json<ExecuteResponse>, (axum::http::StatusCode, String)> {
    let lang = match payload.language.to_lowercase().as_str() {
        "rust" => Language::Rust,
        "python" => Language::Python,
        other => {
            return Err((
                axum::http::StatusCode::BAD_REQUEST,
                format!("Language {other} is not yet supported in local supervisor"),
            ));
        }
    };

    let job = ExecutionJob::new(
        payload.implementation_id,
        lang,
        payload.source_code,
        payload.test_code,
    );

    let output = state
        .supervisor
        .execute(job)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let is_ok = matches!(output.exit_reason, skulme_core::models::ExitReason::Ok(_));
    let signature_verified = output.proof.verify().unwrap_or(false);

    Ok(Json(ExecuteResponse {
        passed: is_ok,
        exit_reason: format!("{}", output.exit_reason),
        stdout: output.stdout,
        stderr: output.stderr,
        execution_time_ms: output.metrics.wall_time_ms,
        payload_digest: output.proof.payload_digest,
        runner_public_key: output.proof.runner_public_key,
        runner_signature: output.proof.runner_signature,
        signature_verified,
    }))
}

async fn ws_session_handler(
    ws: WebSocketUpgrade,
    Path(session_id): Path<String>,
    State(_state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |mut socket| async move {
        info!("WebSocket connection established for session: {session_id}");
        use axum::extract::ws::Message;
        let _ = socket
            .send(Message::Text(
                format!("{{\"event\": \"connected\", \"session\": \"{session_id}\"}}").into(),
            ))
            .await;
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemSummary {
    pub slug: String,
    pub title: String,
    pub category: String,
    pub difficulty: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemDetail {
    pub slug: String,
    pub title: String,
    pub category: String,
    pub difficulty: String,
    pub description: String,
    pub time_complexity: String,
    pub space_complexity: String,
    pub templates: std::collections::HashMap<String, String>,
}

fn canonical_problems() -> Vec<ProblemDetail> {
    vec![
        ProblemDetail {
            slug: "two-sum".to_string(),
            title: "Two Sum".to_string(),
            category: "Arrays & Hash Maps".to_string(),
            difficulty: "Easy".to_string(),
            description: "Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.".to_string(),
            time_complexity: "O(N)".to_string(),
            space_complexity: "O(N)".to_string(),
            templates: [
                ("rust".to_string(), "pub fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {\n    todo!()\n}".to_string()),
                ("python".to_string(), "def two_sum(nums: list[int], target: int) -> list[int]:\n    pass".to_string()),
            ].into_iter().collect(),
        },
        ProblemDetail {
            slug: "binary-search".to_string(),
            title: "Binary Search".to_string(),
            category: "Binary Search".to_string(),
            difficulty: "Easy".to_string(),
            description: "Given a sorted array of integers nums and an integer target, return the index if target exists, else -1.".to_string(),
            time_complexity: "O(log N)".to_string(),
            space_complexity: "O(1)".to_string(),
            templates: [
                ("rust".to_string(), "pub fn search(nums: &[i32], target: i32) -> Option<usize> {\n    todo!()\n}".to_string()),
                ("python".to_string(), "def search(nums: list[int], target: int) -> int:\n    pass".to_string()),
            ].into_iter().collect(),
        },
        ProblemDetail {
            slug: "coin-change".to_string(),
            title: "Coin Change".to_string(),
            category: "Dynamic Programming".to_string(),
            difficulty: "Medium".to_string(),
            description: "Return the fewest number of coins needed to make up the given amount, or -1 if impossible.".to_string(),
            time_complexity: "O(S * N)".to_string(),
            space_complexity: "O(S)".to_string(),
            templates: [
                ("rust".to_string(), "pub fn coin_change(coins: &[i32], amount: i32) -> i32 {\n    todo!()\n}".to_string()),
                ("python".to_string(), "def coin_change(coins: list[int], amount: int) -> int:\n    pass".to_string()),
            ].into_iter().collect(),
        },
    ]
}

async fn list_problems() -> Json<Vec<ProblemSummary>> {
    let summaries = canonical_problems()
        .into_iter()
        .map(|p| ProblemSummary {
            slug: p.slug,
            title: p.title,
            category: p.category,
            difficulty: p.difficulty,
        })
        .collect();
    Json(summaries)
}

async fn get_problem(
    Path(slug): Path<String>,
) -> Result<Json<ProblemDetail>, (axum::http::StatusCode, String)> {
    canonical_problems()
        .into_iter()
        .find(|p| p.slug == slug)
        .map(Json)
        .ok_or((axum::http::StatusCode::NOT_FOUND, format!("Problem '{slug}' not found")))
}

async fn sync_repository(
    State(state): State<AppState>,
) -> Result<Json<skulme_ingestion::IngestionStats>, (axum::http::StatusCode, String)> {
    let importer = skulme_ingestion::DynamicRepoImporter::new();
    let stats = importer
        .import_repository(&state.db, std::path::Path::new("."))
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(stats))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let db = SkulmeDb::connect_memory().await?;
    db.migrate().await?;
    info!("SurrealDB memory schema and permissions migrated successfully");

    // Dynamically ingest problems from the repository
    let importer = skulme_ingestion::DynamicRepoImporter::new();
    if let Ok(stats) = importer.import_repository(&db, std::path::Path::new(".")).await {
        info!(
            "Dynamically ingested {} concepts and {} implementations into SurrealDB in {} ms",
            stats.concepts_imported, stats.implementations_imported, stats.duration_ms
        );
    }

    let supervisor = Arc::new(SupervisorSandbox::new("runner_local_supervisor".to_string()));
    info!("Supervisor initialized with verifying key: {}", supervisor.public_key_hex());

    let state = AppState { db, supervisor };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/sync", post(sync_repository))
        .route("/api/v1/problems", get(list_problems))
        .route("/api/v1/problems/{slug}", get(get_problem))
        .route("/api/v1/sandbox/execute", post(execute_code))
        .route("/ws/session/{session_id}", get(ws_session_handler))
        .layer(cors)
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("Skul.me API Gateway listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check() {
        assert_eq!(health_check().await, "skulme-api ok");
    }

    #[tokio::test]
    async fn test_list_problems() {
        let problems = list_problems().await;
        assert!(!problems.0.is_empty());
        assert_eq!(problems.0[0].slug, "two-sum");
    }

    #[tokio::test]
    async fn test_get_problem() {
        let res = get_problem(Path("two-sum".to_string())).await;
        assert!(res.is_ok());
        let problem = res.unwrap().0;
        assert_eq!(problem.title, "Two Sum");

        let err = get_problem(Path("non-existent".to_string())).await;
        assert!(err.is_err());
    }

    #[tokio::test]
    async fn test_execute_endpoint() {
        let db = SkulmeDb::connect_memory().await.unwrap();
        let supervisor = Arc::new(SupervisorSandbox::new("test_api_runner".to_string()));
        let state = AppState { db, supervisor };

        let payload = ExecuteRequest {
            implementation_id: "test_impl_1".to_string(),
            language: "python".to_string(),
            source_code: "def solve(): return 42".to_string(),
            test_code: "assert solve() == 42\nprint('PYTHON_API_TEST_OK')".to_string(),
        };

        let response = execute_code(State(state), Json(payload)).await.unwrap();
        assert!(response.0.passed);
        assert!(response.0.signature_verified);
        assert!(response.0.stdout.contains("PYTHON_API_TEST_OK"));
    }
}
