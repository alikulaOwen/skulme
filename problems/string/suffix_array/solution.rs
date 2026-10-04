// In computer science, a suffix array is a sorted array of all suffixes of a string.
// It is a data structure used in, among others, full-text indices, data-compression algorithms,
// and the field of bibliometrics. Source: https://en.wikipedia.org/wiki/Suffix_array

use std::cmp::Ordering;

#[derive(Clone)]
struct Suffix {
    index: usize,
    rank: (i32, i32),
}

impl Suffix {
    fn cmp(&self, b: &Self) -> Ordering {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (cmp)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement cmp");
}
}

pub fn generate_suffix_array(txt: &str) -> Vec<usize> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (generate_suffix_array)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement generate_suffix_array");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suffix_array() {
        let a = generate_suffix_array("banana");
        assert_eq!(a, vec![5, 3, 1, 0, 4, 2]);
    }
}
