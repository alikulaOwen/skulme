// Author : cyrixninja
// Interquartile Range  : An implementation of interquartile range (IQR) which is a measure of statistical
//                        dispersion, which is the spread of the data.
// Wikipedia Reference  : https://en.wikipedia.org/wiki/Interquartile_range

use std::cmp::Ordering;

pub fn find_median(numbers: &[f64]) -> f64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (find_median)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement find_median");
}

pub fn interquartile_range(numbers: &[f64]) -> f64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (interquartile_range)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement interquartile_range");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_median() {
        let numbers1 = vec![1.0, 2.0, 2.0, 3.0, 4.0];
        assert_eq!(find_median(&numbers1), 2.0);

        let numbers2 = vec![1.0, 2.0, 2.0, 3.0, 4.0, 4.0];
        assert_eq!(find_median(&numbers2), 2.5);

        let numbers3 = vec![-1.0, 2.0, 0.0, 3.0, 4.0, -4.0];
        assert_eq!(find_median(&numbers3), 1.0);

        let numbers4 = vec![1.1, 2.2, 2.0, 3.3, 4.4, 4.0];
        assert_eq!(find_median(&numbers4), 2.75);
    }

    #[test]
    fn test_interquartile_range() {
        let numbers1 = vec![4.0, 1.0, 2.0, 3.0, 2.0];
        assert_eq!(interquartile_range(&numbers1), 2.0);

        let numbers2 = vec![-2.0, -7.0, -10.0, 9.0, 8.0, 4.0, -67.0, 45.0];
        assert_eq!(interquartile_range(&numbers2), 17.0);

        let numbers3 = vec![-2.1, -7.1, -10.1, 9.1, 8.1, 4.1, -67.1, 45.1];
        assert_eq!(interquartile_range(&numbers3), 17.2);

        let numbers4 = vec![0.0, 0.0, 0.0, 0.0, 0.0];
        assert_eq!(interquartile_range(&numbers4), 0.0);
    }

    #[test]
    #[should_panic(expected = "Error: The list is empty. Please provide a non-empty list.")]
    fn test_interquartile_range_empty_list() {
        let numbers: Vec<f64> = vec![];
        interquartile_range(&numbers);
    }
}
