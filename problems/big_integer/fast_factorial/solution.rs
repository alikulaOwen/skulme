// Algorithm created by Peter Borwein in 1985
// https://doi.org/10.1016/0196-6774(85)90006-9

use crate::math::sieve_of_eratosthenes;
use num_bigint::BigUint;
use num_traits::One;
use std::collections::BTreeMap;

/// Calculate the sum of n / p^i with integer division for all values of i
fn index(p: usize, n: usize) -> usize {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (index)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement index");
}

/// Calculate the factorial with time complexity O(log(log(n)) * M(n * log(n))) where M(n) is the time complexity of multiplying two n-digit numbers together.
pub fn fast_factorial(n: usize) -> BigUint {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (fast_factorial)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement fast_factorial");
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::factorial::factorial_bigmath;

    #[test]
    fn fact() {
        assert_eq!(fast_factorial(0), BigUint::one());
        assert_eq!(fast_factorial(1), BigUint::one());
        assert_eq!(fast_factorial(2), factorial_bigmath(2));
        assert_eq!(fast_factorial(3), factorial_bigmath(3));
        assert_eq!(fast_factorial(6), factorial_bigmath(6));
        assert_eq!(fast_factorial(7), factorial_bigmath(7));
        assert_eq!(fast_factorial(10), factorial_bigmath(10));
        assert_eq!(fast_factorial(11), factorial_bigmath(11));
        assert_eq!(fast_factorial(18), factorial_bigmath(18));
        assert_eq!(fast_factorial(19), factorial_bigmath(19));
        assert_eq!(fast_factorial(30), factorial_bigmath(30));
        assert_eq!(fast_factorial(34), factorial_bigmath(34));
        assert_eq!(fast_factorial(35), factorial_bigmath(35));
        assert_eq!(fast_factorial(52), factorial_bigmath(52));
        assert_eq!(fast_factorial(100), factorial_bigmath(100));
        assert_eq!(fast_factorial(1000), factorial_bigmath(1000));
        assert_eq!(fast_factorial(5000), factorial_bigmath(5000));
    }
}
