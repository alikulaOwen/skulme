/// Sorts the elements of `arr` in-place using radix sort.
///
/// Time complexity is `O((n + b) * logb(k))`, where `n` is the number of elements,
/// `b` is the base (the radix), and `k` is the largest element.
/// When `n` and `b` are roughly the same maginitude, this algorithm runs in linear time.
///
/// Space complexity is `O(n + b)`.
pub fn radix_sort(arr: &mut [u64]) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (radix_sort)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement radix_sort");
}


#[cfg(test)]
mod tests {
    use super::radix_sort;
    use crate::sorting::have_same_elements;
    use crate::sorting::is_sorted;

    #[test]
    fn empty() {
        let mut a: [u64; 0] = [];
        let cloned = a;
        radix_sort(&mut a);
        assert!(is_sorted(&a) && have_same_elements(&a, &cloned));
    }

    #[test]
    fn descending() {
        let mut v = vec![201, 127, 64, 37, 24, 4, 1];
        let cloned = v.clone();
        radix_sort(&mut v);
        assert!(is_sorted(&v) && have_same_elements(&v, &cloned));
    }

    #[test]
    fn ascending() {
        let mut v = vec![1, 4, 24, 37, 64, 127, 201];
        let cloned = v.clone();
        radix_sort(&mut v);
        assert!(is_sorted(&v) && have_same_elements(&v, &cloned));
    }
}
