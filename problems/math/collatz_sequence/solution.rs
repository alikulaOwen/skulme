// collatz conjecture : https://en.wikipedia.org/wiki/Collatz_conjecture
pub fn sequence(mut n: usize) -> Option<Vec<usize>> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (sequence)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement sequence");
}


#[cfg(test)]
mod tests {
    use super::sequence;

    #[test]
    fn validity_check() {
        assert_eq!(sequence(10).unwrap(), [10, 5, 16, 8, 4, 2, 1]);
        assert_eq!(
            sequence(15).unwrap(),
            [15, 46, 23, 70, 35, 106, 53, 160, 80, 40, 20, 10, 5, 16, 8, 4, 2, 1]
        );
        assert_eq!(sequence(0).unwrap_or_else(|| vec![0]), [0]);
    }
}
