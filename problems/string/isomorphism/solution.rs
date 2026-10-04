//! This module provides functionality to determine whether two strings are isomorphic.
//!
//! Two strings are considered isomorphic if the characters in one string can be replaced
//! by some mapping relation to obtain the other string.
use std::collections::HashMap;

/// Determines whether two strings are isomorphic.
///
/// # Arguments
///
/// * `s` - The first string.
/// * `t` - The second string.
///
/// # Returns
///
/// `true` if the strings are isomorphic, `false` otherwise.
pub fn is_isomorphic(s: &str, t: &str) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (is_isomorphic)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement is_isomorphic");
}

/// Checks the mapping between two characters and updates the map.
///
/// # Arguments
///
/// * `map` - The HashMap to store the mapping.
/// * `key` - The key character.
/// * `value` - The value character.
///
/// # Returns
///
/// `true` if the mapping is consistent, `false` otherwise.
fn check_mapping(map: &mut HashMap<char, char>, key: char, value: char) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (check_mapping)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement check_mapping");
}


#[cfg(test)]
mod tests {
    use super::is_isomorphic;
    macro_rules! test_is_isomorphic {
        ($($name:ident: $inputs:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (s, t, expected) = $inputs;
                assert_eq!(is_isomorphic(s, t), expected);
                assert_eq!(is_isomorphic(t, s), expected);
                assert!(is_isomorphic(s, s));
                assert!(is_isomorphic(t, t));
            }
        )*
        }
    }
    test_is_isomorphic! {
        isomorphic: ("egg", "add", true),
        isomorphic_long: ("abcdaabdcdbbabababacdadad", "AbCdAAbdCdbbAbAbAbACdAdAd", true),
        not_isomorphic: ("egg", "adc", false),
        non_isomorphic_long: ("abcdaabdcdbbabababacdadad", "AACdAAbdCdbbAbAbAbACdAdAd", false),
        isomorphic_unicode: ("天苍苍", "野茫茫", true),
        isomorphic_unicode_different_byte_size: ("abb", "野茫茫", true),
        empty: ("", "", true),
        different_length: ("abc", "abcd", false),
    }
}
