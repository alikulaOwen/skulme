pub fn generate_suffix_array_manber_myers(input: &str) -> Vec<usize> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (generate_suffix_array_manber_myers)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement generate_suffix_array_manber_myers");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suffix_array() {
        let input = "banana";
        let expected_result = vec![5, 3, 1, 0, 4, 2];
        assert_eq!(generate_suffix_array_manber_myers(input), expected_result);
    }

    #[test]
    fn test_empty_string() {
        let input = "";
        let expected_result: Vec<usize> = Vec::new();
        assert_eq!(generate_suffix_array_manber_myers(input), expected_result);
    }

    #[test]
    fn test_single_character() {
        let input = "a";
        let expected_result = vec![0];
        assert_eq!(generate_suffix_array_manber_myers(input), expected_result);
    }
    #[test]
    fn test_repeating_characters() {
        let input = "zzzzzz";
        let expected_result = vec![5, 4, 3, 2, 1, 0];
        assert_eq!(generate_suffix_array_manber_myers(input), expected_result);
    }

    #[test]
    fn test_long_string() {
        let input = "abcdefghijklmnopqrstuvwxyz";
        let expected_result: Vec<usize> = (0..26).collect();
        assert_eq!(generate_suffix_array_manber_myers(input), expected_result);
    }

    #[test]
    fn test_mix_of_characters() {
        let input = "abracadabra!";
        let expected_result = vec![11, 10, 7, 0, 3, 5, 8, 1, 4, 6, 9, 2];
        assert_eq!(generate_suffix_array_manber_myers(input), expected_result);
    }

    #[test]
    fn test_whitespace_characters() {
        let input = " hello world ";
        let expected_result = vec![12, 0, 6, 11, 2, 1, 10, 3, 4, 5, 8, 9, 7];
        assert_eq!(generate_suffix_array_manber_myers(input), expected_result);
    }
}
