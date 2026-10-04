use std::cmp::min;
use std::cmp::Ordering;

pub fn fibonacci_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (fibonacci_search)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement fibonacci_search");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let index = fibonacci_search(&"a", &[]);
        assert_eq!(index, None);
    }

    #[test]
    fn one_item() {
        let index = fibonacci_search(&"a", &["a"]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn search_strings() {
        let index = fibonacci_search(&"a", &["a", "b", "c", "d", "google", "zoo"]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn search_ints() {
        let index = fibonacci_search(&4, &[1, 2, 3, 4]);
        assert_eq!(index, Some(3));

        let index = fibonacci_search(&3, &[1, 2, 3, 4]);
        assert_eq!(index, Some(2));

        let index = fibonacci_search(&2, &[1, 2, 3, 4]);
        assert_eq!(index, Some(1));

        let index = fibonacci_search(&1, &[1, 2, 3, 4]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn not_found() {
        let index = fibonacci_search(&5, &[1, 2, 3, 4]);
        assert_eq!(index, None);
    }
}
