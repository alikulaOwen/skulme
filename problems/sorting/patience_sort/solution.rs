use std::vec;

pub fn patience_sort<T: Ord + Copy>(arr: &mut [T]) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (patience_sort)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement patience_sort");
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::sorting::have_same_elements;
    use crate::sorting::is_sorted;

    #[test]
    fn basic() {
        let mut array = vec![
            -2, 7, 15, -14, 0, 15, 0, 10_033, 7, -7, -4, -13, 5, 8, -14, 12,
        ];
        let cloned = array.clone();
        patience_sort(&mut array);
        assert!(is_sorted(&array) && have_same_elements(&array, &cloned));
    }

    #[test]
    fn empty() {
        let mut array = Vec::<i32>::new();
        let cloned = array.clone();
        patience_sort(&mut array);
        assert!(is_sorted(&array) && have_same_elements(&array, &cloned));
    }

    #[test]
    fn one_element() {
        let mut array = vec![3];
        let cloned = array.clone();
        patience_sort(&mut array);
        assert!(is_sorted(&array) && have_same_elements(&array, &cloned));
    }

    #[test]
    fn pre_sorted() {
        let mut array = vec![-123_456, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let cloned = array.clone();
        patience_sort(&mut array);
        assert!(is_sorted(&array) && have_same_elements(&array, &cloned));
    }
}
