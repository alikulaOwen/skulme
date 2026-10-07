use skulme_core::ast::{AstMetadata, AstSignature};
use skulme_core::models::Language;
use thiserror::Error;
use tree_sitter::Parser;

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("Tree-sitter parser failed: {0}")]
    TreeSitterError(String),

    #[error("Language grammar error: {0}")]
    GrammarError(String),
}

/// Multi-language Tree-sitter AST parser
pub struct MultiLangAstParser;

impl MultiLangAstParser {
    pub fn new() -> Self {
        Self
    }

    /// Parse source code across any of the 6 supported languages
    pub fn parse(&self, language: Language, source: &str) -> Result<AstMetadata, ParserError> {
        let mut parser = Parser::new();
        
        let ts_lang = match language {
            Language::Rust => tree_sitter_rust::LANGUAGE.into(),
            Language::Python => tree_sitter_python::LANGUAGE.into(),
            Language::Java => tree_sitter_java::LANGUAGE.into(),
            Language::Cpp => tree_sitter_cpp::LANGUAGE.into(),
            Language::Go => tree_sitter_go::LANGUAGE.into(),
            Language::TypeScript => tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
        };

        parser
            .set_language(&ts_lang)
            .map_err(|e| ParserError::GrammarError(e.to_string()))?;

        let tree = parser
            .parse(source, None)
            .ok_or_else(|| ParserError::TreeSitterError("Failed to parse tree".to_string()))?;

        let root_node = tree.root_node();
        let ast_sexpr = root_node.to_sexp();
        let ast_hash = blake3::hash(ast_sexpr.as_bytes()).to_hex().to_string();

        // Extract docstrings
        let docstrings: Vec<String> = source
            .lines()
            .filter(|l| {
                let t = l.trim();
                t.starts_with("///") || t.starts_with("//!") || t.starts_with("#") || t.starts_with("/**") || t.starts_with("*")
            })
            .map(|l| l.trim().to_string())
            .collect();

        let docstring = if docstrings.is_empty() {
            None
        } else {
            Some(docstrings.join("\n"))
        };

        // Extract keywords for Jaccard similarity term
        let mut keywords: Vec<String> = source
            .split_whitespace()
            .filter(|w| w.len() > 3 && w.chars().all(|c| c.is_alphabetic()))
            .map(|w| w.to_lowercase())
            .collect();
        keywords.sort();
        keywords.dedup();

        // Extract functions from top-level AST nodes
        let mut functions = Vec::new();
        let mut cursor = root_node.walk();
        for child in root_node.children(&mut cursor) {
            let kind = child.kind();
            if kind.contains("function") || kind.contains("method") || kind.contains("fn_item") {
                functions.push(AstSignature {
                    name: format!("{kind}_{}", child.id()),
                    params: Vec::new(),
                    return_type: None,
                    is_async: false,
                    is_public: true,
                });
            }
        }

        let test_count = source.matches("#[test]").count()
            + source.matches("def test_").count()
            + source.matches("@Test").count();

        Ok(AstMetadata {
            language,
            docstring,
            functions,
            keywords,
            time_complexity: None,
            space_complexity: None,
            test_count,
            ast_hash,
        })
    }
}
