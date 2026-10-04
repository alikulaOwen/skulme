use crate::sorting::partition;
use std::cmp::{Ordering, PartialOrd};

/// Returns k-th smallest element of an array, i.e. its order statistics.
/// Time complexity is O(n^2) in the worst case, but only O(n) on average.
/// It mutates the input, and therefore does not require additional space.
pub fn kth_smallest<T>(input: &mut [T], k: usize) -> Option<T>
where
    T: PartialOrd + Copy,
{
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (kth_smallest)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement kth_smallest");
}

fn _kth_smallest<T>(input: &mut [T], k: usize, lo: usize, hi: usize) -> T
where
    T: PartialOrd + Copy,
{
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (_kth_smallest)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement _kth_smallest");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut zero: [u8; 0] = [];
        let first = kth_smallest(&mut zero, 1);

        assert_eq!(None, first);
    }

    #[test]
    fn one_element() {
        let mut one = [1];
        let first = kth_smallest(&mut one, 1);

        assert_eq!(1, first.unwrap());
    }

    #[test]
    fn many_elements() {
        // 0 1 3 4 5 7 8 9 9 10 12 13 16 17
        let mut many = [9, 17, 3, 16, 13, 10, 1, 5, 7, 12, 4, 8, 9, 0];

        let first = kth_smallest(&mut many, 1);
        let third = kth_smallest(&mut many, 3);
        let sixth = kth_smallest(&mut many, 6);
        let fourteenth = kth_smallest(&mut many, 14);

        assert_eq!(0, first.unwrap());
        assert_eq!(3, third.unwrap());
        assert_eq!(7, sixth.unwrap());
        assert_eq!(17, fourteenth.unwrap());
    }
}
