// Negative Log Likelihood Loss Function
//
// The `neg_log_likelihood` function calculates the Negative Log Likelyhood loss,
// which is a loss function used for classification problems in machine learning.
//
// ## Formula
//
// For a pair of actual and predicted values, represented as vectors `y_true` and
// `y_pred`, the Negative Log Likelihood loss is calculated as:
//
// - loss = `-y_true * log(y_pred) - (1 - y_true) * log(1 - y_pred)`.
//
// It returns the average loss by dividing the `total_loss` by total no. of
// elements.
//
// https://towardsdatascience.com/cross-entropy-negative-log-likelihood-and-all-that-jazz-47a95bd2e81
// http://neuralnetworksanddeeplearning.com/chap3.html
// Derivation of the formula:
// https://medium.com/@bhardwajprakarsh/negative-log-likelihood-loss-why-do-we-use-it-for-binary-classification-7625f9e3c944

pub fn neg_log_likelihood(
    y_true: &[f64],
    y_pred: &[f64],
) -> Result<f64, NegativeLogLikelihoodLossError> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (neg_log_likelihood)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement neg_log_likelihood");
}

#[derive(Debug, PartialEq, Eq)]
pub enum NegativeLogLikelihoodLossError {
    InputsHaveDifferentLength,
    EmptyInputs,
    InvalidValues,
}

fn are_all_values_in_range(values: &[f64]) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (are_all_values_in_range)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement are_all_values_in_range");
}


#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_with_wrong_inputs {
        ($($name:ident: $inputs:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (values_a, values_b, expected_error) = $inputs;
                    assert_eq!(neg_log_likelihood(&values_a, &values_b), expected_error);
                    assert_eq!(neg_log_likelihood(&values_b, &values_a), expected_error);
                }
            )*
        }
    }

    test_with_wrong_inputs! {
        different_length: (vec![0.9, 0.0, 0.8], vec![0.9, 0.1], Err(NegativeLogLikelihoodLossError::InputsHaveDifferentLength)),
        different_length_one_empty: (vec![], vec![0.9, 0.1], Err(NegativeLogLikelihoodLossError::InputsHaveDifferentLength)),
        value_greater_than_1: (vec![1.1, 0.0, 0.8], vec![0.1, 0.2, 0.3], Err(NegativeLogLikelihoodLossError::InvalidValues)),
        value_greater_smaller_than_0: (vec![0.9, 0.0, -0.1], vec![0.1, 0.2, 0.3], Err(NegativeLogLikelihoodLossError::InvalidValues)),
        empty_input: (vec![], vec![], Err(NegativeLogLikelihoodLossError::EmptyInputs)),
    }

    macro_rules! test_neg_log_likelihood {
        ($($name:ident: $inputs:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (actual_values, predicted_values, expected) = $inputs;
                    assert_eq!(neg_log_likelihood(&actual_values, &predicted_values).unwrap(), expected);
                }
            )*
        }
    }

    test_neg_log_likelihood! {
        set_0: (vec![1.0, 0.0, 1.0], vec![0.9, 0.1, 0.8], 0.14462152754328741),
        set_1: (vec![1.0, 0.0, 1.0], vec![0.1, 0.2, 0.3], 1.2432338162113972),
        set_2: (vec![0.0, 1.0, 0.0], vec![0.1, 0.2, 0.3], 0.6904911240102196),
        set_3: (vec![1.0, 0.0, 1.0, 0.0], vec![0.9, 0.1, 0.8, 0.2], 0.164252033486018),
    }
}
