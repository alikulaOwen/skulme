/// From Wikipedia:
/// Tournament sort is a sorting algorithm. It improves upon the naive
/// selection sort by using a priority queue to find the next element in
/// the sort.
///
/// Time complexity is `O(n log n)`, where `n` is the number of elements.
/// Space complexity is `O(n)`.
pub fn tournament_sort<T>(arr: &[T]) -> Vec<T>
where
    T: Ord + Clone,
{
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (tournament_sort)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement tournament_sort");
}

fn min_opt<T>(a: &Option<T>, b: &Option<T>) -> Option<T>
where
    T: Ord + Clone,
{
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (min_opt)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement min_opt");
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::sorting::have_same_elements;
    use crate::sorting::is_sorted;

    #[test]
    fn descending() {
        let arr = vec![6, 5, 4, 3, 2, 1];
        let res = tournament_sort(&arr);
        assert!(is_sorted(&res) && have_same_elements(&res, &arr));
    }

    #[test]
    fn empty() {
        let arr = Vec::<i32>::new();
        let res = tournament_sort(&arr);
        assert!(is_sorted(&res) && have_same_elements(&res, &arr));
    }

    #[test]
    fn negative_numbers() {
        let arr = vec![-32, -54, -65, -12, -7];
        let res = tournament_sort(&arr);
        assert!(is_sorted(&res) && have_same_elements(&res, &arr));
    }

    #[test]
    fn one_element() {
        let arr = vec![1];
        let res = tournament_sort(&arr);
        assert!(is_sorted(&res) && have_same_elements(&res, &arr));
    }

    #[test]
    fn pre_sorted() {
        let arr = vec![5, 12, 23, 54, 57, 60];
        let res = tournament_sort(&arr);
        assert!(is_sorted(&res) && have_same_elements(&res, &arr));
    }

    #[test]
    fn repeated_elements() {
        let arr = vec![42, 42, 42, 42];
        let res = tournament_sort(&arr);
        assert_eq!(&res, &arr);
    }
}
