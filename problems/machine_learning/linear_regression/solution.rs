/// Returns the parameters of the line after performing simple linear regression on the input data.
pub fn linear_regression(data_points: Vec<(f64, f64)>) -> Option<(f64, f64)> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (linear_regression)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement linear_regression");
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_linear_regression() {
        assert_eq!(
            linear_regression(vec![(0.0, 0.0), (1.0, 1.0), (2.0, 2.0)]),
            Some((2.220446049250313e-16, 0.9999999999999998))
        );
    }

    #[test]
    fn test_empty_list_linear_regression() {
        assert_eq!(linear_regression(vec![]), None);
    }
}
