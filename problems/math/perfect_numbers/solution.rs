pub fn is_perfect_number(num: usize) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (is_perfect_number)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement is_perfect_number");
}

pub fn perfect_numbers(max: usize) -> Vec<usize> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (perfect_numbers)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement perfect_numbers");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert!(is_perfect_number(6));
        assert!(is_perfect_number(28));
        assert!(is_perfect_number(496));
        assert!(is_perfect_number(8128));

        assert!(!is_perfect_number(5));
        assert!(!is_perfect_number(86));
        assert!(!is_perfect_number(497));
        assert!(!is_perfect_number(8120));

        assert_eq!(perfect_numbers(10), vec![6]);
        assert_eq!(perfect_numbers(100), vec![6, 28]);
        assert_eq!(perfect_numbers(496), vec![6, 28, 496]);
        assert_eq!(perfect_numbers(1000), vec![6, 28, 496]);
    }
}
