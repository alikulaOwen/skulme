//! This module provides functions for finding the shortest palindrome
//! that can be formed by adding characters to the left of a given string.
//! References
//!
//! - [KMP](https://www.scaler.com/topics/data-structures/kmp-algorithm/)
//! - [Prefix Functions and KPM](https://oi-wiki.org/string/kmp/)

/// Finds the shortest palindrome that can be formed by adding characters
/// to the left of the given string `s`.
///
/// # Arguments
///
/// * `s` - A string slice that holds the input string.
///
/// # Returns
///
/// Returns a new string that is the shortest palindrome, formed by adding
/// the necessary characters to the beginning of `s`.
pub fn shortest_palindrome(s: &str) -> String {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (shortest_palindrome)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement shortest_palindrome");
}

/// Computes the suffix table used for the KMP (Knuth-Morris-Pratt) string
/// matching algorithm.
///
/// # Arguments
///
/// * `chars` - A slice of characters for which the suffix table is computed.
///
/// # Returns
///
/// Returns a vector of `usize` representing the suffix table. Each element
/// at index `i` indicates the longest proper suffix which is also a proper
/// prefix of the substring `chars[0..=i]`.
pub fn compute_suffix(chars: &[char]) -> Vec<usize> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (compute_suffix)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement compute_suffix");
}

/// Computes the prefix matches of the original string against its reversed
/// version using the suffix table.
///
/// # Arguments
///
/// * `original` - A slice of characters representing the original string.
/// * `reversed` - A slice of characters representing the reversed string.
/// * `suffix` - A slice containing the suffix table computed for the original string.
///
/// # Returns
///
/// Returns a vector of `usize` where each element at index `i` indicates the
/// length of the longest prefix of `original` that matches a suffix of
/// `reversed[0..=i]`.
pub fn compute_prefix_match(original: &[char], reversed: &[char], suffix: &[usize]) -> Vec<usize> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (compute_prefix_match)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement compute_prefix_match");
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::string::is_palindrome;

    macro_rules! test_shortest_palindrome {
        ($($name:ident: $inputs:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $inputs;
                    assert!(is_palindrome(expected));
                    assert_eq!(shortest_palindrome(input), expected);
                    assert_eq!(shortest_palindrome(expected), expected);
                }
            )*
        }
    }

    test_shortest_palindrome! {
        empty: ("", ""),
        extend_left_1: ("aacecaaa", "aaacecaaa"),
        extend_left_2: ("abcd", "dcbabcd"),
        unicode_1: ("അ", "അ"),
        unicode_2: ("a牛", "牛a牛"),
        single_char: ("x", "x"),
        already_palindrome: ("racecar", "racecar"),
        extend_left_3: ("abcde", "edcbabcde"),
        extend_left_4: ("abca", "acbabca"),
        long_string: ("abcdefg", "gfedcbabcdefg"),
        repetitive: ("aaaaa", "aaaaa"),
        complex: ("abacdfgdcaba", "abacdgfdcabacdfgdcaba"),
    }
}
