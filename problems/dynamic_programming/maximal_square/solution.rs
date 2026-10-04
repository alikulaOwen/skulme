use std::cmp::max;
use std::cmp::min;

/// Maximal Square
///
/// Given an `m` * `n` binary matrix filled with 0's and 1's, find the largest square containing only 1's and return its area.\
/// <https://leetcode.com/problems/maximal-square/>
///
/// # Arguments:
///   * `matrix` - an array of integer array
///
/// # Complexity
///   - time complexity: O(n^2),
///   - space complexity: O(n),
pub fn maximal_square(matrix: &mut [Vec<i32>]) -> i32 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (maximal_square)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement maximal_square");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(maximal_square(&mut []), 0);

        let mut matrix = vec![vec![0, 1], vec![1, 0]];
        assert_eq!(maximal_square(&mut matrix), 1);

        let mut matrix = vec![
            vec![1, 0, 1, 0, 0],
            vec![1, 0, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
            vec![1, 0, 0, 1, 0],
        ];
        assert_eq!(maximal_square(&mut matrix), 4);

        let mut matrix = vec![vec![0]];
        assert_eq!(maximal_square(&mut matrix), 0);
    }
}
