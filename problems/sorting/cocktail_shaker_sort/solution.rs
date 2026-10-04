pub fn cocktail_shaker_sort<T: Ord>(arr: &mut [T]) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (cocktail_shaker_sort)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement cocktail_shaker_sort");
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::sorting::have_same_elements;
    use crate::sorting::is_sorted;

    #[test]
    fn basic() {
        let mut arr = vec![5, 2, 1, 3, 4, 6];
        let cloned = arr.clone();
        cocktail_shaker_sort(&mut arr);
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }

    #[test]
    fn empty() {
        let mut arr = Vec::<i32>::new();
        let cloned = arr.clone();
        cocktail_shaker_sort(&mut arr);
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }

    #[test]
    fn one_element() {
        let mut arr = vec![1];
        let cloned = arr.clone();
        cocktail_shaker_sort(&mut arr);
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }

    #[test]
    fn pre_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5, 6];
        let cloned = arr.clone();
        cocktail_shaker_sort(&mut arr);
        assert!(is_sorted(&arr) && have_same_elements(&arr, &cloned));
    }
}
