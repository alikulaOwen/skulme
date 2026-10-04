// mersenne prime : https://en.wikipedia.org/wiki/Mersenne_prime
pub fn is_mersenne_prime(n: usize) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (is_mersenne_prime)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement is_mersenne_prime");
}

pub fn get_mersenne_primes(limit: usize) -> Vec<usize> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (get_mersenne_primes)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement get_mersenne_primes");
}


#[cfg(test)]
mod tests {
    use super::{get_mersenne_primes, is_mersenne_prime};

    #[test]
    fn validity_check() {
        assert!(is_mersenne_prime(3));
        assert!(is_mersenne_prime(13));
        assert!(!is_mersenne_prime(32));
    }

    #[allow(dead_code)]
    fn generation_check() {
        assert_eq!(get_mersenne_primes(30), [2, 3, 5, 7, 13, 17, 19]);
    }
}
