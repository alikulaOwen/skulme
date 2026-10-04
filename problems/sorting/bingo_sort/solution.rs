use std::cmp::{max, min};

// Function for finding the maximum and minimum element of the Array
fn max_min(vec: &[i32], bingo: &mut i32, next_bingo: &mut i32) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (max_min)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement max_min");
}

pub fn bingo_sort(vec: &mut [i32]) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (bingo_sort)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement bingo_sort");
}

#[allow(dead_code)]
fn print_array(arr: &[i32]) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (print_array)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement print_array");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bingo_sort() {
        let mut arr = vec![5, 4, 8, 5, 4, 8, 5, 4, 4, 4];
        bingo_sort(&mut arr);
        assert_eq!(arr, vec![4, 4, 4, 4, 4, 5, 5, 5, 8, 8]);

        let mut arr2 = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        bingo_sort(&mut arr2);
        assert_eq!(arr2, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

        let mut arr3 = vec![0, 1, 0, 1, 0, 1];
        bingo_sort(&mut arr3);
        assert_eq!(arr3, vec![0, 0, 0, 1, 1, 1]);
    }

    #[test]
    fn test_empty_array() {
        let mut arr = Vec::new();
        bingo_sort(&mut arr);
        assert_eq!(arr, Vec::new());
    }

    #[test]
    fn test_single_element_array() {
        let mut arr = vec![42];
        bingo_sort(&mut arr);
        assert_eq!(arr, vec![42]);
    }

    #[test]
    fn test_negative_numbers() {
        let mut arr = vec![-5, -4, -3, -2, -1];
        bingo_sort(&mut arr);
        assert_eq!(arr, vec![-5, -4, -3, -2, -1]);
    }

    #[test]
    fn test_already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        bingo_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut arr = vec![5, 4, 3, 2, 1];
        bingo_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_duplicates() {
        let mut arr = vec![1, 2, 3, 4, 5, 1, 2, 3, 4, 5];
        bingo_sort(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5]);
    }
}
