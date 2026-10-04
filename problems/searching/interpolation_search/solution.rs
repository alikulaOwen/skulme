pub fn interpolation_search<Ordering>(nums: &[i32], item: &i32) -> Result<usize, usize> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (interpolation_search)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement interpolation_search");
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    #[test]
    fn returns_err_if_empty_slice() {
        let nums = [];
        assert_eq!(interpolation_search::<Ordering>(&nums, &3), Err(0));
    }

    #[test]
    fn returns_err_if_target_not_found() {
        let nums = [1, 2, 3, 4, 5, 6];
        assert_eq!(interpolation_search::<Ordering>(&nums, &10), Err(0));
    }

    #[test]
    fn returns_first_index() {
        let index: Result<usize, usize> = interpolation_search::<Ordering>(&[1, 2, 3, 4, 5], &1);
        assert_eq!(index, Ok(0));
    }

    #[test]
    fn returns_last_index() {
        let index: Result<usize, usize> = interpolation_search::<Ordering>(&[1, 2, 3, 4, 5], &5);
        assert_eq!(index, Ok(4));
    }

    #[test]
    fn returns_middle_index() {
        let index: Result<usize, usize> = interpolation_search::<Ordering>(&[1, 2, 3, 4, 5], &3);
        assert_eq!(index, Ok(2));
    }
}
