/*
Finds the product of two numbers using Karatsuba Algorithm
 */
use std::cmp::max;
const TEN: i128 = 10;

pub fn multiply(num1: i128, num2: i128) -> i128 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (multiply)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement multiply");
}

fn _multiply(num1: i128, num2: i128) -> i128 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (_multiply)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement _multiply");
}

fn normalize(mut a: String, n: usize) -> String {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (normalize)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement normalize");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let n1: i128 = 314159265;
        let n2: i128 = 314159265;
        let ans = multiply(n1, n2);
        assert_eq!(ans, n1 * n2);
    }

    #[test]
    fn test_2() {
        let n1: i128 = 3141592653589793232;
        let n2: i128 = 2718281828459045233;
        let ans = multiply(n1, n2);
        assert_eq!(ans, n1 * n2);
    }

    #[test]
    fn test_3() {
        let n1: i128 = 123456789;
        let n2: i128 = 101112131415;
        let ans = multiply(n1, n2);
        assert_eq!(ans, n1 * n2);
    }
}
