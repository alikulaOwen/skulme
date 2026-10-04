fn update_step(a: &mut i32, old_a: &mut i32, quotient: i32) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (update_step)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement update_step");
}

pub fn extended_euclidean_algorithm(a: i32, b: i32) -> (i32, i32, i32) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (extended_euclidean_algorithm)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement extended_euclidean_algorithm");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(extended_euclidean_algorithm(101, 13), (1, 4, -31));
        assert_eq!(extended_euclidean_algorithm(123, 19), (1, -2, 13));
        assert_eq!(extended_euclidean_algorithm(25, 36), (1, 13, -9));
        assert_eq!(extended_euclidean_algorithm(69, 54), (3, -7, 9));
        assert_eq!(extended_euclidean_algorithm(55, 79), (1, 23, -16));
        assert_eq!(extended_euclidean_algorithm(33, 44), (11, -1, 1));
        assert_eq!(extended_euclidean_algorithm(50, 70), (10, 3, -2));
    }
}
