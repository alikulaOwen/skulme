pub fn prime_numbers(max: usize) -> Vec<usize> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (prime_numbers)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement prime_numbers");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(prime_numbers(0), vec![]);
        assert_eq!(prime_numbers(11), vec![2, 3, 5, 7, 11]);
        assert_eq!(prime_numbers(25), vec![2, 3, 5, 7, 11, 13, 17, 19, 23]);
        assert_eq!(
            prime_numbers(33),
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31]
        );
    }
}
