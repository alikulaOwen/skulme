//! # Mean Square Loss Function
//!
//! The `mse_loss` function calculates the Mean Square Error loss, which is a
//! robust loss function used in machine learning.
//!
//! ## Formula
//!
//! For a pair of actual and predicted values, represented as vectors `actual`
//! and `predicted`, the Mean Square  loss is calculated as:
//!
//! - loss = `(actual - predicted)^2 / n_elements`.
//!
//! It returns the average loss by dividing the `total_loss` by total no. of
//! elements.
//!
pub fn mse_loss(predicted: &[f64], actual: &[f64]) -> f64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (mse_loss)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement mse_loss");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mse_loss() {
        let predicted_values: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];
        let actual_values: Vec<f64> = vec![1.0, 3.0, 3.5, 4.5];
        assert_eq!(mse_loss(&predicted_values, &actual_values), 0.375);
    }
}
