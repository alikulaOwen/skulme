//! =============================================================================
//! Architect's Balanced Bridges: Parentheses Generator
//! CATEGORY: backtracking
//! =============================================================================
//!
//! -----------------------------------------------------------------------------
//! 1. REAL-WORLD STORY & CONTEXT (WHAT IS THIS?)
//! -----------------------------------------------------------------------------
//! Imagine you are an architect designing suspended skybridges between skyscrapers:
//! - An open bracket '(' anchors a suspension cable from Tower A.
//! - A close bracket ')' locks that cable into Tower B.
//! - You can NEVER place a close bracket without a matching open cable already in place!
//!
//! -----------------------------------------------------------------------------
//! 2. GUIDED HINTING QUESTIONS AS YOUR PROBLEM SPECIFICATION
//! -----------------------------------------------------------------------------
//! ❓ Q1: When can we add an open bracket? (When open_count < n)
//! ❓ Q2: When can we add a close bracket? (When close_count < open_count)
//! ❓ Q3: When is the bridge complete? (When current.len() == 2 * n)
//! -----------------------------------------------------------------------------

/// Generates all combinations of well-formed parentheses given a non-negative integer `n`.
///
/// This function uses backtracking to generate all possible combinations of well-formed
/// parentheses. The resulting combinations are returned as a vector of strings.
///
/// # Arguments
///
/// * `n` - A non-negative integer representing the number of pairs of parentheses.
pub fn generate_parentheses(n: usize) -> Vec<String> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (generate_parentheses)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement generate_parentheses");
}

/// Helper function for generating parentheses recursively.
///
/// This function is called recursively to build combinations of well-formed parentheses.
/// It tracks the number of open and close parentheses added so far and adds a new parenthesis
/// if it's valid to do so.
///
/// # Arguments
///
/// * `current` - The current string of parentheses being built.
/// * `open_count` - The count of open parentheses in the current string.
/// * `close_count` - The count of close parentheses in the current string.
/// * `n` - The total number of pairs of parentheses to be generated.
/// * `result` - A mutable reference to the vector storing the generated combinations.
fn generate(
    current: &str,
    open_count: usize,
    close_count: usize,
    n: usize,
    result: &mut Vec<String>,
) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (generate)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement generate");
}


#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! generate_parentheses_tests {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (n, expected_result) = $test_case;
                    assert_eq!(generate_parentheses(n), expected_result);
                }
            )*
        };
    }

    generate_parentheses_tests! {
        test_generate_parentheses_0: (0, Vec::<String>::new()),
        test_generate_parentheses_1: (1, vec!["()"]),
        test_generate_parentheses_2: (2, vec!["(())", "()()"]),
        test_generate_parentheses_3: (3, vec!["((()))", "(()())", "(())()", "()(())", "()()()"]),
        test_generate_parentheses_4: (4, vec!["(((())))", "((()()))", "((())())", "((()))()", "(()(()))", "(()()())", "(()())()", "(())(())", "(())()()", "()((()))", "()(()())", "()(())()", "()()(())", "()()()()"]),
    }
}
