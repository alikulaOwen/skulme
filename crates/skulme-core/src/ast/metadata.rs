use serde::{Deserialize, Serialize};

use crate::models::Language;

/// Concrete function signature extracted from source code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstSignature {
    pub name: String,
    pub params: Vec<(String, String)>,
    pub return_type: Option<String>,
    pub is_async: bool,
    pub is_public: bool,
}

/// Abstract metadata extracted from parsed CST/AST
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstMetadata {
    pub language: Language,
    pub docstring: Option<String>,
    pub functions: Vec<AstSignature>,
    pub keywords: Vec<String>,
    pub time_complexity: Option<String>,
    pub space_complexity: Option<String>,
    pub test_count: usize,
    pub ast_hash: String,
}
