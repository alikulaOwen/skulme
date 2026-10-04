/// Marginal Ranking
///
/// The 'average_margin_ranking_loss' function calculates the Margin Ranking loss, which is a
/// loss function used for ranking problems in machine learning.
///
/// ## Formula
///
/// For a pair of values `x_first` and `x_second`, `margin`, and `y_true`,
/// the Margin Ranking loss is calculated as:
///
///  - loss = `max(0, -y_true * (x_first - x_second) + margin)`.
///
/// It returns the average loss by dividing the `total_loss` by total no. of
/// elements.
///
/// Pytorch implementation:
/// https://pytorch.org/docs/stable/generated/torch.nn.MarginRankingLoss.html
/// https://gombru.github.io/2019/04/03/ranking_loss/
/// https://vinija.ai/concepts/loss/#pairwise-ranking-loss
///

pub fn average_margin_ranking_loss(
    x_first: &[f64],
    x_second: &[f64],
    margin: f64,
    y_true: f64,
) -> Result<f64, MarginalRankingLossError> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (average_margin_ranking_loss)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement average_margin_ranking_loss");
}

fn check_input(
    x_first: &[f64],
    x_second: &[f64],
    margin: f64,
    y_true: f64,
) -> Result<(), MarginalRankingLossError> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (check_input)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement check_input");
}

#[derive(Debug, PartialEq, Eq)]
pub enum MarginalRankingLossError {
    InputsHaveDifferentLength,
    EmptyInputs,
    InvalidValues,
    NegativeMargin,
}


#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_with_wrong_inputs {
        ($($name:ident: $inputs:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (vec_a, vec_b, margin, y_true, expected) = $inputs;
                    assert_eq!(average_margin_ranking_loss(&vec_a, &vec_b, margin, y_true), expected);
                    assert_eq!(average_margin_ranking_loss(&vec_b, &vec_a, margin, y_true), expected);
                }
            )*
        }
    }

    test_with_wrong_inputs! {
        invalid_length0: (vec![1.0, 2.0, 3.0], vec![2.0, 3.0], 1.0, 1.0, Err(MarginalRankingLossError::InputsHaveDifferentLength)),
        invalid_length1: (vec![1.0, 2.0], vec![2.0, 3.0, 4.0], 1.0, 1.0, Err(MarginalRankingLossError::InputsHaveDifferentLength)),
        invalid_length2: (vec![], vec![1.0, 2.0, 3.0], 1.0, 1.0, Err(MarginalRankingLossError::InputsHaveDifferentLength)),
        invalid_length3: (vec![1.0, 2.0, 3.0], vec![], 1.0, 1.0, Err(MarginalRankingLossError::InputsHaveDifferentLength)),
        invalid_values: (vec![1.0, 2.0, 3.0], vec![2.0, 3.0, 4.0], -1.0, 1.0, Err(MarginalRankingLossError::NegativeMargin)),
        invalid_y_true: (vec![1.0, 2.0, 3.0], vec![2.0, 3.0, 4.0], 1.0, 2.0, Err(MarginalRankingLossError::InvalidValues)),
        empty_inputs: (vec![], vec![], 1.0, 1.0, Err(MarginalRankingLossError::EmptyInputs)),
    }

    macro_rules! test_average_margin_ranking_loss {
        ($($name:ident: $inputs:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (x_first, x_second, margin, y_true, expected) = $inputs;
                    assert_eq!(average_margin_ranking_loss(&x_first, &x_second, margin, y_true), Ok(expected));
                }
            )*
        }
    }

    test_average_margin_ranking_loss! {
        set_0: (vec![1.0, 2.0, 3.0], vec![2.0, 3.0, 4.0], 1.0, -1.0, 0.0),
        set_1: (vec![1.0, 2.0, 3.0], vec![2.0, 3.0, 4.0], 1.0, 1.0, 2.0),
        set_2: (vec![1.0, 2.0, 3.0], vec![1.0, 2.0, 3.0], 0.0, 1.0, 0.0),
        set_3: (vec![4.0, 5.0, 6.0], vec![1.0, 2.0, 3.0], 1.0, -1.0, 4.0),
    }
}
