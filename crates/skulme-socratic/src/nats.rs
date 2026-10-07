use serde::{Deserialize, Serialize};

/// Session subject routing for NATS JetStream
pub struct SessionSubjects;

impl SessionSubjects {
    /// Dedicated subject for buffered Socratic token hints
    pub fn hints(session_id: &str) -> String {
        format!("skulme.session.{session_id}.hints")
    }

    /// Dedicated subject for buffered execution stdout/stderr and telemetry
    pub fn executions(session_id: &str) -> String {
        format!("skulme.session.{session_id}.executions")
    }
}

/// Token or structured message streamed via NATS hints subject
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocraticHintChunk {
    pub session_id: String,
    pub chunk_index: usize,
    pub content: String,
    pub is_final: bool,
}
