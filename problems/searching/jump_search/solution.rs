use std::cmp::min;

pub fn jump_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (jump_search)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement jump_search");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert!(jump_search(&"a", &[]).is_none());
    }

    #[test]
    fn one_item() {
        assert_eq!(jump_search(&"a", &["a"]).unwrap(), 0);
    }

    #[test]
    fn search_strings() {
        assert_eq!(
            jump_search(&"a", &["a", "b", "c", "d", "google", "zoo"]).unwrap(),
            0
        );
    }

    #[test]
    fn search_ints() {
        let arr = [1, 2, 3, 4];
        assert_eq!(jump_search(&4, &arr).unwrap(), 3);
        assert_eq!(jump_search(&3, &arr).unwrap(), 2);
        assert_eq!(jump_search(&2, &arr).unwrap(), 1);
        assert_eq!(jump_search(&1, &arr).unwrap(), 0);
    }

    #[test]
    fn not_found() {
        let arr = [1, 2, 3, 4];

        assert!(jump_search(&5, &arr).is_none());
        assert!(jump_search(&0, &arr).is_none());
    }
}
