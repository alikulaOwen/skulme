//! # Huber Loss Function
//!
//! The `huber_loss` function calculates the Huber loss, which is a robust loss function used in machine learning, particularly in regression problems.
//!
//! Huber loss combines the benefits of mean squared error (MSE) and mean absolute error (MAE). It behaves like MSE when the difference between actual and predicted values is small (less than a specified `delta`), and like MAE when the difference is large.
//!
//! ## Formula
//!
//! For a pair of actual and predicted values, represented as vectors `actual` and `predicted`, and a specified `delta` value, the Huber loss is calculated as:
//!
//! - If the absolute difference between `actual[i]` and `predicted[i]` is less than or equal to `delta`, the loss is `0.5 * (actual[i] - predicted[i])^2`.
//! - If the absolute difference is greater than `delta`, the loss is `delta * |actual[i] - predicted[i]| - 0.5 * delta`.
//!
//! The total loss is the sum of individual losses over all elements.
//!
//! ## Huber Loss Function Implementation
//!
//! This implementation takes two references to vectors of f64 values, `actual` and `predicted`, and a `delta` value. It returns the Huber loss between them, providing a robust measure of dissimilarity between actual and predicted values.
//!
pub fn huber_loss(actual: &[f64], predicted: &[f64], delta: f64) -> f64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (huber_loss)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement huber_loss");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_huber_loss() {
        let test_vector_actual = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let test_vector = vec![5.0, 7.0, 9.0, 11.0, 13.0];
        assert_eq!(huber_loss(&test_vector_actual, &test_vector, 1.0), 27.5);
    }
}
