// https://en.wikipedia.org/wiki/Square_pyramidal_number
// 1² + 2² + ... = ... (total)

pub fn square_pyramidal_number(n: u64) -> u64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (square_pyramidal_number)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement square_pyramidal_number");
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test0() {
        assert_eq!(0, square_pyramidal_number(0));
        assert_eq!(1, square_pyramidal_number(1));
        assert_eq!(5, square_pyramidal_number(2));
        assert_eq!(14, square_pyramidal_number(3));
    }
}
