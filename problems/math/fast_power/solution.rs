/// fast_power returns the result of base^power mod modulus
pub fn fast_power(mut base: usize, mut power: usize, modulus: usize) -> usize {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (fast_power)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement fast_power");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        const MOD: usize = 1000000007;
        assert_eq!(fast_power(2, 1, MOD), 2);
        assert_eq!(fast_power(2, 2, MOD), 4);
        assert_eq!(fast_power(2, 4, MOD), 16);
        assert_eq!(fast_power(3, 4, MOD), 81);
        assert_eq!(fast_power(2, 100, MOD), 976371285);
    }
}
