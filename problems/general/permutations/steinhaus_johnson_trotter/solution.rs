/// <https://en.wikipedia.org/wiki/Steinhaus%E2%80%93Johnson%E2%80%93Trotter_algorithm>
pub fn steinhaus_johnson_trotter_permute<T: Clone>(array: &[T]) -> Vec<Vec<T>> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (steinhaus_johnson_trotter_permute)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement steinhaus_johnson_trotter_permute");
}


#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use crate::general::permutations::steinhaus_johnson_trotter::steinhaus_johnson_trotter_permute;
    use crate::general::permutations::tests::{
        assert_permutations, assert_valid_permutation, NotTooBigVec,
    };

    #[test]
    fn test_3_different_values() {
        let original = vec![1, 2, 3];
        let res = steinhaus_johnson_trotter_permute(&original);
        assert_eq!(res.len(), 6); // 3!
        for permut in res {
            assert_valid_permutation(&original, &permut)
        }
    }

    #[test]
    fn test_3_times_the_same_value() {
        let original = vec![1, 1, 1];
        let res = steinhaus_johnson_trotter_permute(&original);
        assert_eq!(res.len(), 6); // 3!
        for permut in res {
            assert_valid_permutation(&original, &permut)
        }
    }

    #[quickcheck]
    fn test_some_elements(NotTooBigVec { inner: original }: NotTooBigVec) {
        let permutations = steinhaus_johnson_trotter_permute(&original);
        assert_permutations(&original, &permutations)
    }
}
