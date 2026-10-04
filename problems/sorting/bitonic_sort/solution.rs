fn _comp_and_swap<T: Ord>(array: &mut [T], left: usize, right: usize, ascending: bool) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (_comp_and_swap)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement _comp_and_swap");
}

fn _bitonic_merge<T: Ord>(array: &mut [T], low: usize, length: usize, ascending: bool) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (_bitonic_merge)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement _bitonic_merge");
}

pub fn bitonic_sort<T: Ord>(array: &mut [T], low: usize, length: usize, ascending: bool) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (bitonic_sort)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement bitonic_sort");
}

//Note that this program works only when size of input is a power of 2.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sorting::have_same_elements;
    use crate::sorting::is_descending_sorted;
    use crate::sorting::is_sorted;

    #[test]
    fn descending() {
        //descending
        let mut ve1 = vec![6, 5, 4, 3];
        let cloned = ve1.clone();
        bitonic_sort(&mut ve1, 0, 4, true);
        assert!(is_sorted(&ve1) && have_same_elements(&ve1, &cloned));
    }

    #[test]
    fn ascending() {
        //pre-sorted
        let mut ve2 = vec![1, 2, 3, 4];
        let cloned = ve2.clone();
        bitonic_sort(&mut ve2, 0, 4, false);
        assert!(is_descending_sorted(&ve2) && have_same_elements(&ve2, &cloned));
    }
}
