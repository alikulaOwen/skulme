// From Wikipedia: Pigeonhole sorting is a sorting algorithm that is suitable for sorting lists of elements where the number of elements (n) and the length of the range of possible key values (N) are approximately the same. It requires O(n + N) time.

pub fn pigeonhole_sort(array: &mut [i32]) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (pigeonhole_sort)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement pigeonhole_sort");
}


#[cfg(test)]
mod tests {
    use super::super::is_sorted;
    use super::*;

    #[test]
    fn test1() {
        let mut arr1 = [3, 3, 3, 1, 2, 6, 5, 5, 5, 4, 1, 6, 3];
        pigeonhole_sort(&mut arr1);
        assert!(is_sorted(&arr1));
        let mut arr2 = [6, 5, 4, 3, 2, 1];
        pigeonhole_sort(&mut arr2);
        assert!(is_sorted(&arr2));
    }
}
