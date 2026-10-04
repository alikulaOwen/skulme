use num_bigint::BigUint;
use num_traits::One;
#[allow(unused_imports)]
use std::str::FromStr;

pub fn factorial(number: u64) -> u64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (factorial)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement factorial");
}

pub fn factorial_recursive(n: u64) -> u64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (factorial_recursive)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement factorial_recursive");
}

pub fn factorial_bigmath(num: u32) -> BigUint {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (factorial_bigmath)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement factorial_bigmath");
}

// Module for tests

#[cfg(test)]
mod tests {
    use super::*;

    // Test cases for the iterative factorial function
    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(6), 720);
        assert_eq!(factorial(10), 3628800);
        assert_eq!(factorial(20), 2432902008176640000);
    }

    // Test cases for the recursive factorial function
    #[test]
    fn test_factorial_recursive() {
        assert_eq!(factorial_recursive(0), 1);
        assert_eq!(factorial_recursive(1), 1);
        assert_eq!(factorial_recursive(6), 720);
        assert_eq!(factorial_recursive(10), 3628800);
        assert_eq!(factorial_recursive(20), 2432902008176640000);
    }

    #[test]
    fn basic_factorial() {
        assert_eq!(factorial_bigmath(10), BigUint::from_str("3628800").unwrap());
        assert_eq!(
            factorial_bigmath(50),
            BigUint::from_str("30414093201713378043612608166064768844377641568960512000000000000")
                .unwrap()
        );
    }
}
