// floyds_algorithm.rs
// https://github.com/rust-lang/rust/blob/master/library/alloc/src/collections/linked_list.rs#L113
// use std::collections::linked_list::LinkedList;
// https://www.reddit.com/r/rust/comments/t7wquc/is_it_possible_to_solve_leetcode_problem141/

use crate::data_structures::linked_list::LinkedList; // Import the LinkedList from linked_list.rs

pub fn detect_cycle<T>(linked_list: &LinkedList<T>) -> Option<usize> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (detect_cycle)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement detect_cycle");
}

pub fn has_cycle<T>(linked_list: &LinkedList<T>) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (has_cycle)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement has_cycle");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_cycle_no_cycle() {
        let mut linked_list = LinkedList::new();
        linked_list.insert_at_tail(1);
        linked_list.insert_at_tail(2);
        linked_list.insert_at_tail(3);

        assert!(!has_cycle(&linked_list));

        assert_eq!(detect_cycle(&linked_list), None);
    }

    #[test]
    fn test_detect_cycle_with_cycle() {
        let mut linked_list = LinkedList::new();
        linked_list.insert_at_tail(1);
        linked_list.insert_at_tail(2);
        linked_list.insert_at_tail(3);

        // Create a cycle for testing
        unsafe {
            if let Some(mut tail) = linked_list.tail {
                if let Some(head) = linked_list.head {
                    tail.as_mut().next = Some(head);
                }
            }
        }

        assert!(has_cycle(&linked_list));
        assert_eq!(detect_cycle(&linked_list), Some(3));
    }
}
