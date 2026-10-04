// list all subset combinations of n element in given set of r element.
// This is a recursive function that collects all subsets of the set of size n
// with the given set of size r.
pub fn list_subset(
    set: &[i32],
    n: usize,
    r: usize,
    index: usize,
    data: &mut [i32],
    i: usize,
) -> Vec<Vec<i32>> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (list_subset)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement list_subset");
}

// Test module

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_subset3() {
        let set = [1, 2, 3, 4, 5];
        let n = set.len();
        const R: usize = 3;
        let mut data = [0; R];

        let res = list_subset(&set, n, R, 0, &mut data, 0);

        assert_eq!(
            res,
            vec![
                vec![1, 2, 3],
                vec![1, 2, 4],
                vec![1, 2, 5],
                vec![1, 3, 4],
                vec![1, 3, 5],
                vec![1, 4, 5],
                vec![2, 3, 4],
                vec![2, 3, 5],
                vec![2, 4, 5],
                vec![3, 4, 5]
            ]
        );
    }

    #[test]
    fn test_print_subset4() {
        let set = [1, 2, 3, 4, 5];
        let n = set.len();
        const R: usize = 4;
        let mut data = [0; R];

        let res = list_subset(&set, n, R, 0, &mut data, 0);

        assert_eq!(
            res,
            vec![
                vec![1, 2, 3, 4],
                vec![1, 2, 3, 5],
                vec![1, 2, 4, 5],
                vec![1, 3, 4, 5],
                vec![2, 3, 4, 5]
            ]
        );
    }

    #[test]
    fn test_print_subset5() {
        let set = [1, 2, 3, 4, 5];
        let n = set.len();
        const R: usize = 5;
        let mut data = [0; R];

        let res = list_subset(&set, n, R, 0, &mut data, 0);

        assert_eq!(res, vec![vec![1, 2, 3, 4, 5]]);
    }

    #[test]
    fn test_print_incorrect_subset() {
        let set = [1, 2, 3, 4, 5];
        let n = set.len();
        const R: usize = 6;
        let mut data = [0; R];

        let res = list_subset(&set, n, R, 0, &mut data, 0);

        let result_set: Vec<Vec<i32>> = Vec::new();
        assert_eq!(res, result_set);
    }
}
