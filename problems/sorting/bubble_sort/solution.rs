//! =============================================================================
//! The Warehouse Pod Organizer: Bubble Sort
//! CATEGORY: sorting
//! =============================================================================
//!
//! -----------------------------------------------------------------------------
//! 1. REAL-WORLD STORY & CONTEXT (WHAT IS THIS?)
//! -----------------------------------------------------------------------------
//! Imagine rearranging sorting bins on a conveyor belt:
//! - Compare adjacent items and swap if out of order.
//! - Largest items 'bubble' up to the end with each pass.
//!
//! -----------------------------------------------------------------------------
//! 2. GUIDED HINTING QUESTIONS AS YOUR PROBLEM SPECIFICATION
//! -----------------------------------------------------------------------------
//! ❓ Q1: When is an array sorted? (When a full pass makes 0 swaps)
//! ❓ Q2: How does optimization work? (Track swapped flag)
//! -----------------------------------------------------------------------------

pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (bubble_sort)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement bubble_sort");
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::sorting::have_same_elements;
    use crate::sorting::is_sorted;

    #[test]
    fn descending() {
        //descending
        let mut ve1 = vec![6, 5, 4, 3, 2, 1];
        let cloned = ve1.clone();
        bubble_sort(&mut ve1);
        assert!(is_sorted(&ve1) && have_same_elements(&ve1, &cloned));
    }

    #[test]
    fn ascending() {
        //pre-sorted
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        let cloned = ve2.clone();
        bubble_sort(&mut ve2);
        assert!(is_sorted(&ve2) && have_same_elements(&ve2, &cloned));
    }
    #[test]
    fn empty() {
        let mut ve3: Vec<usize> = vec![];
        let cloned = ve3.clone();
        bubble_sort(&mut ve3);
        assert!(is_sorted(&ve3) && have_same_elements(&ve3, &cloned));
    }
}
