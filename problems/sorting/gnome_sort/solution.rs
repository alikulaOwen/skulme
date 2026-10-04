use std::cmp;

pub fn gnome_sort<T>(arr: &[T]) -> Vec<T>
where
    T: cmp::PartialEq + cmp::PartialOrd + Clone,
{
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (gnome_sort)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement gnome_sort");
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::sorting::have_same_elements;
    use crate::sorting::is_sorted;

    #[test]
    fn basic() {
        let original = [6, 5, -8, 3, 2, 3];
        let res = gnome_sort(&original);
        assert!(is_sorted(&res) && have_same_elements(&res, &original));
    }

    #[test]
    fn already_sorted() {
        let original = gnome_sort(&["a", "b", "c"]);
        let res = gnome_sort(&original);
        assert!(is_sorted(&res) && have_same_elements(&res, &original));
    }

    #[test]
    fn odd_number_of_elements() {
        let original = gnome_sort(&["d", "a", "c", "e", "b"]);
        let res = gnome_sort(&original);
        assert!(is_sorted(&res) && have_same_elements(&res, &original));
    }

    #[test]
    fn one_element() {
        let original = gnome_sort(&[3]);
        let res = gnome_sort(&original);
        assert!(is_sorted(&res) && have_same_elements(&res, &original));
    }

    #[test]
    fn empty() {
        let original = gnome_sort(&Vec::<u8>::new());
        let res = gnome_sort(&original);
        assert!(is_sorted(&res) && have_same_elements(&res, &original));
    }
}
