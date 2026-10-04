use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Range};

pub struct LazySegmentTree<T: Debug + Default + Ord + Copy + Display + AddAssign + Add<Output = T>>
{
    len: usize,
    tree: Vec<T>,
    lazy: Vec<Option<T>>,
    merge: fn(T, T) -> T,
}

impl<T: Debug + Default + Ord + Copy + Display + AddAssign + Add<Output = T>> LazySegmentTree<T> {
    pub fn from_vec(arr: &[T], merge: fn(T, T) -> T) -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (from_vec)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement from_vec");
}

    fn build_recursive(
        &mut self,
        arr: &[T],
        idx: usize,
        range: Range<usize>,
        merge: fn(T, T) -> T,
    ) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (build_recursive)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement build_recursive");
}

    pub fn query(&mut self, range: Range<usize>) -> Option<T> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (query)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement query");
}

    fn query_recursive(
        &mut self,
        idx: usize,
        element_range: Range<usize>,
        query_range: &Range<usize>,
    ) -> Option<T> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (query_recursive)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement query_recursive");
}

    pub fn update(&mut self, target_range: Range<usize>, val: T) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (update)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement update");
}

    fn update_recursive(
        &mut self,
        idx: usize,
        element_range: Range<usize>,
        target_range: &Range<usize>,
        val: T,
    ) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (update_recursive)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement update_recursive");
}

    fn propagation(&mut self, idx: usize, element_range: &Range<usize>, parent_lazy: T) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (propagation)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement propagation");
}
}


#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::TestResult;
    use quickcheck_macros::quickcheck;
    use std::cmp::{max, min};

    #[test]
    fn test_min_segments() {
        let vec = vec![-30, 2, -4, 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8];
        let mut min_seg_tree = LazySegmentTree::from_vec(&vec, min);
        // [-30, 2, -4, 7, (3, -5, 6), 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(-5), min_seg_tree.query(4..7));
        // [(-30, 2, -4, 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8)]
        assert_eq!(Some(-30), min_seg_tree.query(0..vec.len()));
        // [(-30, 2), -4, 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(-30), min_seg_tree.query(0..2));
        // [-30, (2, -4), 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(-4), min_seg_tree.query(1..3));
        // [-30, (2, -4, 7, 3, -5, 6), 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(-5), min_seg_tree.query(1..7));
    }

    #[test]
    fn test_max_segments() {
        let vec = vec![-30, 2, -4, 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8];
        let mut max_seg_tree = LazySegmentTree::from_vec(&vec, max);
        // [-30, 2, -4, 7, (3, -5, 6), 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(6), max_seg_tree.query(4..7));
        // [(-30, 2, -4, 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8)]
        assert_eq!(Some(15), max_seg_tree.query(0..vec.len()));
        // [(-30, 2), -4, 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(2), max_seg_tree.query(0..2));
        // [-30, (2, -4), 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(2), max_seg_tree.query(1..3));
        // [-30, (2, -4, 7, 3, -5, 6), 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(7), max_seg_tree.query(1..7));
    }

    #[test]
    fn test_sum_segments() {
        let vec = vec![-30, 2, -4, 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8];
        let mut max_seg_tree = LazySegmentTree::from_vec(&vec, |x, y| x + y);
        // [-30, 2, -4, 7, (3, -5, 6), 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(4), max_seg_tree.query(4..7));
        // [(-30, 2, -4, 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8)]
        assert_eq!(Some(7), max_seg_tree.query(0..vec.len()));
        // [(-30, 2), -4, 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(-28), max_seg_tree.query(0..2));
        // [-30, (2, -4), 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(-2), max_seg_tree.query(1..3));
        // [-30, (2, -4, 7, 3, -5, 6), 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(9), max_seg_tree.query(1..7));
    }

    #[test]
    fn test_update_segments_tiny() {
        let vec = vec![0, 0, 0, 0, 0];
        let mut update_seg_tree = LazySegmentTree::from_vec(&vec, |x, y| x + y);
        update_seg_tree.update(0..3, 3);
        update_seg_tree.update(2..5, 3);
        assert_eq!(Some(3), update_seg_tree.query(0..1));
        assert_eq!(Some(3), update_seg_tree.query(1..2));
        assert_eq!(Some(6), update_seg_tree.query(2..3));
        assert_eq!(Some(3), update_seg_tree.query(3..4));
        assert_eq!(Some(3), update_seg_tree.query(4..5));
    }

    #[test]
    fn test_update_segments() {
        let vec = vec![-30, 2, -4, 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8];
        let mut update_seg_tree = LazySegmentTree::from_vec(&vec, |x, y| x + y);
        // -> [-30, (5, -1, 10, 6), -5, 6, 11, -20, 9, 14, 15, 5, 2, -8]
        update_seg_tree.update(1..5, 3);

        // [-30, 5, -1, 10, (6 -5, 6), 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(7), update_seg_tree.query(4..7));
        // [(-30, 5, -1, 10, 6 , -5, 6, 11, -20, 9, 14, 15, 5, 2, -8)]
        assert_eq!(Some(19), update_seg_tree.query(0..vec.len()));
        // [(-30, 5), -1, 10, 6, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(-25), update_seg_tree.query(0..2));
        // [-30, (5, -1), 10, 6, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(4), update_seg_tree.query(1..3));
        // [-30, (5, -1, 10, 6, -5, 6), 11, -20, 9, 14, 15, 5, 2, -8]
        assert_eq!(Some(21), update_seg_tree.query(1..7));
    }

    // Some properties over segment trees:
    //  When asking for the range of the overall array, return the same as iter().min() or iter().max(), etc.
    //  When asking for an interval containing a single value, return this value, no matter the merge function

    #[quickcheck]
    fn check_overall_interval_min(array: Vec<i32>) -> TestResult {
        let mut seg_tree = LazySegmentTree::from_vec(&array, min);
        TestResult::from_bool(array.iter().min().copied() == seg_tree.query(0..array.len()))
    }

    #[quickcheck]
    fn check_overall_interval_max(array: Vec<i32>) -> TestResult {
        let mut seg_tree = LazySegmentTree::from_vec(&array, max);
        TestResult::from_bool(array.iter().max().copied() == seg_tree.query(0..array.len()))
    }

    #[quickcheck]
    fn check_overall_interval_sum(array: Vec<i32>) -> TestResult {
        let mut seg_tree = LazySegmentTree::from_vec(&array, max);
        TestResult::from_bool(array.iter().max().copied() == seg_tree.query(0..array.len()))
    }

    #[quickcheck]
    fn check_single_interval_min(array: Vec<i32>) -> TestResult {
        let mut seg_tree = LazySegmentTree::from_vec(&array, min);
        for (i, value) in array.into_iter().enumerate() {
            let res = seg_tree.query(Range {
                start: i,
                end: i + 1,
            });
            if res != Some(value) {
                return TestResult::error(format!("Expected {:?}, got {:?}", Some(value), res));
            }
        }
        TestResult::passed()
    }

    #[quickcheck]
    fn check_single_interval_max(array: Vec<i32>) -> TestResult {
        let mut seg_tree = LazySegmentTree::from_vec(&array, max);
        for (i, value) in array.into_iter().enumerate() {
            let res = seg_tree.query(Range {
                start: i,
                end: i + 1,
            });
            if res != Some(value) {
                return TestResult::error(format!("Expected {:?}, got {:?}", Some(value), res));
            }
        }
        TestResult::passed()
    }

    #[quickcheck]
    fn check_single_interval_sum(array: Vec<i32>) -> TestResult {
        let mut seg_tree = LazySegmentTree::from_vec(&array, max);
        for (i, value) in array.into_iter().enumerate() {
            let res = seg_tree.query(Range {
                start: i,
                end: i + 1,
            });
            if res != Some(value) {
                return TestResult::error(format!("Expected {:?}, got {:?}", Some(value), res));
            }
        }
        TestResult::passed()
    }
}
