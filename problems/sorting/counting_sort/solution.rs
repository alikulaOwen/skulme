/// In place counting sort for collections of u32
/// O(n + maxval) in time, where maxval is the biggest value an input can possibly take
/// O(maxval) in memory
/// u32 is chosen arbitrarly, a counting sort probably should'nt be used on data that requires bigger types.

pub fn counting_sort(arr: &mut [u32], maxval: usize) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (counting_sort)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement counting_sort");
}

use std::ops::AddAssign;
/// Generic implementation of a counting sort for all usigned types
pub fn generic_counting_sort<T: Into<u64> + From<u8> + AddAssign + Copy>(
    arr: &mut [T],
    maxval: usize,
) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (generic_counting_sort)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement generic_counting_sort");
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::sorting::have_same_elements;
    use crate::sorting::is_sorted;

    #[test]
    fn counting_sort_descending() {
        let mut ve1 = vec![6, 5, 4, 3, 2, 1];
        let cloned = ve1.clone();
        counting_sort(&mut ve1, 6);

        assert!(is_sorted(&ve1) && have_same_elements(&ve1, &cloned));
    }

    #[test]
    fn counting_sort_pre_sorted() {
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        let cloned = ve2.clone();
        counting_sort(&mut ve2, 6);

        assert!(is_sorted(&ve2) && have_same_elements(&ve2, &cloned));
    }

    #[test]
    fn generic_counting_sort() {
        let mut ve1: Vec<u8> = vec![100, 30, 60, 10, 20, 120, 1];
        let cloned = ve1.clone();
        super::generic_counting_sort(&mut ve1, 120);

        assert!(is_sorted(&ve1) && have_same_elements(&ve1, &cloned));
    }

    #[test]
    fn presorted_u64_counting_sort() {
        let mut ve2: Vec<u64> = vec![1, 2, 3, 4, 5, 6];
        let cloned = ve2.clone();
        super::generic_counting_sort(&mut ve2, 6);

        assert!(is_sorted(&ve2) && have_same_elements(&ve2, &cloned));
    }
}
