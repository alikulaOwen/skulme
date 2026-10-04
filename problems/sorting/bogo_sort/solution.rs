use crate::math::PCG32;
use std::time::{SystemTime, UNIX_EPOCH};

const DEFAULT: u64 = 4294967296;

fn is_sorted<T: Ord>(arr: &[T], len: usize) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (is_sorted)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement is_sorted");
}

#[cfg(target_pointer_width = "64")]
fn generate_index(range: usize, generator: &mut PCG32) -> usize {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (generate_index)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement generate_index");
}

#[cfg(not(target_pointer_width = "64"))]
fn generate_index(range: usize, generator: &mut PCG32) -> usize {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (generate_index)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement generate_index");
}

/**
 * Fisher–Yates shuffle for generating random permutation.
 */
fn permute_randomly<T>(arr: &mut [T], len: usize, generator: &mut PCG32) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (permute_randomly)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement permute_randomly");
}

pub fn bogo_sort<T: Ord>(arr: &mut [T]) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (bogo_sort)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement bogo_sort");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_array() {
        let mut arr = [1, 8, 3, 2, 7, 4, 6, 5];
        bogo_sort(&mut arr);

        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }

    #[test]
    fn sorted_array() {
        let mut arr = [1, 2, 3, 4, 5, 6, 7, 8];
        bogo_sort(&mut arr);

        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }
}
