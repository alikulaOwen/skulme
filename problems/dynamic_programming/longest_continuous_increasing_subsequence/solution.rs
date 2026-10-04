use std::cmp::Ordering;

/// Finds the longest continuous increasing subsequence in a slice.
///
/// Given a slice of elements, this function returns a slice representing
/// the longest continuous subsequence where each element is strictly
/// less than the following element.
///
/// # Arguments
///
/// * `arr` - A reference to a slice of elements
///
/// # Returns
///
/// A subslice of the input, representing the longest continuous increasing subsequence.
/// If there are multiple subsequences of the same length, the function returns the first one found.
pub fn longest_continuous_increasing_subsequence<T: Ord>(arr: &[T]) -> &[T] {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (longest_continuous_increasing_subsequence)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement longest_continuous_increasing_subsequence");
}


#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_cases {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $test_case;
                    assert_eq!(longest_continuous_increasing_subsequence(input), expected);
                }
            )*
        };
    }

    test_cases! {
        empty_array: (&[] as &[isize], &[] as &[isize]),
        single_element: (&[1], &[1]),
        all_increasing: (&[1, 2, 3, 4, 5], &[1, 2, 3, 4, 5]),
        all_decreasing: (&[5, 4, 3, 2, 1], &[5]),
        with_equal_elements: (&[1, 2, 2, 3, 4, 2], &[1, 2, 2, 3, 4]),
        increasing_with_plateau: (&[1, 2, 2, 2, 3, 3, 4], &[1, 2, 2, 2, 3, 3, 4]),
        mixed_elements: (&[5, 4, 3, 4, 2, 1], &[3, 4]),
        alternating_increase_decrease: (&[1, 2, 1, 2, 1, 2], &[1, 2]),
        zigzag: (&[1, 3, 2, 4, 3, 5], &[1, 3]),
        single_negative_element: (&[-1], &[-1]),
        negative_and_positive_mixed: (&[-2, -1, 0, 1, 2, 3], &[-2, -1, 0, 1, 2, 3]),
        increasing_then_decreasing: (&[1, 2, 3, 4, 3, 2, 1], &[1, 2, 3, 4]),
        single_increasing_subsequence_later: (&[3, 2, 1, 1, 2, 3, 4], &[1, 1, 2, 3, 4]),
        longer_subsequence_at_start: (&[5, 6, 7, 8, 9, 2, 3, 4, 5], &[5, 6, 7, 8, 9]),
        longer_subsequence_at_end: (&[2, 3, 4, 10, 5, 6, 7, 8, 9], &[5, 6, 7, 8, 9]),
        longest_subsequence_at_start: (&[2, 3, 4, 5, 1, 0], &[2, 3, 4, 5]),
        longest_subsequence_at_end: (&[1, 7, 2, 3, 4, 5,], &[2, 3, 4, 5]),
        repeated_elements: (&[1, 1, 1, 1, 1], &[1, 1, 1, 1, 1]),
    }
}
