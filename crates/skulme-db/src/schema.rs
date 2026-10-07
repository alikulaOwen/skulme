/// Fully restored permissioned SurrealQL DDL for Skul.me multi-model database
pub const SCHEMA_SURREALQL: &str = r#"
-- Define Namespaces and Databases
DEFINE NAMESPACE IF NOT EXISTS skulme;
USE NAMESPACE skulme;
DEFINE DATABASE IF NOT EXISTS skulme_core;
USE DATABASE skulme_core;

-- Upstream Repository Tracking (ETag, Commit Hash, Licensing)
DEFINE TABLE IF NOT EXISTS repo SCHEMAFULL
    PERMISSIONS
        FOR select FULL
        FOR create, update, delete WHERE $auth.role = 'admin';

DEFINE FIELD IF NOT EXISTS name ON TABLE repo TYPE string;
DEFINE FIELD IF NOT EXISTS url ON TABLE repo TYPE string;
DEFINE FIELD IF NOT EXISTS default_branch ON TABLE repo TYPE string;
DEFINE FIELD IF NOT EXISTS last_commit_sha ON TABLE repo TYPE string;
DEFINE FIELD IF NOT EXISTS etag ON TABLE repo TYPE option<string>;
DEFINE FIELD IF NOT EXISTS license_spdx ON TABLE repo TYPE string;
DEFINE FIELD IF NOT EXISTS updated_at ON TABLE repo TYPE datetime DEFAULT time::now();

-- Upstream Commit Audit Log
DEFINE TABLE IF NOT EXISTS commit SCHEMAFULL
    PERMISSIONS
        FOR select FULL
        FOR create, update, delete WHERE $auth.role = 'admin';

DEFINE FIELD IF NOT EXISTS repo_id ON TABLE commit TYPE record<repo>;
DEFINE FIELD IF NOT EXISTS commit_sha ON TABLE commit TYPE string;
DEFINE FIELD IF NOT EXISTS processed ON TABLE commit TYPE bool DEFAULT false;
DEFINE FIELD IF NOT EXISTS file_deltas ON TABLE commit TYPE array<string>;
DEFINE FIELD IF NOT EXISTS committed_at ON TABLE commit TYPE datetime;

-- User Profile & Reputation (Bootstrap starting at 1.0)
DEFINE TABLE IF NOT EXISTS user SCHEMAFULL
    PERMISSIONS
        FOR select FULL
        FOR create FULL
        FOR update WHERE $auth.id = id OR $auth.role = 'admin'
        FOR delete WHERE $auth.role = 'admin';

DEFINE FIELD IF NOT EXISTS username ON TABLE user TYPE string;
DEFINE FIELD IF NOT EXISTS display_name ON TABLE user TYPE string;
DEFINE FIELD IF NOT EXISTS email ON TABLE user TYPE string;
DEFINE FIELD IF NOT EXISTS reputation_score ON TABLE user TYPE float DEFAULT 1.0;
DEFINE FIELD IF NOT EXISTS created_at ON TABLE user TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS deleted_at ON TABLE user TYPE option<datetime>;

DEFINE INDEX IF NOT EXISTS user_username_idx ON TABLE user FIELDS username UNIQUE;

-- Canonical Concept Table (Pedagogical Hierarchy - Admin Gated)
DEFINE TABLE IF NOT EXISTS concept SCHEMAFULL
    PERMISSIONS
        FOR select FULL
        FOR create, update, delete WHERE $auth.role = 'admin';

DEFINE FIELD IF NOT EXISTS slug ON TABLE concept TYPE string;
DEFINE FIELD IF NOT EXISTS title ON TABLE concept TYPE string;
DEFINE FIELD IF NOT EXISTS category ON TABLE concept TYPE string;
DEFINE FIELD IF NOT EXISTS time_complexity ON TABLE concept TYPE option<string>;
DEFINE FIELD IF NOT EXISTS space_complexity ON TABLE concept TYPE option<string>;
DEFINE FIELD IF NOT EXISTS description ON TABLE concept TYPE string;
DEFINE FIELD IF NOT EXISTS story_metaphor ON TABLE concept TYPE option<string>;
DEFINE FIELD IF NOT EXISTS guided_inquiry ON TABLE concept TYPE option<string>;
DEFINE FIELD IF NOT EXISTS status ON TABLE concept TYPE string DEFAULT 'published';
DEFINE FIELD IF NOT EXISTS created_at ON TABLE concept TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS updated_at ON TABLE concept TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS deleted_at ON TABLE concept TYPE option<datetime>;

DEFINE INDEX IF NOT EXISTS concept_slug_idx ON TABLE concept FIELDS slug UNIQUE;

-- Multi-Model Snippet Embeddings (Namespaced by model_id)
DEFINE TABLE IF NOT EXISTS snippet_embedding SCHEMAFULL
    PERMISSIONS
        FOR select FULL
        FOR create, update, delete WHERE $auth.role = 'admin';

DEFINE FIELD IF NOT EXISTS concept_id ON TABLE snippet_embedding TYPE record<concept>;
DEFINE FIELD IF NOT EXISTS model_id ON TABLE snippet_embedding TYPE string;
DEFINE FIELD IF NOT EXISTS language ON TABLE snippet_embedding TYPE string;
DEFINE FIELD IF NOT EXISTS snippet_kind ON TABLE snippet_embedding TYPE string;
DEFINE FIELD IF NOT EXISTS embedding ON TABLE snippet_embedding TYPE array<float>;
DEFINE FIELD IF NOT EXISTS created_at ON TABLE snippet_embedding TYPE datetime DEFAULT time::now();

-- Semantic Vector Index for Snippet Embeddings
DEFINE INDEX IF NOT EXISTS snippet_embedding_hnsw ON TABLE snippet_embedding
    FIELDS embedding HNSW DIMENSION 1536 DIST COSINE;

-- Canonical Language Implementations
DEFINE TABLE IF NOT EXISTS implementation SCHEMAFULL
    PERMISSIONS
        FOR select FULL
        FOR create, update, delete WHERE $auth.role = 'admin';

DEFINE FIELD IF NOT EXISTS language ON TABLE implementation TYPE string;
DEFINE FIELD IF NOT EXISTS source_code ON TABLE implementation TYPE string;
DEFINE FIELD IF NOT EXISTS ast_hash ON TABLE implementation TYPE string;
DEFINE FIELD IF NOT EXISTS commit_sha ON TABLE implementation TYPE string;
DEFINE FIELD IF NOT EXISTS original_authors ON TABLE implementation TYPE array<string>;
DEFINE FIELD IF NOT EXISTS keywords ON TABLE implementation TYPE array<string>;
DEFINE FIELD IF NOT EXISTS license_spdx ON TABLE implementation TYPE string;
DEFINE FIELD IF NOT EXISTS origin_repo ON TABLE implementation TYPE string;
DEFINE FIELD IF NOT EXISTS origin_path ON TABLE implementation TYPE string;
DEFINE FIELD IF NOT EXISTS created_at ON TABLE implementation TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS deleted_at ON TABLE implementation TYPE option<datetime>;

-- Test Run (Immutable Execution Telemetry signed by system_runner)
DEFINE TABLE IF NOT EXISTS test_run SCHEMAFULL
    PERMISSIONS
        FOR select FULL
        FOR create, update, delete WHERE $auth.role = 'system_runner';

DEFINE FIELD IF NOT EXISTS implementation_id ON TABLE test_run TYPE record<implementation>;
DEFINE FIELD IF NOT EXISTS runner_id ON TABLE test_run TYPE string;
DEFINE FIELD IF NOT EXISTS runner_public_key ON TABLE test_run TYPE string;
DEFINE FIELD IF NOT EXISTS runner_signature ON TABLE test_run TYPE string;
DEFINE FIELD IF NOT EXISTS nonce ON TABLE test_run TYPE string;
DEFINE FIELD IF NOT EXISTS exit_reason ON TABLE test_run TYPE string;
DEFINE FIELD IF NOT EXISTS stdout_hash ON TABLE test_run TYPE string;
DEFINE FIELD IF NOT EXISTS stderr_hash ON TABLE test_run TYPE string;
DEFINE FIELD IF NOT EXISTS cpu_time_ms ON TABLE test_run TYPE int;
DEFINE FIELD IF NOT EXISTS wall_time_ms ON TABLE test_run TYPE int;
DEFINE FIELD IF NOT EXISTS peak_memory_bytes ON TABLE test_run TYPE int;
DEFINE FIELD IF NOT EXISTS test_count ON TABLE test_run TYPE int;
DEFINE FIELD IF NOT EXISTS executed_at ON TABLE test_run TYPE datetime DEFAULT time::now();

-- Student Submission Record
DEFINE TABLE IF NOT EXISTS submission SCHEMAFULL
    PERMISSIONS
        FOR select WHERE $auth.id = user_id OR $auth.role = 'admin'
        FOR create WHERE $auth.id = user_id
        FOR update, delete WHERE $auth.role = 'admin';

DEFINE FIELD IF NOT EXISTS user_id ON TABLE submission TYPE record<user>;
DEFINE FIELD IF NOT EXISTS concept_id ON TABLE submission TYPE record<concept>;
DEFINE FIELD IF NOT EXISTS test_run_id ON TABLE submission TYPE option<record<test_run>>;
DEFINE FIELD IF NOT EXISTS solution_code ON TABLE submission TYPE string;
DEFINE FIELD IF NOT EXISTS language ON TABLE submission TYPE string;
DEFINE FIELD IF NOT EXISTS socratic_score ON TABLE submission TYPE float DEFAULT 1.0;
DEFINE FIELD IF NOT EXISTS hints_consumed ON TABLE submission TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS passed ON TABLE submission TYPE bool;
DEFINE FIELD IF NOT EXISTS created_at ON TABLE submission TYPE datetime DEFAULT time::now();

-- Typed Directed Graph Relations
DEFINE TABLE IF NOT EXISTS PREREQUISITE_FOR TYPE RELATION IN concept OUT concept SCHEMAFULL
    PERMISSIONS
        FOR select FULL
        FOR create, update, delete WHERE $auth.role = 'admin';

DEFINE TABLE IF NOT EXISTS IMPLEMENTS TYPE RELATION IN implementation OUT concept SCHEMAFULL
    PERMISSIONS
        FOR select FULL
        FOR create, update, delete WHERE $auth.role = 'admin';

-- Cryptographically Verified Attestation: test_run -> implementation
-- Restriced to system_runner
DEFINE TABLE IF NOT EXISTS SANDBOX_VERIFIED TYPE RELATION IN test_run OUT implementation SCHEMAFULL
    PERMISSIONS
        FOR select FULL
        FOR create, update, delete WHERE $auth.role = 'system_runner';

DEFINE FIELD IF NOT EXISTS proof_payload_hash ON TABLE SANDBOX_VERIFIED TYPE string;
DEFINE FIELD IF NOT EXISTS verified_at ON TABLE SANDBOX_VERIFIED TYPE datetime DEFAULT time::now();

-- Peer Review & Attestation: user -> submission
DEFINE TABLE IF NOT EXISTS PEER_ATTESTED TYPE RELATION IN user OUT submission SCHEMAFULL
    PERMISSIONS
        FOR select FULL
        FOR create WHERE in.reputation_score >= 1.0
        FOR update, delete WHERE in = $auth.id OR $auth.role = 'admin';

DEFINE FIELD IF NOT EXISTS weight ON TABLE PEER_ATTESTED TYPE float DEFAULT 1.0;
DEFINE FIELD IF NOT EXISTS review_comment ON TABLE PEER_ATTESTED TYPE option<string>;
DEFINE FIELD IF NOT EXISTS created_at ON TABLE PEER_ATTESTED TYPE datetime DEFAULT time::now();
"#;
