//Wiggle Sort.
//Given an unsorted array nums, reorder it such
//that nums[0] < nums[1] > nums[2] < nums[3]....
//For example:
//if input numbers = [3, 5, 2, 1, 6, 4]
//one possible Wiggle Sorted answer is [3, 5, 1, 6, 2, 4].

pub fn wiggle_sort(nums: &mut Vec<i32>) -> &mut Vec<i32> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (wiggle_sort)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement wiggle_sort");
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::sorting::have_same_elements;

    fn is_wiggle_sorted(nums: &[i32]) -> bool {
        if nums.is_empty() {
            return true;
        }
        let mut previous = nums[0];
        let mut result = true;
        nums.iter().enumerate().skip(1).for_each(|(i, &item)| {
            if i != 0 {
                result =
                    result && ((i % 2 == 1 && previous < item) || (i % 2 == 0 && previous > item));
            }

            previous = item;
        });
        result
    }

    #[test]
    fn wingle_elements() {
        let arr = vec![3, 5, 2, 1, 6, 4];
        let mut cloned = arr.clone();
        let res = wiggle_sort(&mut cloned);
        assert!(is_wiggle_sorted(res));
        assert!(have_same_elements(res, &arr));
    }

    #[test]
    fn odd_number_of_elements() {
        let arr = vec![4, 1, 3, 5, 2];
        let mut cloned = arr.clone();
        let res = wiggle_sort(&mut cloned);
        assert!(is_wiggle_sorted(res));
        assert!(have_same_elements(res, &arr));
    }

    #[test]
    fn repeated_elements() {
        let arr = vec![5, 5, 5, 5];
        let mut cloned = arr.clone();
        let res = wiggle_sort(&mut cloned);

        // Negative test, can't be wiggle sorted
        assert!(!is_wiggle_sorted(res));
        assert!(have_same_elements(res, &arr));
    }
}
