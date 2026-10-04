// Operations based around amicable numbers
// Suports u32 but should be interchangable with other types
// Wikipedia reference: https://en.wikipedia.org/wiki/Amicable_numbers

// Returns vec of amicable pairs below N
// N must be positive
pub fn amicable_pairs_under_n(n: u32) -> Option<Vec<(u32, u32)>> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (amicable_pairs_under_n)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement amicable_pairs_under_n");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_amicable_numbers_below_n() {
        // First 10 amicable numbers, sorted (low, high)
        let expected_result = vec![
            (220, 284),
            (1184, 1210),
            (2620, 2924),
            (5020, 5564),
            (6232, 6368),
            (10744, 10856),
            (12285, 14595),
            (17296, 18416),
            (63020, 76084),
            (66928, 66992),
        ];

        // Generate pairs under 100,000
        let mut result = amicable_pairs_under_n(100_000).unwrap();

        // There should be 13 pairs under 100,000
        assert_eq!(result.len(), 13);

        // Check the first 10 against known values
        result = result[..10].to_vec();
        assert_eq!(result, expected_result);

        // N that does not have any amicable pairs below it, the result should be None
        assert_eq!(amicable_pairs_under_n(100), None);
    }
}
