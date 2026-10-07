use std::collections::HashSet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClusterDecision {
    /// Score >= 0.85: High confidence match, automatically link implementation via IMPLEMENTS
    AutoLink { concept_id: String, score: f32 },
    /// 0.70 <= Score < 0.85: Ambiguous match, flag for human maintainer review queue
    NeedsReview { candidate_concept_id: String, score: f32 },
    /// Score < 0.70: Staged candidate concept awaiting admin sign-off
    StageCandidate { score: f32 },
}

/// Tri-Band Multi-Modal Concept Clustering Evaluator
pub struct TriBandClusterer {
    pub auto_link_threshold: f32,
    pub review_threshold: f32,
}

impl Default for TriBandClusterer {
    fn default() -> Self {
        Self {
            auto_link_threshold: 0.85,
            review_threshold: 0.70,
        }
    }
}

impl TriBandClusterer {
    pub fn new(auto_link_threshold: f32, review_threshold: f32) -> Self {
        Self {
            auto_link_threshold,
            review_threshold,
        }
    }

    /// Compute cosine similarity between two float vectors
    pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
        if a.is_empty() || b.is_empty() || a.len() != b.len() {
            return 0.0;
        }
        let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm_a == 0.0 || norm_b == 0.0 {
            0.0
        } else {
            dot / (norm_a * norm_b)
        }
    }

    /// Compute Jaccard similarity J(A, B) = |A ∩ B| / |A ∪ B|
    pub fn jaccard_similarity(a: &[String], b: &[String]) -> f32 {
        let set_a: HashSet<&String> = a.iter().collect();
        let set_b: HashSet<&String> = b.iter().collect();
        let intersection = set_a.intersection(&set_b).count();
        let union = set_a.union(&set_b).count();
        if union == 0 {
            0.0
        } else {
            intersection as f32 / union as f32
        }
    }

    /// Compute multi-modal similarity score S(I_k, C_n)
    pub fn compute_similarity(
        &self,
        doc_emb_a: &[f32],
        doc_emb_b: &[f32],
        ast_emb_a: &[f32],
        ast_emb_b: &[f32],
        keywords_a: &[String],
        keywords_b: &[String],
    ) -> f32 {
        let sim_doc = Self::cosine_similarity(doc_emb_a, doc_emb_b);
        let sim_ast = Self::cosine_similarity(ast_emb_a, ast_emb_b);
        let sim_jaccard = Self::jaccard_similarity(keywords_a, keywords_b);

        0.5 * sim_doc + 0.3 * sim_ast + 0.2 * sim_jaccard
    }

    /// Classify into one of the three governance bands
    pub fn classify(&self, score: f32, candidate_concept_id: &str) -> ClusterDecision {
        if score >= self.auto_link_threshold {
            ClusterDecision::AutoLink {
                concept_id: candidate_concept_id.to_string(),
                score,
            }
        } else if score >= self.review_threshold {
            ClusterDecision::NeedsReview {
                candidate_concept_id: candidate_concept_id.to_string(),
                score,
            }
        } else {
            ClusterDecision::StageCandidate { score }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        let c = vec![0.0, 1.0, 0.0];

        assert!((TriBandClusterer::cosine_similarity(&a, &b) - 1.0).abs() < 1e-6);
        assert!((TriBandClusterer::cosine_similarity(&a, &c) - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_jaccard_similarity() {
        let a = vec!["binary".to_string(), "search".to_string(), "tree".to_string()];
        let b = vec!["binary".to_string(), "search".to_string(), "graph".to_string()];

        // intersection = 2, union = 4 -> 2/4 = 0.5
        let sim = TriBandClusterer::jaccard_similarity(&a, &b);
        assert!((sim - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_tri_band_classification() {
        let clusterer = TriBandClusterer::default();

        assert_eq!(
            clusterer.classify(0.92, "concept:two_sum"),
            ClusterDecision::AutoLink {
                concept_id: "concept:two_sum".to_string(),
                score: 0.92
            }
        );

        assert_eq!(
            clusterer.classify(0.78, "concept:two_sum"),
            ClusterDecision::NeedsReview {
                candidate_concept_id: "concept:two_sum".to_string(),
                score: 0.78
            }
        );

        assert_eq!(
            clusterer.classify(0.45, "concept:two_sum"),
            ClusterDecision::StageCandidate { score: 0.45 }
        );
    }
}
