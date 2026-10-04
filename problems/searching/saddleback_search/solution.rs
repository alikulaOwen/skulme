// Saddleback search is a technique used to find an element in a sorted 2D matrix in O(m + n) time,
// where m is the number of rows, and n is the number of columns. It works by starting from the
// top-right corner of the matrix and moving left or down based on the comparison of the current
// element with the target element.
use std::cmp::Ordering;

pub fn saddleback_search(matrix: &[Vec<i32>], element: i32) -> (usize, usize) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (saddleback_search)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement saddleback_search");
}


#[cfg(test)]
mod tests {
    use super::*;

    // Test when the element is not present in the matrix
    #[test]
    fn test_element_not_found() {
        let matrix = vec![vec![1, 10, 100], vec![2, 20, 200], vec![3, 30, 300]];
        assert_eq!(saddleback_search(&matrix, 123), (0, 0));
    }

    // Test when the element is at the top-left corner of the matrix
    #[test]
    fn test_element_at_top_left() {
        let matrix = vec![vec![1, 10, 100], vec![2, 20, 200], vec![3, 30, 300]];
        assert_eq!(saddleback_search(&matrix, 1), (1, 1));
    }

    // Test when the element is at the bottom-right corner of the matrix
    #[test]
    fn test_element_at_bottom_right() {
        let matrix = vec![vec![1, 10, 100], vec![2, 20, 200], vec![3, 30, 300]];
        assert_eq!(saddleback_search(&matrix, 300), (3, 3));
    }

    // Test when the element is at the top-right corner of the matrix
    #[test]
    fn test_element_at_top_right() {
        let matrix = vec![vec![1, 10, 100], vec![2, 20, 200], vec![3, 30, 300]];
        assert_eq!(saddleback_search(&matrix, 100), (1, 3));
    }

    // Test when the element is at the bottom-left corner of the matrix
    #[test]
    fn test_element_at_bottom_left() {
        let matrix = vec![vec![1, 10, 100], vec![2, 20, 200], vec![3, 30, 300]];
        assert_eq!(saddleback_search(&matrix, 3), (3, 1));
    }

    // Additional test case: Element in the middle of the matrix
    #[test]
    fn test_element_in_middle() {
        let matrix = vec![
            vec![1, 10, 100, 1000],
            vec![2, 20, 200, 2000],
            vec![3, 30, 300, 3000],
        ];
        assert_eq!(saddleback_search(&matrix, 200), (2, 3));
    }
}
