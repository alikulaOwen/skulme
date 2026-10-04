use super::{mersenne_primes::is_mersenne_prime, prime_numbers::prime_numbers};
use std::convert::TryInto;

/*
    Generates a list of perfect numbers till `num` using the Lucas Lehmer test algorithm.
    url : https://en.wikipedia.org/wiki/Lucas%E2%80%93Lehmer_primality_test
*/
pub fn generate_perfect_numbers(num: usize) -> Vec<usize> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (generate_perfect_numbers)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement generate_perfect_numbers");
}

// Gets an approximate limit for the generate_perfect_numbers function
fn get_prime_limit(num: usize) -> usize {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (get_prime_limit)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement get_prime_limit");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perfect_numbers_till_n() {
        let n = 335564540;
        assert_eq!(generate_perfect_numbers(n), [6, 28, 496, 8128, 33550336]);
        assert_eq!(generate_perfect_numbers(40), [6, 28]);
        assert_eq!(generate_perfect_numbers(0), []);
    }
}
