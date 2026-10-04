/*
A Rust implementation of the Dutch National Flag sorting algorithm.

Reference implementation: https://github.com/TheAlgorithms/Python/blob/master/sorts/dutch_national_flag_sort.py
More info: https://en.wikipedia.org/wiki/Dutch_national_flag_problem
*/

#[derive(PartialOrd, PartialEq, Eq)]
pub enum Colors {
    Red,   // \
    White, //  | Define the three colors of the Dutch Flag: 🇳🇱
    Blue,  // /
}
use Colors::{Blue, Red, White};

// Algorithm implementation
pub fn dutch_national_flag_sort(mut sequence: Vec<Colors>) -> Vec<Colors> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (dutch_national_flag_sort)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement dutch_national_flag_sort");
}


#[cfg(test)]
mod tests {
    use super::super::is_sorted;
    use super::*;

    #[test]
    fn random_array() {
        let arr = vec![
            Red, Blue, White, White, Blue, Blue, Red, Red, White, Blue, White, Red, White, Blue,
        ];
        let arr = dutch_national_flag_sort(arr);
        assert!(is_sorted(&arr))
    }

    #[test]
    fn sorted_array() {
        let arr = vec![
            Red, Red, Red, Red, Red, White, White, White, White, White, Blue, Blue, Blue, Blue,
        ];
        let arr = dutch_national_flag_sort(arr);
        assert!(is_sorted(&arr))
    }
}
