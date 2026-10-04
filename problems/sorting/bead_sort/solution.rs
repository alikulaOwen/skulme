//Bead sort only works for sequences of non-negative integers.
//https://en.wikipedia.org/wiki/Bead_sort
pub fn bead_sort(a: &mut [usize]) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (bead_sort)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement bead_sort");
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::sorting::have_same_elements;
    use crate::sorting::is_sorted;

    #[test]
    fn descending() {
        //descending
        let mut ve1: [usize; 5] = [5, 4, 3, 2, 1];
        let cloned = ve1;
        bead_sort(&mut ve1);
        assert!(is_sorted(&ve1) && have_same_elements(&ve1, &cloned));
    }

    #[test]
    fn mix_values() {
        //pre-sorted
        let mut ve2: [usize; 5] = [7, 9, 6, 2, 3];
        let cloned = ve2;
        bead_sort(&mut ve2);
        assert!(is_sorted(&ve2) && have_same_elements(&ve2, &cloned));
    }
}
