// Author : cyrixninja
// Sylvester Series     :  Calculates the nth number in Sylvester's sequence.
// Wikipedia Reference  :  https://en.wikipedia.org/wiki/Sylvester%27s_sequence
// Other References     :  https://the-algorithms.com/algorithm/sylvester-sequence?lang=python

pub fn sylvester(number: i32) -> i128 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (sylvester)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement sylvester");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sylvester() {
        assert_eq!(sylvester(8), 113423713055421844361000443_i128);
    }

    #[test]
    #[should_panic(expected = "The input value of [n=-1] has to be > 0")]
    fn test_sylvester_negative() {
        sylvester(-1);
    }
}
