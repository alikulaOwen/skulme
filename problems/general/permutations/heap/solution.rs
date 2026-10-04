use std::fmt::Debug;

/// Computes all permutations of an array using Heap's algorithm
/// Read `recurse_naive` first, since we're building on top of the same intuition
pub fn heap_permute<T: Clone + Debug>(arr: &[T]) -> Vec<Vec<T>> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (heap_permute)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement heap_permute");
}

fn heap_recurse<T: Clone + Debug>(arr: &mut [T], k: usize, collector: &mut Vec<Vec<T>>) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (heap_recurse)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement heap_recurse");
}


#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use crate::general::permutations::heap_permute;
    use crate::general::permutations::tests::{
        assert_permutations, assert_valid_permutation, NotTooBigVec,
    };

    #[test]
    fn test_3_different_values() {
        let original = vec![1, 2, 3];
        let res = heap_permute(&original);
        assert_eq!(res.len(), 6); // 3!
        for permut in res {
            assert_valid_permutation(&original, &permut)
        }
    }

    #[test]
    fn test_3_times_the_same_value() {
        let original = vec![1, 1, 1];
        let res = heap_permute(&original);
        assert_eq!(res.len(), 6); // 3!
        for permut in res {
            assert_valid_permutation(&original, &permut)
        }
    }

    #[quickcheck]
    fn test_some_elements(NotTooBigVec { inner: original }: NotTooBigVec) {
        let permutations = heap_permute(&original);
        assert_permutations(&original, &permutations)
    }
}
