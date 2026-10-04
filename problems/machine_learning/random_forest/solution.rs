use rand::seq::SliceRandom;
use rand::RngExt;

/// Train a single decision tree on a bootstrap sample with random feature subset
#[allow(dead_code)]
fn train_tree(
    training_data: &[(Vec<f64>, f64)],
    num_features: usize,
    max_depth: usize,
    min_samples_split: usize,
    max_features: usize,
) -> Option<crate::machine_learning::decision_tree::DecisionTree> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (train_tree)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement train_tree");
}

#[derive(Debug, PartialEq)]
pub struct RandomForest {
    trees: Vec<crate::machine_learning::decision_tree::DecisionTree>,
    feature_indices: Vec<Vec<usize>>,
    num_classes: usize,
}

impl RandomForest {
    pub fn fit(
        training_data: Vec<(Vec<f64>, f64)>,
        num_trees: usize,
        max_depth: usize,
        min_samples_split: usize,
        max_features: Option<usize>,
    ) -> Option<Self> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (fit)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement fit");
}

    pub fn predict(&self, test_point: &[f64]) -> Option<f64> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (predict)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement predict");
}

    #[allow(dead_code)]
    pub fn predict_batch(&self, test_points: &[Vec<f64>]) -> Vec<Option<f64>> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (predict_batch)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement predict_batch");
}
}

/// Convenience function to train a random forest and make predictions
pub fn random_forest(
    training_data: Vec<(Vec<f64>, f64)>,
    test_point: Vec<f64>,
    num_trees: usize,
    max_depth: usize,
    min_samples_split: usize,
    max_features: Option<usize>,
) -> Option<f64> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (random_forest)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement random_forest");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_forest_linearly_separable() {
        let training_data = vec![
            (vec![1.0, 1.0], 0.0),
            (vec![2.0, 2.0], 0.0),
            (vec![3.0, 3.0], 0.0),
            (vec![5.0, 5.0], 1.0),
            (vec![6.0, 6.0], 1.0),
            (vec![7.0, 7.0], 1.0),
        ];

        let model = RandomForest::fit(training_data, 10, 5, 2, None);
        assert!(model.is_some());

        let model = model.unwrap();

        assert_eq!(model.predict(&[1.5, 1.5]), Some(0.0));
        assert_eq!(model.predict(&[5.5, 5.5]), Some(1.0));
    }

    #[test]
    fn test_random_forest_xor() {
        let training_data = vec![
            (vec![0.0, 0.0], 0.0),
            (vec![0.0, 1.0], 1.0),
            (vec![1.0, 0.0], 1.0),
            (vec![1.0, 1.0], 0.0),
            // Add more samples to help with XOR
            (vec![0.2, 0.2], 0.0),
            (vec![0.8, 0.8], 0.0),
            (vec![0.2, 0.8], 1.0),
            (vec![0.8, 0.2], 1.0),
        ];

        let model = RandomForest::fit(training_data, 20, 5, 2, Some(2));
        assert!(model.is_some());

        let model = model.unwrap();

        // Verify model can make predictions (not necessarily perfect)
        let result = model.predict(&[0.0, 0.0]);
        assert!(result.is_some());

        let result = model.predict(&[1.0, 1.0]);
        assert!(result.is_some());
    }

    #[test]
    fn test_random_forest_multiple_classes() {
        let training_data = vec![
            (vec![1.0, 1.0], 0.0),
            (vec![2.0, 2.0], 0.0),
            (vec![5.0, 5.0], 1.0),
            (vec![6.0, 6.0], 1.0),
            (vec![9.0, 9.0], 2.0),
            (vec![10.0, 10.0], 2.0),
        ];

        let model = RandomForest::fit(training_data, 10, 5, 2, None);
        assert!(model.is_some());

        let model = model.unwrap();

        assert_eq!(model.predict(&[1.5, 1.5]), Some(0.0));
        assert_eq!(model.predict(&[5.5, 5.5]), Some(1.0));
        assert_eq!(model.predict(&[9.5, 9.5]), Some(2.0));
    }

    #[test]
    fn test_random_forest_one_feature() {
        let training_data = vec![
            (vec![1.0], 0.0),
            (vec![2.0], 0.0),
            (vec![3.0], 0.0),
            (vec![5.0], 1.0),
            (vec![6.0], 1.0),
            (vec![7.0], 1.0),
        ];

        let model = RandomForest::fit(training_data, 10, 5, 2, None);
        assert!(model.is_some());

        let model = model.unwrap();

        assert_eq!(model.predict(&[2.5]), Some(0.0));
        assert_eq!(model.predict(&[5.5]), Some(1.0));
    }

    #[test]
    fn test_random_forest_empty_training_data() {
        let training_data = vec![];
        let model = RandomForest::fit(training_data, 10, 5, 2, None);
        assert_eq!(model, None);
    }

    #[test]
    fn test_random_forest_empty_features() {
        let training_data = vec![(vec![], 0.0), (vec![], 1.0)];
        let model = RandomForest::fit(training_data, 10, 5, 2, None);
        assert_eq!(model, None);
    }

    #[test]
    fn test_random_forest_predict_batch() {
        let training_data = vec![
            (vec![1.0, 1.0], 0.0),
            (vec![2.0, 2.0], 0.0),
            (vec![5.0, 5.0], 1.0),
            (vec![6.0, 6.0], 1.0),
        ];

        let model = RandomForest::fit(training_data, 10, 5, 2, None);
        assert!(model.is_some());

        let model = model.unwrap();

        let test_points = vec![vec![1.5, 1.5], vec![5.5, 5.5]];
        let predictions = model.predict_batch(&test_points);

        assert_eq!(predictions.len(), 2);
        assert_eq!(predictions[0], Some(0.0));
        assert_eq!(predictions[1], Some(1.0));
    }

    #[test]
    fn test_random_forest_custom_max_features() {
        let training_data = vec![
            (vec![1.0, 2.0, 3.0], 0.0),
            (vec![2.0, 3.0, 4.0], 0.0),
            (vec![5.0, 6.0, 7.0], 1.0),
            (vec![6.0, 7.0, 8.0], 1.0),
        ];

        let model = RandomForest::fit(training_data, 10, 5, 2, Some(2));
        assert!(model.is_some());

        let model = model.unwrap();

        assert_eq!(model.predict(&[1.5, 2.5, 3.5]), Some(0.0));
        assert_eq!(model.predict(&[5.5, 6.5, 7.5]), Some(1.0));
    }

    #[test]
    fn test_random_forest_convenience_function() {
        let training_data = vec![
            (vec![1.0, 1.0], 0.0),
            (vec![2.0, 2.0], 0.0),
            (vec![5.0, 5.0], 1.0),
            (vec![6.0, 6.0], 1.0),
        ];

        let result = random_forest(training_data, vec![1.5, 1.5], 10, 5, 2, None);
        assert_eq!(result, Some(0.0));

        let training_data = vec![
            (vec![1.0, 1.0], 0.0),
            (vec![2.0, 2.0], 0.0),
            (vec![5.0, 5.0], 1.0),
            (vec![6.0, 6.0], 1.0),
        ];

        let result = random_forest(training_data, vec![5.5, 5.5], 10, 5, 2, None);
        assert_eq!(result, Some(1.0));
    }

    #[test]
    fn test_random_forest_single_tree() {
        let training_data = vec![
            (vec![1.0, 1.0], 0.0),
            (vec![2.0, 2.0], 0.0),
            (vec![5.0, 5.0], 1.0),
            (vec![6.0, 6.0], 1.0),
        ];

        let model = RandomForest::fit(training_data, 1, 5, 2, None);
        assert!(model.is_some());

        let model = model.unwrap();

        // With single tree and bootstrap sampling, predictions may vary
        // Just verify model can make predictions
        let result1 = model.predict(&[1.5, 1.5]);
        let result2 = model.predict(&[5.5, 5.5]);

        assert!(result1.is_some());
        assert!(result2.is_some());
    }

    #[test]
    fn test_random_forest_empty_test_point() {
        let training_data = vec![
            (vec![1.0, 1.0], 0.0),
            (vec![2.0, 2.0], 0.0),
            (vec![5.0, 5.0], 1.0),
            (vec![6.0, 6.0], 1.0),
        ];

        let model = RandomForest::fit(training_data, 10, 5, 2, None);
        assert!(model.is_some());

        let model = model.unwrap();

        let result = model.predict(&[]);
        assert_eq!(result, None);
    }

    #[test]
    fn test_random_forest_different_num_trees() {
        let training_data = vec![
            (vec![1.0, 1.0], 0.0),
            (vec![2.0, 2.0], 0.0),
            (vec![5.0, 5.0], 1.0),
            (vec![6.0, 6.0], 1.0),
        ];

        let model_5 = RandomForest::fit(training_data.clone(), 5, 5, 2, None);
        let model_20 = RandomForest::fit(training_data, 20, 5, 2, None);

        assert!(model_5.is_some());
        assert!(model_20.is_some());

        let model_5 = model_5.unwrap();
        let model_20 = model_20.unwrap();

        assert_eq!(model_5.predict(&[1.5, 1.5]), Some(0.0));
        assert_eq!(model_20.predict(&[1.5, 1.5]), Some(0.0));
    }
}
