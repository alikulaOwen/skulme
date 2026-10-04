//! Provides functions to calculate the Levenshtein distance between two strings.
//!
//! The Levenshtein distance is a measure of the similarity between two strings by calculating the minimum number of single-character
//! edits (insertions, deletions, or substitutions) required to change one string into the other.

use std::cmp::min;

/// Calculates the Levenshtein distance between two strings using a naive dynamic programming approach.
///
/// The Levenshtein distance is a measure of the similarity between two strings by calculating the minimum number of single-character
/// edits (insertions, deletions, or substitutions) required to change one string into the other.
///
/// # Arguments
///
/// * `string1` - A reference to the first string.
/// * `string2` - A reference to the second string.
///
/// # Returns
///
/// The Levenshtein distance between the two input strings.
///
/// This function computes the Levenshtein distance by constructing a dynamic programming matrix and iteratively filling it in.
/// It follows the standard top-to-bottom, left-to-right approach for filling in the matrix.
///
/// # Complexity
///
/// - Time complexity: O(nm),
/// - Space complexity: O(nm),
///
/// where n and m are lengths of `string1` and `string2`.
///
/// Note that this implementation uses a straightforward dynamic programming approach without any space optimization.
/// It may consume more memory for larger input strings compared to the optimized version.
pub fn naive_levenshtein_distance(string1: &str, string2: &str) -> usize {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (naive_levenshtein_distance)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement naive_levenshtein_distance");
}

/// Calculates the Levenshtein distance between two strings using an optimized dynamic programming approach.
///
/// This edit distance is defined as 1 point per insertion, substitution, or deletion required to make the strings equal.
///
/// # Arguments
///
/// * `string1` - The first string.
/// * `string2` - The second string.
///
/// # Returns
///
/// The Levenshtein distance between the two input strings.
/// For a detailed explanation, check the example on [Wikipedia](https://en.wikipedia.org/wiki/Levenshtein_distance).
/// This function iterates over the bytes in the string, so it may not behave entirely as expected for non-ASCII strings.
///
/// Note that this implementation utilizes an optimized dynamic programming approach, significantly reducing the space complexity from O(nm) to O(n), where n and m are the lengths of `string1` and `string2`.
///
/// Additionally, it minimizes space usage by leveraging the shortest string horizontally and the longest string vertically in the computation matrix.
///
/// # Complexity
///
/// - Time complexity: O(nm),
/// - Space complexity: O(n),
///
/// where n and m are lengths of `string1` and `string2`.
pub fn optimized_levenshtein_distance(string1: &str, string2: &str) -> usize {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (optimized_levenshtein_distance)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement optimized_levenshtein_distance");
}

#[inline]
fn _min3<T: Ord>(a: T, b: T, c: T) -> T {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (_min3)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement _min3");
}


#[cfg(test)]
mod tests {
    const LEVENSHTEIN_DISTANCE_TEST_CASES: &[(&str, &str, usize)] = &[
        ("", "", 0),
        ("Hello, World!", "Hello, World!", 0),
        ("", "Rust", 4),
        ("horse", "ros", 3),
        ("tan", "elephant", 6),
        ("execute", "intention", 8),
    ];

    macro_rules! levenshtein_distance_tests {
        ($function:ident) => {
            mod $function {
                use super::*;

                fn run_test_case(string1: &str, string2: &str, expected_distance: usize) {
                    assert_eq!(super::super::$function(string1, string2), expected_distance);
                    assert_eq!(super::super::$function(string2, string1), expected_distance);
                    assert_eq!(super::super::$function(string1, string1), 0);
                    assert_eq!(super::super::$function(string2, string2), 0);
                }

                #[test]
                fn test_levenshtein_distance() {
                    for &(string1, string2, expected_distance) in
                        LEVENSHTEIN_DISTANCE_TEST_CASES.iter()
                    {
                        run_test_case(string1, string2, expected_distance);
                    }
                }
            }
        };
    }

    levenshtein_distance_tests!(naive_levenshtein_distance);
    levenshtein_distance_tests!(optimized_levenshtein_distance);
}
