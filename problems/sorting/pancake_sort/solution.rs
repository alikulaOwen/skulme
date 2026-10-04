use std::cmp;

pub fn pancake_sort<T>(arr: &mut [T]) -> Vec<T>
where
    T: cmp::PartialEq + cmp::Ord + cmp::PartialOrd + Clone,
{
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (pancake_sort)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement pancake_sort");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let res = pancake_sort(&mut [6, 5, -8, 3, 2, 3]);
        assert_eq!(res, vec![-8, 2, 3, 3, 5, 6]);
    }

    #[test]
    fn already_sorted() {
        let res = pancake_sort(&mut ["a", "b", "c"]);
        assert_eq!(res, vec!["a", "b", "c"]);
    }

    #[test]
    fn odd_number_of_elements() {
        let res = pancake_sort(&mut ["d", "a", "c", "e", "b"]);
        assert_eq!(res, vec!["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn one_element() {
        let res = pancake_sort(&mut [3]);
        assert_eq!(res, vec![3]);
    }

    #[test]
    fn empty() {
        let res = pancake_sort(&mut [] as &mut [u8]);
        assert_eq!(res, vec![]);
    }
}
