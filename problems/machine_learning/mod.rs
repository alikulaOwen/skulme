// Automatically generated category module

pub mod loss_function;
pub mod optimization;
#[path = "cholesky/solution.rs"]
pub mod cholesky;

#[path = "decision_tree/solution.rs"]
pub mod decision_tree;

#[path = "k_means/solution.rs"]
pub mod k_means;

#[path = "k_nearest_neighbors/solution.rs"]
pub mod k_nearest_neighbors;

#[path = "linear_regression/solution.rs"]
pub mod linear_regression;

#[path = "logistic_regression/solution.rs"]
pub mod logistic_regression;

#[path = "naive_bayes/solution.rs"]
pub mod naive_bayes;

#[path = "perceptron/solution.rs"]
pub mod perceptron;

#[path = "principal_component_analysis/solution.rs"]
pub mod principal_component_analysis;

#[path = "random_forest/solution.rs"]
pub mod random_forest;

#[path = "support_vector_classifier/solution.rs"]
pub mod support_vector_classifier;


pub use self::cholesky::cholesky;
pub use self::decision_tree::decision_tree;
pub use self::k_means::k_means;
pub use self::k_nearest_neighbors::k_nearest_neighbors;
pub use self::linear_regression::linear_regression;
pub use self::logistic_regression::logistic_regression;
pub use self::loss_function::{
    average_margin_ranking_loss, hng_loss, huber_loss, kld_loss, mae_loss, mse_loss,
    neg_log_likelihood,
};
pub use self::naive_bayes::naive_bayes;
pub use self::optimization::{gradient_descent, Adam};
pub use self::perceptron::{classify, perceptron};
pub use self::principal_component_analysis::principal_component_analysis;
pub use self::random_forest::random_forest;
pub use self::support_vector_classifier::{Kernel, SVCError, SVC};
