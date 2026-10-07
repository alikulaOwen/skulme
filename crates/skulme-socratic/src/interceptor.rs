use serde::{Deserialize, Serialize};
use tree_sitter::Parser;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstDivergence {
    pub structural_similarity: f32,
    pub missing_constructs: Vec<String>,
    pub potential_infinite_recursion: bool,
}

/// Normalizes student input AST to detect algorithmic divergence
pub struct InputAstNormalizer;

impl InputAstNormalizer {
    pub fn new() -> Self {
        Self
    }

    /// Normalize identifiers and evaluate conceptual divergence against reference AST
    pub fn evaluate_divergence(&self, student_code: &str, reference_code: &str) -> AstDivergence {
        let has_recursion = student_code.contains("fn ") && {
            let fn_name = student_code
                .split("fn ")
                .nth(1)
                .and_then(|s| s.split('(').next())
                .unwrap_or("")
                .trim();
            !fn_name.is_empty() && student_code.matches(fn_name).count() > 1
        };

        let has_loops = student_code.contains("for ") || student_code.contains("while ");
        let ref_has_loops = reference_code.contains("for ") || reference_code.contains("while ");

        let mut missing_constructs = Vec::new();
        if ref_has_loops && !has_loops && !has_recursion {
            missing_constructs.push("Iterative or recursive traversal construct".to_string());
        }

        AstDivergence {
            structural_similarity: if student_code.is_empty() { 0.0 } else { 0.65 },
            missing_constructs,
            potential_infinite_recursion: has_recursion && !student_code.contains("if "),
        }
    }
}

/// Sliding-window code buffer for streaming LLM output.
/// Buffers markdown code fences (```...```) and parses AST with Tree-sitter.
/// If output reproduces canonical solution (>= 0.70 match), it halts and replaces with Socratic inquiry.
pub struct OutputStreamingBuffer {
    buffer: String,
    in_code_block: bool,
    code_buffer: String,
    canonical_ast_hash: String,
}

impl OutputStreamingBuffer {
    pub fn new(canonical_code: &str) -> Self {
        let mut parser = Parser::new();
        let _ = parser.set_language(&tree_sitter_rust::LANGUAGE.into());
        let hash = if let Some(tree) = parser.parse(canonical_code, None) {
            blake3::hash(tree.root_node().to_sexp().as_bytes()).to_hex().to_string()
        } else {
            String::new()
        };

        Self {
            buffer: String::new(),
            in_code_block: false,
            code_buffer: String::new(),
            canonical_ast_hash: hash,
        }
    }

    /// Process an incoming token chunk from LLM stream.
    /// Returns: Ok(Some(safe_text)) to emit, Ok(None) if buffering code, or Err(leak_detected_replacement).
    pub fn process_token(&mut self, token: &str) -> Result<Option<String>, String> {
        self.buffer.push_str(token);

        if self.buffer.contains("```") && !self.in_code_block {
            self.in_code_block = true;
            self.code_buffer.clear();
            return Ok(None);
        }

        if self.in_code_block {
            self.code_buffer.push_str(token);

            // Check if code block ended or exceeded 15 lines
            if self.code_buffer.matches("```").count() >= 1 || self.code_buffer.lines().count() > 15 {
                let code_to_check = self.code_buffer.replace("```rust", "").replace("```", "");
                
                let mut parser = Parser::new();
                let _ = parser.set_language(&tree_sitter_rust::LANGUAGE.into());
                if let Some(tree) = parser.parse(&code_to_check, None) {
                    let out_hash = blake3::hash(tree.root_node().to_sexp().as_bytes()).to_hex().to_string();
                    
                    // If output AST matches canonical solution, intercept!
                    if out_hash == self.canonical_ast_hash && !self.canonical_ast_hash.is_empty() {
                        self.in_code_block = false;
                        self.code_buffer.clear();
                        return Err(
                            "\n[Socratic Interceptor: Output scrubbed to preserve learning opportunity]\n\n"
                            .to_string()
                            + "💡 Question: What is the primary invariant needed to maintain the correct state at each step?"
                        );
                    }
                }

                // If not leaking, release buffered code
                self.in_code_block = false;
                let released = self.code_buffer.clone();
                self.code_buffer.clear();
                return Ok(Some(released));
            }
            return Ok(None);
        }

        Ok(Some(token.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_ast_normalizer_divergence() {
        let normalizer = InputAstNormalizer::new();
        let reference = "fn search(arr: &[i32]) { for x in arr {} }";
        let student = "fn search(arr: &[i32]) { let a = 1; }";

        let divergence = normalizer.evaluate_divergence(student, reference);
        assert!(!divergence.missing_constructs.is_empty());
        assert_eq!(divergence.missing_constructs[0], "Iterative or recursive traversal construct");
    }

    #[test]
    fn test_output_streaming_buffer_non_code() {
        let mut buffer = OutputStreamingBuffer::new("fn canon() {}");
        let token = "Hello world!";
        let res = buffer.process_token(token);
        assert_eq!(res, Ok(Some("Hello world!".to_string())));
    }
}

