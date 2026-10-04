/// Naive Bayes classifier for classification tasks.
/// This implementation uses Gaussian Naive Bayes, which assumes that
/// features follow a normal (Gaussian) distribution.
/// The algorithm calculates class priors and feature statistics (mean and variance)
/// for each class, then uses Bayes' theorem to predict class probabilities.

pub struct ClassStatistics {
    pub class_label: f64,
    pub prior: f64,
    pub feature_means: Vec<f64>,
    pub feature_variances: Vec<f64>,
}

fn calculate_class_statistics(
    training_data: &[(Vec<f64>, f64)],
    class_label: f64,
    num_features: usize,
) -> Option<ClassStatistics> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (calculate_class_statistics)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement calculate_class_statistics");
}

fn gaussian_log_pdf(x: f64, mean: f64, variance: f64) -> f64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (gaussian_log_pdf)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement gaussian_log_pdf");
}

pub fn train_naive_bayes(training_data: Vec<(Vec<f64>, f64)>) -> Option<Vec<ClassStatistics>> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (train_naive_bayes)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement train_naive_bayes");
}

pub fn predict_naive_bayes(model: &[ClassStatistics], test_point: &[f64]) -> Option<f64> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (predict_naive_bayes)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement predict_naive_bayes");
}

pub fn naive_bayes(training_data: Vec<(Vec<f64>, f64)>, test_point: Vec<f64>) -> Option<f64> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (naive_bayes)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement naive_bayes");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_naive_bayes_simple_classification() {
        let training_data = vec![
            (vec![1.0, 1.0], 0.0),
            (vec![1.1, 1.0], 0.0),
            (vec![1.0, 1.1], 0.0),
            (vec![5.0, 5.0], 1.0),
            (vec![5.1, 5.0], 1.0),
            (vec![5.0, 5.1], 1.0),
        ];

        // Test point closer to class 0
        let test_point = vec![1.05, 1.05];
        let result = naive_bayes(training_data.clone(), test_point);
        assert_eq!(result, Some(0.0));

        // Test point closer to class 1
        let test_point = vec![5.05, 5.05];
        let result = naive_bayes(training_data, test_point);
        assert_eq!(result, Some(1.0));
    }

    #[test]
    fn test_naive_bayes_one_dimensional() {
        let training_data = vec![
            (vec![1.0], 0.0),
            (vec![1.1], 0.0),
            (vec![1.2], 0.0),
            (vec![5.0], 1.0),
            (vec![5.1], 1.0),
            (vec![5.2], 1.0),
        ];

        let test_point = vec![1.15];
        let result = naive_bayes(training_data.clone(), test_point);
        assert_eq!(result, Some(0.0));

        let test_point = vec![5.15];
        let result = naive_bayes(training_data, test_point);
        assert_eq!(result, Some(1.0));
    }

    #[test]
    fn test_naive_bayes_empty_training_data() {
        let training_data = vec![];
        let test_point = vec![1.0, 2.0];
        let result = naive_bayes(training_data, test_point);
        assert_eq!(result, None);
    }

    #[test]
    fn test_naive_bayes_empty_test_point() {
        let training_data = vec![(vec![1.0, 2.0], 0.0)];
        let test_point = vec![];
        let result = naive_bayes(training_data, test_point);
        assert_eq!(result, None);
    }

    #[test]
    fn test_naive_bayes_dimension_mismatch() {
        let training_data = vec![(vec![1.0, 2.0], 0.0), (vec![3.0, 4.0], 1.0)];
        let test_point = vec![1.0]; // Wrong dimension
        let result = naive_bayes(training_data, test_point);
        assert_eq!(result, None);
    }

    #[test]
    fn test_naive_bayes_inconsistent_feature_dimensions() {
        let training_data = vec![
            (vec![1.0, 2.0], 0.0),
            (vec![3.0], 1.0), // Different dimension
        ];
        let test_point = vec![1.0, 2.0];
        let result = naive_bayes(training_data, test_point);
        assert_eq!(result, None);
    }

    #[test]
    fn test_naive_bayes_multiple_classes() {
        let training_data = vec![
            (vec![1.0, 1.0], 0.0),
            (vec![1.1, 1.0], 0.0),
            (vec![5.0, 5.0], 1.0),
            (vec![5.1, 5.0], 1.0),
            (vec![9.0, 9.0], 2.0),
            (vec![9.1, 9.0], 2.0),
        ];

        let test_point = vec![1.05, 1.05];
        let result = naive_bayes(training_data.clone(), test_point);
        assert_eq!(result, Some(0.0));

        let test_point = vec![5.05, 5.05];
        let result = naive_bayes(training_data.clone(), test_point);
        assert_eq!(result, Some(1.0));

        let test_point = vec![9.05, 9.05];
        let result = naive_bayes(training_data, test_point);
        assert_eq!(result, Some(2.0));
    }

    #[test]
    fn test_train_and_predict_separately() {
        let training_data = vec![
            (vec![1.0, 1.0], 0.0),
            (vec![1.1, 1.0], 0.0),
            (vec![5.0, 5.0], 1.0),
            (vec![5.1, 5.0], 1.0),
        ];

        let model = train_naive_bayes(training_data);
        assert!(model.is_some());

        let model = model.unwrap();
        assert_eq!(model.len(), 2);

        let test_point = vec![1.05, 1.05];
        let result = predict_naive_bayes(&model, &test_point);
        assert_eq!(result, Some(0.0));
    }
}
