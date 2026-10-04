use std::collections::BTreeSet;

// Find minimum excluded number from a set of given numbers using a set
/// Finds the MEX of the values provided in `arr`
/// Uses [`BTreeSet`](std::collections::BTreeSet)
/// O(nlog(n)) implementation
pub fn mex_using_set(arr: &[i64]) -> i64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (mex_using_set)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement mex_using_set");
}
/// Finds the MEX of the values provided in `arr`
/// Uses sorting
/// O(nlog(n)) implementation
pub fn mex_using_sort(arr: &[i64]) -> i64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (mex_using_sort)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement mex_using_sort");
}


#[cfg(test)]
mod tests {
    use super::*;
    struct MexTests {
        test_arrays: Vec<Vec<i64>>,
        outputs: Vec<i64>,
    }
    impl MexTests {
        fn new() -> Self {
            Self {
                test_arrays: vec![
                    vec![-1, 0, 1, 2, 3],
                    vec![-100, 0, 1, 2, 3, 5],
                    vec![-1000000, 0, 1, 2, 5],
                    vec![2, 0, 1, 2, 4],
                    vec![1, 2, 3, 0, 4],
                    vec![0, 1, 5, 2, 4, 3],
                    vec![0, 1, 2, 3, 4, 5, 6],
                    vec![0, 1, 2, 3, 4, 5, 6, 7],
                    vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
                ],
                outputs: vec![4, 4, 3, 3, 5, 6, 7, 8, 9],
            }
        }
        fn test_function(&self, f: fn(&[i64]) -> i64) {
            for (nums, output) in self.test_arrays.iter().zip(self.outputs.iter()) {
                assert_eq!(f(nums), *output);
            }
        }
    }
    #[test]
    fn test_mex_using_set() {
        let tests = MexTests::new();
        mex_using_set(&[1, 23, 3]);
        tests.test_function(mex_using_set);
    }
    #[test]
    fn test_mex_using_sort() {
        let tests = MexTests::new();
        tests.test_function(mex_using_sort);
    }
}
