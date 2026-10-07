use async_trait::async_trait;
use thiserror::Error;

use crate::models::{AttestationProof, ExitReason, ExecutionMetrics, Language};

#[derive(Debug, Error)]
pub enum SandboxError {
    #[error("Compilation error: {0}")]
    CompilationFailed(String),

    #[error("Sandbox environment failure: {0}")]
    EnvironmentError(String),

    #[error("Execution runtime error: {0}")]
    RuntimeError(String),

    #[error("Execution timeout")]
    Timeout,

    #[error("Attestation signing failure: {0}")]
    SigningFailed(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Execution job with strict research resource limits
#[derive(Debug, Clone)]
pub struct ExecutionJob {
    pub implementation_id: String,
    pub language: Language,
    pub source_code: String,
    pub test_code: String,
    pub wall_time_limit_ms: u64,
    pub cpu_limit_ms: u64,
    pub memory_limit_bytes: u64,
    pub pids_max: u32,
    pub output_cap_bytes: usize,
}

impl ExecutionJob {
    pub fn new(implementation_id: String, language: Language, source_code: String, test_code: String) -> Self {
        Self {
            implementation_id,
            language,
            source_code,
            test_code,
            wall_time_limit_ms: 3000,              // 3000ms wall-clock limit
            cpu_limit_ms: 2000,                    // 2000ms CPU limit
            memory_limit_bytes: 256 * 1024 * 1024, // 256 MB RSS
            pids_max: 32,                          // Max 32 threads/processes
            output_cap_bytes: 512 * 1024,          // 512 KB output cap
        }
    }
}

/// Standardized output returned from sandbox runner
#[derive(Debug, Clone)]
pub struct ExecutionOutput {
    pub exit_reason: ExitReason,
    pub stdout: String,
    pub stderr: String,
    pub metrics: ExecutionMetrics,
    pub proof: AttestationProof,
}

/// Unified trait implemented by supervisor runner
#[async_trait]
pub trait SandboxExecutor: Send + Sync {
    async fn execute(&self, job: ExecutionJob) -> Result<ExecutionOutput, SandboxError>;
}
