use super::optimization::gradient_descent;
use std::f64::consts::E;

/// Returns the weights after performing Logistic regression on the input data points.
pub fn logistic_regression(
    data_points: Vec<(Vec<f64>, f64)>,
    iterations: usize,
    learning_rate: f64,
) -> Option<Vec<f64>> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (logistic_regression)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement logistic_regression");
}

fn derivative(params: &[f64], data_points: &[(Vec<f64>, f64)]) -> Vec<f64> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (derivative)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement derivative");
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_logistic_regression_simple() {
        let data = vec![
            (vec![0.0], 0.0),
            (vec![1.0], 0.0),
            (vec![2.0], 0.0),
            (vec![3.0], 1.0),
            (vec![4.0], 1.0),
            (vec![5.0], 1.0),
        ];

        let result = logistic_regression(data, 10000, 0.05);
        assert!(result.is_some());

        let params = result.unwrap();
        assert!((params[0] + 17.65).abs() < 1.0);
        assert!((params[1] - 7.13).abs() < 1.0);
    }

    #[test]
    fn test_logistic_regression_extreme_data() {
        let data = vec![
            (vec![-100.0], 0.0),
            (vec![-10.0], 0.0),
            (vec![0.0], 0.0),
            (vec![10.0], 1.0),
            (vec![100.0], 1.0),
        ];

        let result = logistic_regression(data, 10000, 0.05);
        assert!(result.is_some());

        let params = result.unwrap();
        assert!((params[0] + 6.20).abs() < 1.0);
        assert!((params[1] - 5.5).abs() < 1.0);
    }

    #[test]
    fn test_logistic_regression_no_data() {
        let result = logistic_regression(vec![], 5000, 0.1);
        assert_eq!(result, None);
    }
}
