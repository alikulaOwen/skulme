use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

/// Here's a basic (naive) implementation for generating permutations
pub fn permute<T: Clone + Debug>(arr: &[T]) -> Vec<Vec<T>> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (permute)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement permute");
}

fn permute_recurse<T: Clone + Debug>(arr: &mut Vec<T>, k: usize, collector: &mut Vec<Vec<T>>) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (permute_recurse)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement permute_recurse");
}

/// A common variation of generating permutations is to generate only unique permutations
/// Of course, we could use the version above together with a Set as collector instead of a Vec.
/// But let's try something different: how can we avoid to generate duplicated permutations in the first place, can we tweak the algorithm above?
pub fn permute_unique<T: Clone + Debug + Eq + Hash + Copy>(arr: &[T]) -> Vec<Vec<T>> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (permute_unique)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement permute_unique");
}

fn permute_recurse_unique<T: Clone + Debug + Eq + Hash + Copy>(
    arr: &mut Vec<T>,
    k: usize,
    collector: &mut Vec<Vec<T>>,
) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (permute_recurse_unique)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement permute_recurse_unique");
}


#[cfg(test)]
mod tests {
    use crate::general::permutations::naive::{permute, permute_unique};
    use crate::general::permutations::tests::{
        assert_permutations, assert_valid_permutation, NotTooBigVec,
    };
    use quickcheck_macros::quickcheck;
    use std::collections::HashSet;

    #[test]
    fn test_3_different_values() {
        let original = vec![1, 2, 3];
        let res = permute(&original);
        assert_eq!(res.len(), 6); // 3!
        for permut in res {
            assert_valid_permutation(&original, &permut)
        }
    }

    #[test]
    fn empty_array() {
        let empty: std::vec::Vec<u8> = vec![];
        assert_eq!(permute(&empty), vec![vec![]]);
        assert_eq!(permute_unique(&empty), vec![vec![]]);
    }

    #[test]
    fn test_3_times_the_same_value() {
        let original = vec![1, 1, 1];
        let res = permute(&original);
        assert_eq!(res.len(), 6); // 3!
        for permut in res {
            assert_valid_permutation(&original, &permut)
        }
    }

    #[quickcheck]
    fn test_some_elements(NotTooBigVec { inner: original }: NotTooBigVec) {
        let permutations = permute(&original);
        assert_permutations(&original, &permutations)
    }

    #[test]
    fn test_unique_values() {
        let original = vec![1, 1, 2, 2];
        let unique_permutations = permute_unique(&original);
        let every_permutation = permute(&original);
        for unique_permutation in &unique_permutations {
            assert!(every_permutation.contains(unique_permutation));
        }
        assert_eq!(
            unique_permutations.len(),
            every_permutation.iter().collect::<HashSet<_>>().len()
        )
    }
}
