// Function to calculate combinations of k elements from a set of n elements
pub fn combinations(n: i64, k: i64) -> i64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (combinations)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement combinations");
}


#[cfg(test)]
mod tests {
    use super::*;

    // Test case for combinations(10, 5)
    #[test]
    fn test_combinations_10_choose_5() {
        assert_eq!(combinations(10, 5), 252);
    }

    // Test case for combinations(6, 3)
    #[test]
    fn test_combinations_6_choose_3() {
        assert_eq!(combinations(6, 3), 20);
    }

    // Test case for combinations(20, 5)
    #[test]
    fn test_combinations_20_choose_5() {
        assert_eq!(combinations(20, 5), 15504);
    }

    // Test case for invalid input (negative values)
    #[test]
    #[should_panic(expected = "Please insert positive values")]
    fn test_combinations_invalid_input() {
        combinations(-5, 10);
    }
}
