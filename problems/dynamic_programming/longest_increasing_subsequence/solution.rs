/// Finds the longest increasing subsequence and returns it.
///
/// If multiple subsequences with the longest possible subsequence length can be found, the
/// subsequence which appeared first will be returned (see `test_example_1`).
///
/// Inspired by [this LeetCode problem](https://leetcode.com/problems/longest-increasing-subsequence/).
pub fn longest_increasing_subsequence<T: Ord + Clone>(input_array: &[T]) -> Vec<T> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (longest_increasing_subsequence)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement longest_increasing_subsequence");
}


#[cfg(test)]
mod tests {
    use super::longest_increasing_subsequence;

    #[test]
    /// Need to specify generic type T in order to function
    fn test_empty_vec() {
        assert_eq!(longest_increasing_subsequence::<i32>(&[]), vec![]);
    }

    #[test]
    fn test_example_1() {
        assert_eq!(
            longest_increasing_subsequence(&[10, 9, 2, 5, 3, 7, 101, 18]),
            vec![2, 3, 7, 18]
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            longest_increasing_subsequence(&[0, 1, 0, 3, 2, 3]),
            vec![0, 1, 2, 3]
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            longest_increasing_subsequence(&[7, 7, 7, 7, 7, 7, 7]),
            vec![7]
        );
    }

    #[test]
    fn test_tle() {
        let mut input_array = vec![0i64; 1e5 as usize];
        let mut expected_result: Vec<i64> = Vec::with_capacity(5e4 as usize);
        for (idx, num) in input_array.iter_mut().enumerate() {
            match idx % 2 {
                0 => {
                    *num = idx as i64;
                    expected_result.push(*num);
                }
                1 => *num = -(idx as i64),
                _ => unreachable!(),
            }
        }
        expected_result[0] = -1;
        assert_eq!(
            longest_increasing_subsequence(&input_array),
            expected_result
        );
        // should be [-1, 2, 4, 6, 8, ...]
        // the first number is not 0, it would be replaced by -1 before 2 is added
    }

    #[test]
    fn test_negative_elements() {
        assert_eq!(longest_increasing_subsequence(&[-2, -1]), vec![-2, -1]);
    }
}
