use super::extended_euclidean_algorithm;

fn mod_inv(x: i32, n: i32) -> Option<i32> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (mod_inv)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement mod_inv");
}

pub fn chinese_remainder_theorem(residues: &[i32], modulli: &[i32]) -> Option<i32> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (chinese_remainder_theorem)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement chinese_remainder_theorem");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(chinese_remainder_theorem(&[3, 5, 7], &[2, 3, 1]), Some(5));
        assert_eq!(chinese_remainder_theorem(&[1, 4, 6], &[3, 5, 7]), Some(34));
        assert_eq!(chinese_remainder_theorem(&[1, 4, 6], &[1, 2, 0]), None);
        assert_eq!(chinese_remainder_theorem(&[2, 5, 7], &[6, 9, 15]), None);
    }
}
