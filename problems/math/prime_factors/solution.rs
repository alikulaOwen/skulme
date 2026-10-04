// Finds the prime factors of a number in increasing order, with repetition.

pub fn prime_factors(n: u64) -> Vec<u64> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (prime_factors)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement prime_factors");
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(prime_factors(0), vec![]);
        assert_eq!(prime_factors(1), vec![]);
        assert_eq!(prime_factors(11), vec![11]);
        assert_eq!(prime_factors(25), vec![5, 5]);
        assert_eq!(prime_factors(33), vec![3, 11]);
        assert_eq!(prime_factors(2560), vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 5]);
    }
}
