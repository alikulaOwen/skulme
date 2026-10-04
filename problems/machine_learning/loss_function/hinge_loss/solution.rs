//! # Hinge Loss
//!
//! The `hng_loss` function calculates the Hinge loss, which is a
//! loss function used for classification problems in machine learning.
//!
//! ## Formula
//!
//! For a pair of actual and predicted values, represented as vectors `y_true` and
//! `y_pred`, the Hinge loss is calculated as:
//!
//! - loss = `max(0, 1 - y_true * y_pred)`.
//!
//! It returns the average loss by dividing the `total_loss` by total no. of
//! elements.
//!
pub fn hng_loss(y_true: &[f64], y_pred: &[f64]) -> f64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (hng_loss)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement hng_loss");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hinge_loss() {
        let predicted_values: Vec<f64> = vec![-1.0, 1.0, 1.0];
        let actual_values: Vec<f64> = vec![-1.0, -1.0, 1.0];
        assert_eq!(
            hng_loss(&predicted_values, &actual_values),
            0.6666666666666666
        );
    }
}
