fn merge<T: Ord + Copy>(arr: &mut [T], mid: usize) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (merge)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement merge");
}

pub fn top_down_merge_sort<T: Ord + Copy>(arr: &mut [T]) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (top_down_merge_sort)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement top_down_merge_sort");
}

pub fn bottom_up_merge_sort<T: Copy + Ord>(a: &mut [T]) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (bottom_up_merge_sort)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement bottom_up_merge_sort");
}


#[cfg(test)]
mod tests {
    #[cfg(test)]
    mod top_down_merge_sort {
        use super::super::*;
        use crate::sorting::have_same_elements;
        use crate::sorting::is_sorted;

        #[test]
        fn basic() {
            let mut res = vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
            let cloned = res.clone();
            top_down_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }

        #[test]
        fn basic_string() {
            let mut res = vec!["a", "bb", "d", "cc"];
            let cloned = res.clone();
            top_down_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }

        #[test]
        fn empty() {
            let mut res = Vec::<u8>::new();
            let cloned = res.clone();
            top_down_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }

        #[test]
        fn one_element() {
            let mut res = vec![1];
            let cloned = res.clone();
            top_down_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }

        #[test]
        fn pre_sorted() {
            let mut res = vec![1, 2, 3, 4];
            let cloned = res.clone();
            top_down_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }

        #[test]
        fn reverse_sorted() {
            let mut res = vec![4, 3, 2, 1];
            let cloned = res.clone();
            top_down_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }
    }

    #[cfg(test)]
    mod bottom_up_merge_sort {
        use super::super::*;
        use crate::sorting::have_same_elements;
        use crate::sorting::is_sorted;

        #[test]
        fn basic() {
            let mut res = vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
            let cloned = res.clone();
            bottom_up_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }

        #[test]
        fn basic_string() {
            let mut res = vec!["a", "bb", "d", "cc"];
            let cloned = res.clone();
            bottom_up_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }

        #[test]
        fn empty() {
            let mut res = Vec::<u8>::new();
            let cloned = res.clone();
            bottom_up_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }

        #[test]
        fn one_element() {
            let mut res = vec![1];
            let cloned = res.clone();
            bottom_up_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }

        #[test]
        fn pre_sorted() {
            let mut res = vec![1, 2, 3, 4];
            let cloned = res.clone();
            bottom_up_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }

        #[test]
        fn reverse_sorted() {
            let mut res = vec![4, 3, 2, 1];
            let cloned = res.clone();
            bottom_up_merge_sort(&mut res);
            assert!(is_sorted(&res) && have_same_elements(&res, &cloned));
        }
    }
}
