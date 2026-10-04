fn _stooge_sort<T: Ord>(arr: &mut [T], start: usize, end: usize) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (_stooge_sort)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement _stooge_sort");
}

pub fn stooge_sort<T: Ord>(arr: &mut [T]) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (stooge_sort)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement stooge_sort");
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::sorting::have_same_elements;
    use crate::sorting::is_sorted;

    #[test]
    fn basic() {
        let mut vec = vec![3, 5, 6, 3, 1, 4];
        let cloned = vec.clone();
        stooge_sort(&mut vec);
        assert!(is_sorted(&vec) && have_same_elements(&vec, &cloned));
    }

    #[test]
    fn empty() {
        let mut vec: Vec<i32> = vec![];
        let cloned = vec.clone();
        stooge_sort(&mut vec);
        assert!(is_sorted(&vec) && have_same_elements(&vec, &cloned));
    }

    #[test]
    fn reverse() {
        let mut vec = vec![6, 5, 4, 3, 2, 1];
        let cloned = vec.clone();
        stooge_sort(&mut vec);
        assert!(is_sorted(&vec) && have_same_elements(&vec, &cloned));
    }

    #[test]
    fn already_sorted() {
        let mut vec = vec![1, 2, 3, 4, 5, 6];
        let cloned = vec.clone();
        stooge_sort(&mut vec);
        assert!(is_sorted(&vec) && have_same_elements(&vec, &cloned));
    }
}
