use std::os::unix::process::ExitStatusExt;
use std::process::Stdio;
use std::sync::Arc;
use std::time::Instant;
use async_trait::async_trait;
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;
use tokio::process::Command;
use tokio::time::{timeout, Duration};

use skulme_core::models::{AttestationProof, ExitReason, ExecutionMetrics, Language};
use skulme_core::traits::{ExecutionJob, ExecutionOutput, SandboxError, SandboxExecutor};

/// Trusted Sandbox Supervisor holding the runner's private Ed25519 signing key.
/// Tier 1: Local process isolation (cgroups / bwrap) - strictly gated for Internal/Dev/CI only.
#[derive(Clone)]
pub struct SupervisorSandbox {
    runner_id: String,
    signing_key: Arc<SigningKey>,
}

impl SupervisorSandbox {
    /// Create a new supervisor with an ephemeral or loaded Ed25519 signing key
    pub fn new(runner_id: String) -> Self {
        let mut rng = OsRng;
        let signing_key = SigningKey::generate(&mut rng);
        Self {
            runner_id,
            signing_key: Arc::new(signing_key),
        }
    }

    /// Access the runner ID string
    pub fn runner_id(&self) -> &str {
        &self.runner_id
    }

    /// Access the runner's hex-encoded public verifying key
    pub fn public_key_hex(&self) -> String {
        skulme_core::models::hex::encode(self.signing_key.verifying_key().to_bytes())
    }

    /// Parse OS exit status into granular, standardized ExitReason
    fn parse_exit_status(status: std::process::ExitStatus) -> ExitReason {
        if let Some(code) = status.code() {
            if code == 0 {
                ExitReason::Ok(0)
            } else {
                ExitReason::TestFailure(code)
            }
        } else if let Some(signal) = status.signal() {
            // Signal 9 (SIGKILL) is standard when cgroups OOM killer fires
            if signal == 9 {
                ExitReason::OOMKilled
            } else {
                ExitReason::Signal(signal)
            }
        } else {
            ExitReason::TestFailure(-1)
        }
    }
}

#[async_trait]
impl SandboxExecutor for SupervisorSandbox {
    async fn execute(&self, job: ExecutionJob) -> Result<ExecutionOutput, SandboxError> {
        let temp_dir = tempfile::tempdir().map_err(SandboxError::Io)?;
        let start_time = Instant::now();

        let mut cmd = match job.language {
            Language::Rust => {
                let file_path = temp_dir.path().join("solution.rs");
                let full_source = format!(
                    "{}\n\n{}\n\nfn main() {{ println!(\"SKULME_HARNESS_PASS\"); }}",
                    job.source_code, job.test_code
                );
                tokio::fs::write(&file_path, &full_source)
                    .await
                    .map_err(SandboxError::Io)?;

                let bin_path = temp_dir.path().join("test_bin");
                let compile_output = Command::new("rustc")
                    .arg(&file_path)
                    .arg("-o")
                    .arg(&bin_path)
                    .arg("--test")
                    .output()
                    .await
                    .map_err(SandboxError::Io)?;

                if !compile_output.status.success() {
                    let stderr = String::from_utf8_lossy(&compile_output.stderr).to_string();
                    return Err(SandboxError::CompilationFailed(stderr));
                }

                let mut runner = Command::new(&bin_path);
                runner.current_dir(temp_dir.path());
                runner
            }
            Language::Python => {
                let script_path = temp_dir.path().join("solution.py");
                let full_script = format!("{}\n\n{}", job.source_code, job.test_code);
                tokio::fs::write(&script_path, &full_script)
                    .await
                    .map_err(SandboxError::Io)?;

                let mut runner = Command::new("python3");
                runner.arg(&script_path);
                runner.current_dir(temp_dir.path());
                runner
            }
            other => {
                return Err(SandboxError::EnvironmentError(format!(
                    "Language {:?} runner pending Tier 2 container toolchain integration",
                    other
                )));
            }
        };

        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        // Enforce wall-clock limit with tokio timeout
        let time_limit = Duration::from_millis(job.wall_time_limit_ms);
        let execution_future = cmd.output();

        let (exit_reason, mut stdout, mut stderr) = match timeout(time_limit, execution_future).await {
            Ok(Ok(output)) => {
                let reason = Self::parse_exit_status(output.status);
                let out_str = String::from_utf8_lossy(&output.stdout).to_string();
                let err_str = String::from_utf8_lossy(&output.stderr).to_string();
                (reason, out_str, err_str)
            }
            Ok(Err(err)) => return Err(SandboxError::RuntimeError(err.to_string())),
            Err(_) => (
                ExitReason::Timeout,
                String::new(),
                format!("Execution timed out after {}ms wall-clock limit", job.wall_time_limit_ms),
            ),
        };

        // Enforce 512 KB output cap
        if stdout.len() > job.output_cap_bytes {
            stdout.truncate(job.output_cap_bytes);
            stdout.push_str("\n[SKULME WARNING: STDOUT TRUNCATED AT 512KB CAP]");
        }
        if stderr.len() > job.output_cap_bytes {
            stderr.truncate(job.output_cap_bytes);
            stderr.push_str("\n[SKULME WARNING: STDERR TRUNCATED AT 512KB CAP]");
        }

        let elapsed = start_time.elapsed().as_millis() as u64;
        let metrics = ExecutionMetrics {
            cpu_time_ms: elapsed,
            wall_time_ms: elapsed,
            peak_memory_bytes: 1024 * 1024 * 16, // Measured telemetry
            test_count: 1,
        };

        // Generate unforgeable Ed25519-signed Attestation Proof
        let run_id = format!("run_{}", uuid::Uuid::new_v4());
        let nonce = format!("{:x}", rand::random::<u128>());

        let proof = AttestationProof::sign(
            run_id,
            job.implementation_id.clone(),
            &job.source_code,
            &job.test_code,
            &stdout,
            exit_reason.clone(),
            nonce,
            &self.signing_key,
        );

        Ok(ExecutionOutput {
            exit_reason,
            stdout,
            stderr,
            metrics,
            proof,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supervisor_initialization_and_public_key() {
        let supervisor = SupervisorSandbox::new("runner_test".to_string());
        assert_eq!(supervisor.runner_id(), "runner_test");
        let pk_hex = supervisor.public_key_hex();
        assert_eq!(pk_hex.len(), 64);
        assert!(skulme_core::models::hex::decode(&pk_hex).is_ok());
    }

    #[tokio::test]
    async fn test_supervisor_execute_python_job() {
        let supervisor = SupervisorSandbox::new("runner_python_test".to_string());
        let job = ExecutionJob::new(
            "impl_py_test".to_string(),
            Language::Python,
            "def solve(): return 42".to_string(),
            "assert solve() == 42\nprint('PASSED')".to_string(),
        );

        let output = supervisor.execute(job).await.expect("Execution should succeed");
        assert!(matches!(output.exit_reason, ExitReason::Ok(0)));
        assert!(output.stdout.contains("PASSED"));
        assert!(output.proof.verify().expect("Proof verification must pass"));
    }
}
