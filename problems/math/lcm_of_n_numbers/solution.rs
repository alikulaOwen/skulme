// returns the least common multiple of n numbers

pub fn lcm(nums: &[usize]) -> usize {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (lcm)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement lcm");
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
        assert_eq!(lcm(&[1, 2, 3, 4, 5]), 60);
        assert_eq!(lcm(&[2, 4, 6, 8, 10]), 120);
        assert_eq!(lcm(&[3, 6, 9, 12, 15]), 180);
        assert_eq!(lcm(&[10]), 10);
        assert_eq!(lcm(&[21, 110]), 2310);
    }
}
