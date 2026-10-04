use std::cmp::Ordering;

pub fn ternary_search_rec<T: Ord>(
    target: &T,
    list: &[T],
    start: usize,
    end: usize,
) -> Option<usize> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (ternary_search_rec)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement ternary_search_rec");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_none_if_empty_list() {
        let index = ternary_search_rec(&"a", &[], 1, 10);
        assert_eq!(index, None);
    }

    #[test]
    fn returns_none_if_range_is_invalid() {
        let index = ternary_search_rec(&1, &[1, 2, 3], 2, 1);
        assert_eq!(index, None);
    }

    #[test]
    fn returns_index_if_list_has_one_item() {
        let index = ternary_search_rec(&1, &[1], 0, 1);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn returns_first_index() {
        let index = ternary_search_rec(&1, &[1, 2, 3], 0, 2);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn returns_first_index_if_end_out_of_bounds() {
        let index = ternary_search_rec(&1, &[1, 2, 3], 0, 3);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn returns_last_index() {
        let index = ternary_search_rec(&3, &[1, 2, 3], 0, 2);
        assert_eq!(index, Some(2));
    }

    #[test]
    fn returns_last_index_if_end_out_of_bounds() {
        let index = ternary_search_rec(&3, &[1, 2, 3], 0, 3);
        assert_eq!(index, Some(2));
    }

    #[test]
    fn returns_middle_index() {
        let index = ternary_search_rec(&2, &[1, 2, 3], 0, 2);
        assert_eq!(index, Some(1));
    }

    #[test]
    fn returns_middle_index_if_end_out_of_bounds() {
        let index = ternary_search_rec(&2, &[1, 2, 3], 0, 3);
        assert_eq!(index, Some(1));
    }
}
