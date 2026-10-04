/// Decision Tree classifier using the ID3 algorithm with entropy-based splitting.
/// The tree recursively splits data based on the feature that provides the highest information gain.
/// Supports both categorical and continuous features through threshold-based splitting.

#[derive(Debug, Clone, PartialEq)]
enum TreeNode {
    Leaf {
        class_label: f64,
        samples: usize,
    },
    InternalNode {
        feature_index: usize,
        threshold: f64,
        left: Box<TreeNode>,
        right: Box<TreeNode>,
        samples: usize,
    },
}

/// Calculate entropy of a set of labels
fn calculate_entropy(labels: &[f64]) -> f64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (calculate_entropy)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement calculate_entropy");
}

/// Find the best split for a feature
fn find_best_split(data: &[(Vec<f64>, f64)], feature_index: usize) -> Option<(f64, f64)> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (find_best_split)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement find_best_split");
}

/// Find the best feature and threshold to split on
fn find_best_split_feature(
    data: &[(Vec<f64>, f64)],
    feature_indices: &[usize],
) -> Option<(usize, f64)> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (find_best_split_feature)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement find_best_split_feature");
}

/// Get the majority class label
fn get_majority_class(labels: &[f64]) -> f64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (get_majority_class)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement get_majority_class");
}

/// Build the decision tree recursively
fn build_tree(
    data: &[(Vec<f64>, f64)],
    feature_indices: &[usize],
    max_depth: usize,
    min_samples_split: usize,
    current_depth: usize,
) -> TreeNode {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (build_tree)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement build_tree");
}

/// Predict the class label for a single test point
fn predict_tree(tree: &TreeNode, features: &[f64]) -> f64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (predict_tree)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement predict_tree");
}

#[derive(Debug, PartialEq)]
pub struct DecisionTree {
    tree: TreeNode,
}

impl DecisionTree {
    pub fn fit(
        training_data: Vec<(Vec<f64>, f64)>,
        max_depth: usize,
        min_samples_split: usize,
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

/// Convenience function to train a decision tree and make predictions
pub fn decision_tree(
    training_data: Vec<(Vec<f64>, f64)>,
    test_point: Vec<f64>,
    max_depth: usize,
    min_samples_split: usize,
) -> Option<f64> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (decision_tree)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement decision_tree");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decision_tree_simple_xor() {
        let training_data = vec![
            (vec![0.0, 0.0], 0.0),
            (vec![0.0, 1.0], 1.0),
            (vec![1.0, 0.0], 1.0),
            (vec![1.0, 1.0], 0.0),
        ];

        let model = DecisionTree::fit(training_data, 10, 2);
        assert!(model.is_some());

        let model = model.unwrap();

        // XOR is difficult for decision trees with small dataset
        // Just verify the model can make predictions (not necessarily perfect for XOR)
        let result = model.predict(&[0.0, 0.0]);
        assert!(result.is_some());

        let result = model.predict(&[1.0, 1.0]);
        assert!(result.is_some());
    }

    #[test]
    fn test_decision_tree_linearly_separable() {
        let training_data = vec![
            (vec![1.0, 1.0], 0.0),
            (vec![2.0, 2.0], 0.0),
            (vec![3.0, 3.0], 0.0),
            (vec![5.0, 5.0], 1.0),
            (vec![6.0, 6.0], 1.0),
            (vec![7.0, 7.0], 1.0),
        ];

        let model = DecisionTree::fit(training_data, 10, 2);
        assert!(model.is_some());

        let model = model.unwrap();

        assert_eq!(model.predict(&[1.5, 1.5]), Some(0.0));
        assert_eq!(model.predict(&[5.5, 5.5]), Some(1.0));
    }

    #[test]
    fn test_decision_tree_one_feature() {
        let training_data = vec![
            (vec![1.0], 0.0),
            (vec![2.0], 0.0),
            (vec![3.0], 0.0),
            (vec![5.0], 1.0),
            (vec![6.0], 1.0),
            (vec![7.0], 1.0),
        ];

        let model = DecisionTree::fit(training_data, 10, 2);
        assert!(model.is_some());

        let model = model.unwrap();

        assert_eq!(model.predict(&[2.5]), Some(0.0));
        assert_eq!(model.predict(&[5.5]), Some(1.0));
    }

    #[test]
    fn test_decision_tree_multiple_classes() {
        let training_data = vec![
            (vec![1.0, 1.0], 0.0),
            (vec![2.0, 2.0], 0.0),
            (vec![5.0, 5.0], 1.0),
            (vec![6.0, 6.0], 1.0),
            (vec![9.0, 9.0], 2.0),
            (vec![10.0, 10.0], 2.0),
        ];

        let model = DecisionTree::fit(training_data, 10, 2);
        assert!(model.is_some());

        let model = model.unwrap();

        assert_eq!(model.predict(&[1.5, 1.5]), Some(0.0));
        assert_eq!(model.predict(&[5.5, 5.5]), Some(1.0));
        assert_eq!(model.predict(&[9.5, 9.5]), Some(2.0));
    }

    #[test]
    fn test_decision_tree_empty_training_data() {
        let training_data = vec![];
        let model = DecisionTree::fit(training_data, 10, 2);
        assert_eq!(model, None);
    }

    #[test]
    fn test_decision_tree_empty_features() {
        let training_data = vec![(vec![], 0.0), (vec![], 1.0)];
        let model = DecisionTree::fit(training_data, 10, 2);
        assert_eq!(model, None);
    }

    #[test]
    fn test_decision_tree_max_depth() {
        let training_data = vec![
            (vec![0.0, 0.0], 0.0),
            (vec![0.0, 1.0], 1.0),
            (vec![1.0, 0.0], 1.0),
            (vec![1.0, 1.0], 0.0),
        ];

        let model = DecisionTree::fit(training_data, 1, 2);
        assert!(model.is_some());

        let model = model.unwrap();
        let result = model.predict(&[0.5, 0.5]);
        assert!(result.is_some());
    }

    #[test]
    fn test_decision_tree_min_samples_split() {
        let training_data = vec![
            (vec![1.0, 1.0], 0.0),
            (vec![2.0, 2.0], 0.0),
            (vec![5.0, 5.0], 1.0),
            (vec![6.0, 6.0], 1.0),
        ];

        let model = DecisionTree::fit(training_data, 10, 10);
        assert!(model.is_some());

        let model = model.unwrap();
        let result = model.predict(&[1.5, 1.5]);
        assert!(result.is_some());
    }

    #[test]
    fn test_decision_tree_predict_batch() {
        let training_data = vec![
            (vec![1.0, 1.0], 0.0),
            (vec![2.0, 2.0], 0.0),
            (vec![5.0, 5.0], 1.0),
            (vec![6.0, 6.0], 1.0),
        ];

        let model = DecisionTree::fit(training_data, 10, 2);
        assert!(model.is_some());

        let model = model.unwrap();

        let test_points = vec![vec![1.5, 1.5], vec![5.5, 5.5]];
        let predictions = model.predict_batch(&test_points);

        assert_eq!(predictions.len(), 2);
        assert_eq!(predictions[0], Some(0.0));
        assert_eq!(predictions[1], Some(1.0));
    }

    #[test]
    fn test_decision_tree_convenience_function() {
        let training_data = vec![
            (vec![1.0, 1.0], 0.0),
            (vec![2.0, 2.0], 0.0),
            (vec![5.0, 5.0], 1.0),
            (vec![6.0, 6.0], 1.0),
        ];

        let result = decision_tree(training_data, vec![1.5, 1.5], 10, 2);
        assert_eq!(result, Some(0.0));

        let training_data = vec![
            (vec![1.0, 1.0], 0.0),
            (vec![2.0, 2.0], 0.0),
            (vec![5.0, 5.0], 1.0),
            (vec![6.0, 6.0], 1.0),
        ];

        let result = decision_tree(training_data, vec![5.5, 5.5], 10, 2);
        assert_eq!(result, Some(1.0));
    }

    #[test]
    fn test_calculate_entropy() {
        let labels = vec![0.0, 0.0, 0.0, 0.0];
        let entropy = calculate_entropy(&labels);
        assert!((entropy - 0.0).abs() < 1e-10);

        let labels = vec![0.0, 0.0, 1.0, 1.0];
        let entropy = calculate_entropy(&labels);
        assert!((entropy - 1.0).abs() < 1e-10);

        let labels = vec![0.0, 1.0, 2.0];
        let entropy = calculate_entropy(&labels);
        assert!(entropy > 0.0 && entropy < 2.0);
    }

    #[test]
    fn test_get_majority_class() {
        let labels = vec![0.0, 0.0, 1.0, 1.0, 0.0];
        let majority = get_majority_class(&labels);
        assert_eq!(majority, 0.0);

        let labels = vec![1.0, 1.0, 2.0, 2.0, 2.0];
        let majority = get_majority_class(&labels);
        assert_eq!(majority, 2.0);
    }
}
