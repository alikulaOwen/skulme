// Task Assignment Problem using Bitmasking and DP in Rust
// Time Complexity: O(2^M * N) where M is number of people and N is number of tasks
// Space Complexity: O(2^M * N) for the DP table

use std::collections::HashMap;

/// Solves the task assignment problem where each person can do only certain tasks,
/// each person can do only one task, and each task is performed by only one person.
/// Uses bitmasking and dynamic programming to count total number of valid assignments.
///
/// # Arguments
/// * `task_performed` - A vector of vectors where each inner vector contains tasks
///                      that a person can perform (1-indexed task numbers)
/// * `total_tasks` - The total number of tasks (N)
///
/// # Returns
/// * The total number of valid task assignments
pub fn count_task_assignments(task_performed: Vec<Vec<usize>>, total_tasks: usize) -> i64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (count_task_assignments)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement count_task_assignments");
}


#[cfg(test)]
mod tests {
    use super::*;

    // Macro to generate multiple test cases for the task assignment function
    macro_rules! task_assignment_tests {
        ($($name:ident: $input:expr => $expected:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (task_performed, total_tasks) = $input;
                    assert_eq!(count_task_assignments(task_performed, total_tasks), $expected);
                }
            )*
        };
    }

    task_assignment_tests! {
        test_case_1: (vec![vec![1, 3, 4], vec![1, 2, 5], vec![3, 4]], 5) => 10,
        test_case_2: (vec![vec![1, 2], vec![1, 2]], 2) => 2,
        test_case_3: (vec![vec![1], vec![2], vec![3]], 3) => 1,
        test_case_4: (vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]], 3) => 6,
        test_case_5: (vec![vec![1], vec![1]], 1) => 0,

        // Edge test case
        test_case_single_person: (vec![vec![1, 2, 3]], 3) => 3,
    }
}
