/// Principal Component Analysis (PCA) for dimensionality reduction.
/// PCA transforms data to a new coordinate system where the greatest
/// variance lies on the first coordinate (first principal component),
/// the second greatest variance on the second coordinate, and so on.

/// Compute the mean of each feature across all samples
fn compute_means(data: &[Vec<f64>]) -> Vec<f64> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (compute_means)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement compute_means");
}

/// Center the data by subtracting the mean from each feature
fn center_data(data: &[Vec<f64>], means: &[f64]) -> Vec<Vec<f64>> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (center_data)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement center_data");
}

/// Compute covariance matrix from centered data
fn compute_covariance_matrix(centered_data: &[Vec<f64>]) -> Vec<f64> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (compute_covariance_matrix)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement compute_covariance_matrix");
}

/// Power iteration method to find the dominant eigenvalue and eigenvector
fn power_iteration(matrix: &[f64], n: usize, max_iter: usize, tolerance: f64) -> (f64, Vec<f64>) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (power_iteration)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement power_iteration");
}

/// Deflate a matrix by removing the component along a given eigenvector
fn deflate_matrix(matrix: &[f64], eigenvector: &[f64], eigenvalue: f64, n: usize) -> Vec<f64> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (deflate_matrix)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement deflate_matrix");
}

/// Perform PCA on the input data
/// Returns transformed data with reduced dimensions
pub fn principal_component_analysis(
    data: Vec<Vec<f64>>,
    num_components: usize,
) -> Option<Vec<Vec<f64>>> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (principal_component_analysis)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement principal_component_analysis");
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pca_simple() {
        let data = vec![
            vec![1.0, 2.0],
            vec![2.0, 3.0],
            vec![3.0, 4.0],
            vec![4.0, 5.0],
            vec![5.0, 6.0],
        ];

        let result = principal_component_analysis(data, 1);
        assert!(result.is_some());

        let transformed = result.unwrap();
        assert_eq!(transformed.len(), 5);
        assert_eq!(transformed[0].len(), 1);

        let all_values: Vec<f64> = transformed.iter().map(|v| v[0]).collect();
        let mean = all_values.iter().sum::<f64>() / all_values.len() as f64;

        assert!((mean).abs() < 1e-5);
    }

    #[test]
    fn test_pca_empty_data() {
        let data = vec![];
        let result = principal_component_analysis(data, 2);
        assert_eq!(result, None);
    }

    #[test]
    fn test_pca_empty_features() {
        let data = vec![vec![], vec![]];
        let result = principal_component_analysis(data, 1);
        assert_eq!(result, None);
    }

    #[test]
    fn test_pca_invalid_num_components() {
        let data = vec![vec![1.0, 2.0], vec![2.0, 3.0]];

        let result = principal_component_analysis(data.clone(), 3);
        assert_eq!(result, None);

        let result = principal_component_analysis(data, 0);
        assert_eq!(result, None);
    }

    #[test]
    fn test_pca_preserves_dimensions() {
        let data = vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
        ];

        let result = principal_component_analysis(data, 2);
        assert!(result.is_some());

        let transformed = result.unwrap();
        assert_eq!(transformed.len(), 3);
        assert_eq!(transformed[0].len(), 2);
    }

    #[test]
    fn test_pca_reconstruction_variance() {
        let data = vec![
            vec![2.5, 2.4],
            vec![0.5, 0.7],
            vec![2.2, 2.9],
            vec![1.9, 2.2],
            vec![3.1, 3.0],
            vec![2.3, 2.7],
            vec![2.0, 1.6],
            vec![1.0, 1.1],
            vec![1.5, 1.6],
            vec![1.1, 0.9],
        ];

        let result = principal_component_analysis(data, 1);
        assert!(result.is_some());

        let transformed = result.unwrap();
        assert_eq!(transformed.len(), 10);
        assert_eq!(transformed[0].len(), 1);
    }

    #[test]
    fn test_center_data() {
        let data = vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
        ];

        let means = vec![4.0, 5.0, 6.0];
        let centered = center_data(&data, &means);

        assert_eq!(centered[0], vec![-3.0, -3.0, -3.0]);
        assert_eq!(centered[1], vec![0.0, 0.0, 0.0]);
        assert_eq!(centered[2], vec![3.0, 3.0, 3.0]);
    }

    #[test]
    fn test_compute_means() {
        let data = vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
        ];

        let means = compute_means(&data);
        assert_eq!(means, vec![4.0, 5.0, 6.0]);
    }

    #[test]
    fn test_power_iteration() {
        let matrix = vec![4.0, 1.0, 1.0, 1.0, 3.0, 1.0, 1.0, 1.0, 2.0];

        let (eigenvalue, eigenvector) = power_iteration(&matrix, 3, 1000, 1e-10);

        assert!(eigenvalue > 0.0);
        assert_eq!(eigenvector.len(), 3);

        let norm = eigenvector.iter().map(|x| x * x).sum::<f64>().sqrt();
        assert!((norm - 1.0).abs() < 1e-6);
    }
}
