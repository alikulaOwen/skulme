// Intro Sort (Also known as Introspective Sort)
// Introspective Sort is hybrid sort (Quick Sort + Heap Sort + Insertion Sort)
// https://en.wikipedia.org/wiki/Introsort
fn insertion_sort<T: Ord>(arr: &mut [T]) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (insertion_sort)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement insertion_sort");
}

fn heapify<T: Ord>(arr: &mut [T], n: usize, i: usize) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (heapify)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement heapify");
}

fn heap_sort<T: Ord>(arr: &mut [T]) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (heap_sort)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement heap_sort");
}

pub fn intro_sort<T: Ord>(arr: &mut [T]) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (intro_sort)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement intro_sort");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intro_sort() {
        // Test with integers
        let mut arr1 = vec![67, 34, 29, 15, 21, 9, 99];
        intro_sort(&mut arr1);
        assert_eq!(arr1, vec![9, 15, 21, 29, 34, 67, 99]);

        // Test with strings
        let mut arr2 = vec!["sydney", "london", "tokyo", "beijing", "mumbai"];
        intro_sort(&mut arr2);
        assert_eq!(arr2, vec!["beijing", "london", "mumbai", "sydney", "tokyo"]);

        // Test with an empty array
        let mut arr3: Vec<i32> = vec![];
        intro_sort(&mut arr3);
        assert_eq!(arr3, vec![]);
    }
}
