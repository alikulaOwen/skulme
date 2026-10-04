pub fn prime_check(num: usize) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (prime_check)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement prime_check");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert!(prime_check(3));
        assert!(prime_check(7));
        assert!(prime_check(11));
        assert!(prime_check(2003));

        assert!(!prime_check(4));
        assert!(!prime_check(6));
        assert!(!prime_check(21));
        assert!(!prime_check(2004));
    }
}
