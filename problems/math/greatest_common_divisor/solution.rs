/// Greatest Common Divisor.
///
/// greatest_common_divisor(num1, num2) returns the greatest number of num1 and num2.
///
/// Wikipedia reference: https://en.wikipedia.org/wiki/Greatest_common_divisor
/// gcd(a, b) = gcd(a, -b) = gcd(-a, b) = gcd(-a, -b) by definition of divisibility
use std::cmp::{max, min};

pub fn greatest_common_divisor_recursive(a: i64, b: i64) -> i64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (greatest_common_divisor_recursive)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement greatest_common_divisor_recursive");
}

pub fn greatest_common_divisor_iterative(mut a: i64, mut b: i64) -> i64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (greatest_common_divisor_iterative)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement greatest_common_divisor_iterative");
}

pub fn greatest_common_divisor_stein(a: u64, b: u64) -> u64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (greatest_common_divisor_stein)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement greatest_common_divisor_stein");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_number_recursive() {
        assert_eq!(greatest_common_divisor_recursive(4, 16), 4);
        assert_eq!(greatest_common_divisor_recursive(16, 4), 4);
        assert_eq!(greatest_common_divisor_recursive(3, 5), 1);
        assert_eq!(greatest_common_divisor_recursive(40, 40), 40);
        assert_eq!(greatest_common_divisor_recursive(27, 12), 3);
    }

    #[test]
    fn positive_number_iterative() {
        assert_eq!(greatest_common_divisor_iterative(4, 16), 4);
        assert_eq!(greatest_common_divisor_iterative(16, 4), 4);
        assert_eq!(greatest_common_divisor_iterative(3, 5), 1);
        assert_eq!(greatest_common_divisor_iterative(40, 40), 40);
        assert_eq!(greatest_common_divisor_iterative(27, 12), 3);
    }

    #[test]
    fn positive_number_stein() {
        assert_eq!(greatest_common_divisor_stein(4, 16), 4);
        assert_eq!(greatest_common_divisor_stein(16, 4), 4);
        assert_eq!(greatest_common_divisor_stein(3, 5), 1);
        assert_eq!(greatest_common_divisor_stein(40, 40), 40);
        assert_eq!(greatest_common_divisor_stein(27, 12), 3);
    }

    #[test]
    fn negative_number_recursive() {
        assert_eq!(greatest_common_divisor_recursive(-32, -8), 8);
        assert_eq!(greatest_common_divisor_recursive(-8, -32), 8);
        assert_eq!(greatest_common_divisor_recursive(-3, -5), 1);
        assert_eq!(greatest_common_divisor_recursive(-40, -40), 40);
        assert_eq!(greatest_common_divisor_recursive(-12, -27), 3);
    }

    #[test]
    fn negative_number_iterative() {
        assert_eq!(greatest_common_divisor_iterative(-32, -8), 8);
        assert_eq!(greatest_common_divisor_iterative(-8, -32), 8);
        assert_eq!(greatest_common_divisor_iterative(-3, -5), 1);
        assert_eq!(greatest_common_divisor_iterative(-40, -40), 40);
        assert_eq!(greatest_common_divisor_iterative(-12, -27), 3);
    }

    #[test]
    fn mix_recursive() {
        assert_eq!(greatest_common_divisor_recursive(0, -5), 5);
        assert_eq!(greatest_common_divisor_recursive(-5, 0), 5);
        assert_eq!(greatest_common_divisor_recursive(-64, 32), 32);
        assert_eq!(greatest_common_divisor_recursive(-32, 64), 32);
        assert_eq!(greatest_common_divisor_recursive(-40, 40), 40);
        assert_eq!(greatest_common_divisor_recursive(12, -27), 3);
    }

    #[test]
    fn mix_iterative() {
        assert_eq!(greatest_common_divisor_iterative(0, -5), 5);
        assert_eq!(greatest_common_divisor_iterative(-5, 0), 5);
        assert_eq!(greatest_common_divisor_iterative(-64, 32), 32);
        assert_eq!(greatest_common_divisor_iterative(-32, 64), 32);
        assert_eq!(greatest_common_divisor_iterative(-40, 40), 40);
        assert_eq!(greatest_common_divisor_iterative(12, -27), 3);
    }
}
