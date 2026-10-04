//! # Mean Absolute Error Loss Function
//!
//! The `mae_loss` function calculates the Mean Absolute Error loss, which is a
//! robust loss function used in machine learning.
//!
//! ## Formula
//!
//! For a pair of actual and predicted values, represented as vectors `actual`
//! and `predicted`, the Mean Absolute  loss is calculated as:
//!
//! - loss = `(actual - predicted) / n_elements`.
//!
//! It returns the average loss by dividing the `total_loss` by total no. of
//! elements.
//!
pub fn mae_loss(predicted: &[f64], actual: &[f64]) -> f64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (mae_loss)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement mae_loss");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mae_loss() {
        let predicted_values: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];
        let actual_values: Vec<f64> = vec![1.0, 3.0, 3.5, 4.5];
        assert_eq!(mae_loss(&predicted_values, &actual_values), 0.5);
    }
}
