//! # KL divergence Loss Function
//!
//! For a pair of actual and predicted probability distributions represented as vectors `actual` and `predicted`, the KL divergence loss is calculated as:
//!
//! `L = -Σ(actual[i] * ln(predicted[i]/actual[i]))` for all `i` in the range of the vectors
//!
//! Where `ln` is the natural logarithm function, and `Σ` denotes the summation over all elements of the vectors.
//!
//! ## KL divergence Loss Function Implementation
//!
//! This implementation takes two references to vectors of f64 values, `actual` and `predicted`, and returns the KL divergence loss between them.
//!
pub fn kld_loss(actual: &[f64], predicted: &[f64]) -> f64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (kld_loss)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement kld_loss");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kld_loss() {
        let test_vector_actual = vec![1.346112, 1.337432, 1.246655];
        let test_vector = vec![1.033836, 1.082015, 1.117323];
        assert_eq!(
            kld_loss(&test_vector_actual, &test_vector),
            0.7752789394328498
        );
    }
}
