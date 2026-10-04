/// returns the greatest common divisor of n numbers
pub fn gcd(nums: &[usize]) -> usize {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (gcd)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement gcd");
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (gcd_of_two_numbers)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement gcd_of_two_numbers");
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(gcd(&[1, 2, 3, 4, 5]), 1);
        assert_eq!(gcd(&[2, 4, 6, 8, 10]), 2);
        assert_eq!(gcd(&[3, 6, 9, 12, 15]), 3);
        assert_eq!(gcd(&[10]), 10);
        assert_eq!(gcd(&[21, 110]), 1);
    }
}
