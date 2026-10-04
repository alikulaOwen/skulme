// Author : cyrixninja
// Tree Sort Algorithm
// https://en.wikipedia.org/wiki/Tree_sort
// Wikipedia :A tree sort is a sort algorithm that builds a binary search tree from the elements to be sorted, and then traverses the tree (in-order) so that the elements come out in sorted order.
// Its typical use is sorting elements online: after each insertion, the set of elements seen so far is available in sorted order.

struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
    fn new(value: T) -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}
}

struct BinarySearchTree<T> {
    root: Option<Box<TreeNode<T>>>,
}

impl<T: Ord + Clone> BinarySearchTree<T> {
    fn new() -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}

    fn insert(&mut self, value: T) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (insert)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement insert");
}

    fn insert_recursive(root: Option<Box<TreeNode<T>>>, value: T) -> Box<TreeNode<T>> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (insert_recursive)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement insert_recursive");
}

    fn in_order_traversal(&self, result: &mut Vec<T>) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (in_order_traversal)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement in_order_traversal");
}

    fn in_order_recursive(root: &Option<Box<TreeNode<T>>>, result: &mut Vec<T>) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (in_order_recursive)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement in_order_recursive");
}
}

pub fn tree_sort<T: Ord + Clone>(arr: &mut Vec<T>) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (tree_sort)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement tree_sort");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_array() {
        let mut arr: Vec<i32> = vec![];
        tree_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_single_element() {
        let mut arr = vec![8];
        tree_sort(&mut arr);
        assert_eq!(arr, vec![8]);
    }

    #[test]
    fn test_already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        tree_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut arr = vec![5, 4, 3, 2, 1];
        tree_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_random() {
        let mut arr = vec![9, 6, 10, 11, 2, 19];
        tree_sort(&mut arr);
        assert_eq!(arr, vec![2, 6, 9, 10, 11, 19]);
    }
}
