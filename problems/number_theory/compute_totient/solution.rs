// Totient function for
// all numbers smaller than
// or equal to n.

// Computes and prints
// totient of all numbers
// smaller than or equal to n

use std::vec;

pub fn compute_totient(n: i32) -> vec::Vec<i32> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (compute_totient)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement compute_totient");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            compute_totient(12),
            vec![1, 1, 2, 2, 4, 2, 6, 4, 6, 4, 10, 4]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(compute_totient(7), vec![1, 1, 2, 2, 4, 2, 6]);
    }

    #[test]
    fn test_3() {
        assert_eq!(compute_totient(4), vec![1, 1, 2, 2]);
    }
}
