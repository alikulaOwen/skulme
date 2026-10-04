/// K-Nearest Neighbors (KNN) algorithm for classification.
/// KNN is a simple, instance-based learning algorithm that classifies
/// a data point based on the majority class of its k nearest neighbors.

fn euclidean_distance(p1: &[f64], p2: &[f64]) -> f64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (euclidean_distance)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement euclidean_distance");
}

pub fn k_nearest_neighbors(
    training_data: Vec<(Vec<f64>, f64)>,
    test_point: Vec<f64>,
    k: usize,
) -> Option<f64> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (k_nearest_neighbors)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement k_nearest_neighbors");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_knn() {
        let training_data = vec![
            (vec![0.0, 0.0], 0.0),
            (vec![1.0, 0.0], 0.0),
            (vec![0.0, 1.0], 0.0),
            (vec![5.0, 5.0], 1.0),
            (vec![6.0, 5.0], 1.0),
            (vec![5.0, 6.0], 1.0),
        ];

        let test_point = vec![0.5, 0.5];
        let result = k_nearest_neighbors(training_data.clone(), test_point, 3);
        assert_eq!(result, Some(0.0));

        let test_point = vec![5.5, 5.5];
        let result = k_nearest_neighbors(training_data, test_point, 3);
        assert_eq!(result, Some(1.0));
    }

    #[test]
    fn test_one_dimensional_knn() {
        let training_data = vec![
            (vec![1.0], 0.0),
            (vec![2.0], 0.0),
            (vec![3.0], 0.0),
            (vec![8.0], 1.0),
            (vec![9.0], 1.0),
            (vec![10.0], 1.0),
        ];

        let test_point = vec![2.5];
        let result = k_nearest_neighbors(training_data, test_point, 3);
        assert_eq!(result, Some(0.0));
    }

    #[test]
    fn test_knn_empty_data() {
        let training_data = vec![];
        let test_point = vec![1.0, 2.0];
        let result = k_nearest_neighbors(training_data, test_point, 3);
        assert_eq!(result, None);
    }

    #[test]
    fn test_knn_invalid_k() {
        let training_data = vec![(vec![1.0], 0.0), (vec![2.0], 1.0)];
        let test_point = vec![1.5];

        // k = 0 should return None
        let result = k_nearest_neighbors(training_data.clone(), test_point.clone(), 0);
        assert_eq!(result, None);

        // k > training_data.len() should return None
        let result = k_nearest_neighbors(training_data, test_point, 10);
        assert_eq!(result, None);
    }

    #[test]
    fn test_euclidean_distance_different_dimensions() {
        let training_data = vec![
            (vec![1.0, 2.0], 0.0),
            (vec![2.0, 3.0], 0.0),
            (vec![5.0], 1.0),
        ];
        let test_point = vec![1.5, 2.5];
        let result = k_nearest_neighbors(training_data, test_point, 2);
        assert_eq!(result, Some(0.0));
    }
}
