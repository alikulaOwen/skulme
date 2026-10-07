use chrono::{DateTime, Utc};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};

/// Granular standardized process termination states
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExitReason {
    /// Process exited normally with exit code 0 (All tests passed)
    Ok(i32),
    /// Process exited with a non-zero code indicating test assertion failure
    TestFailure(i32),
    /// Execution exceeded wall-clock limit (3000ms) or CPU limit (2000ms)
    Timeout,
    /// Process was killed by OS/cgroups due to exceeding 256MB RSS (SIGKILL)
    OOMKilled,
    /// Process was terminated unexpectedly by a POSIX signal (SIGSEGV, SIGBUS, SIGFPE, etc.)
    Signal(i32),
}

impl std::fmt::Display for ExitReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok(code) => write!(f, "Ok({code})"),
            Self::TestFailure(code) => write!(f, "TestFailure({code})"),
            Self::Timeout => write!(f, "Timeout (Resource timer kill)"),
            Self::OOMKilled => write!(f, "OOMKilled (Memory limit 256MB exceeded)"),
            Self::Signal(sig) => write!(f, "Terminated by Signal({sig})"),
        }
    }
}

/// Execution telemetry and resource metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionMetrics {
    pub cpu_time_ms: u64,
    pub wall_time_ms: u64,
    pub peak_memory_bytes: u64,
    pub test_count: usize,
}

/// Unforgeable cryptographic sandbox execution attestation proof signed by system_runner
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestationProof {
    pub run_id: String,
    pub implementation_id: String,
    pub code_hash: String,
    pub test_hash: String,
    pub stdout_hash: String,
    pub exit_reason: ExitReason,
    pub timestamp: DateTime<Utc>,
    pub nonce: String,
    pub payload_digest: String,
    pub runner_public_key: String,
    pub runner_signature: String,
}

impl AttestationProof {
    /// Compute the deterministic payload digest
    pub fn compute_payload_digest(
        run_id: &str,
        implementation_id: &str,
        code_hash: &str,
        test_hash: &str,
        stdout_hash: &str,
        exit_reason: &ExitReason,
        timestamp: &DateTime<Utc>,
        nonce: &str,
    ) -> String {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"SKULME_ATTESTATION_PAYLOAD_V2\n");
        hasher.update(run_id.as_bytes());
        hasher.update(b"\n");
        hasher.update(implementation_id.as_bytes());
        hasher.update(b"\n");
        hasher.update(code_hash.as_bytes());
        hasher.update(b"\n");
        hasher.update(test_hash.as_bytes());
        hasher.update(b"\n");
        hasher.update(stdout_hash.as_bytes());
        hasher.update(b"\n");
        hasher.update(format!("{exit_reason}").as_bytes());
        hasher.update(b"\n");
        hasher.update(timestamp.to_rfc3339().as_bytes());
        hasher.update(b"\n");
        hasher.update(nonce.as_bytes());
        hasher.finalize().to_hex().to_string()
    }

    /// Construct and sign an attestation proof using the trusted runner's Ed25519 private key
    pub fn sign(
        run_id: String,
        implementation_id: String,
        code: &str,
        tests: &str,
        stdout: &str,
        exit_reason: ExitReason,
        nonce: String,
        signing_key: &SigningKey,
    ) -> Self {
        let code_hash = blake3::hash(code.as_bytes()).to_hex().to_string();
        let test_hash = blake3::hash(tests.as_bytes()).to_hex().to_string();
        let stdout_hash = blake3::hash(stdout.as_bytes()).to_hex().to_string();
        let timestamp = Utc::now();

        let payload_digest = Self::compute_payload_digest(
            &run_id,
            &implementation_id,
            &code_hash,
            &test_hash,
            &stdout_hash,
            &exit_reason,
            &timestamp,
            &nonce,
        );

        let signature = signing_key.sign(payload_digest.as_bytes());
        let verifying_key = signing_key.verifying_key();

        Self {
            run_id,
            implementation_id,
            code_hash,
            test_hash,
            stdout_hash,
            exit_reason,
            timestamp,
            nonce,
            payload_digest,
            runner_public_key: hex::encode(verifying_key.to_bytes()),
            runner_signature: hex::encode(signature.to_bytes()),
        }
    }

    /// Cryptographically verify the Ed25519 signature
    pub fn verify(&self) -> Result<bool, String> {
        let pk_bytes = hex::decode(&self.runner_public_key)
            .map_err(|e| format!("Invalid public key hex: {e}"))?;
        let pk_arr: [u8; 32] = pk_bytes
            .try_into()
            .map_err(|_| "Public key must be 32 bytes".to_string())?;
        let verifying_key = VerifyingKey::from_bytes(&pk_arr)
            .map_err(|e| format!("Invalid verifying key: {e}"))?;

        let sig_bytes = hex::decode(&self.runner_signature)
            .map_err(|e| format!("Invalid signature hex: {e}"))?;
        let sig_arr: [u8; 64] = sig_bytes
            .try_into()
            .map_err(|_| "Signature must be 64 bytes".to_string())?;
        let signature = Signature::from_bytes(&sig_arr);

        // Recompute expected digest
        let expected_digest = Self::compute_payload_digest(
            &self.run_id,
            &self.implementation_id,
            &self.code_hash,
            &self.test_hash,
            &self.stdout_hash,
            &self.exit_reason,
            &self.timestamp,
            &self.nonce,
        );

        if self.payload_digest != expected_digest {
            return Ok(false);
        }

        Ok(verifying_key.verify(self.payload_digest.as_bytes(), &signature).is_ok())
    }
}

pub mod hex {
    pub fn encode<T: AsRef<[u8]>>(data: T) -> String {
        let bytes = data.as_ref();
        let mut s = String::with_capacity(bytes.len() * 2);
        for &b in bytes {
            use std::fmt::Write;
            let _ = write!(s, "{:02x}", b);
        }
        s
    }

    pub fn decode(hex_str: &str) -> Result<Vec<u8>, String> {
        if hex_str.len() % 2 != 0 {
            return Err("Hex string must have even length".to_string());
        }
        (0..hex_str.len())
            .step_by(2)
            .map(|i| {
                u8::from_str_radix(&hex_str[i..i + 2], 16)
                    .map_err(|e| format!("Invalid hex digit at index {i}: {e}"))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attestation_proof_signing_and_verification() {
        let signing_key = SigningKey::from_bytes(&[42u8; 32]);
        let proof = AttestationProof::sign(
            "run_123".to_string(),
            "impl_456".to_string(),
            "pub fn solve() -> bool { true }",
            "assert!(solve());",
            "test passed\n",
            ExitReason::Ok(0),
            "nonce_789".to_string(),
            &signing_key,
        );

        assert!(proof.verify().expect("Verification should succeed"));
    }

    #[test]
    fn test_attestation_proof_tampering_detected() {
        let signing_key = SigningKey::from_bytes(&[42u8; 32]);
        let mut proof = AttestationProof::sign(
            "run_123".to_string(),
            "impl_456".to_string(),
            "pub fn solve() -> bool { true }",
            "assert!(solve());",
            "test passed\n",
            ExitReason::Ok(0),
            "nonce_789".to_string(),
            &signing_key,
        );

        // Tamper with stdout_hash
        proof.stdout_hash = "tampered_hash".to_string();
        assert!(!proof.verify().expect("Verification should execute"));
    }
}
