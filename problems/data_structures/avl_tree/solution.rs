use std::{
    cmp::{max, Ordering},
    iter::FromIterator,
    mem,
    ops::Not,
};

/// An internal node of an `AVLTree`.
struct AVLNode<T: Ord> {
    value: T,
    height: usize,
    left: Option<Box<AVLNode<T>>>,
    right: Option<Box<AVLNode<T>>>,
}

/// A set based on an AVL Tree.
///
/// An AVL Tree is a self-balancing binary search tree. It tracks the height of each node
/// and performs internal rotations to maintain a height difference of at most 1 between
/// each sibling pair.
pub struct AVLTree<T: Ord> {
    root: Option<Box<AVLNode<T>>>,
    length: usize,
}

/// Refers to the left or right subtree of an `AVLNode`.
#[derive(Clone, Copy)]
enum Side {
    Left,
    Right,
}

impl<T: Ord> AVLTree<T> {
    /// Creates an empty `AVLTree`.
    pub fn new() -> AVLTree<T> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}

    /// Returns `true` if the tree contains a value.
    pub fn contains(&self, value: &T) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (contains)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement contains");
}

    /// Adds a value to the tree.
    ///
    /// Returns `true` if the tree did not yet contain the value.
    pub fn insert(&mut self, value: T) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (insert)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement insert");
}

    /// Removes a value from the tree.
    ///
    /// Returns `true` if the tree contained the value.
    pub fn remove(&mut self, value: &T) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (remove)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement remove");
}

    /// Returns the number of values in the tree.
    pub fn len(&self) -> usize {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (len)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement len");
}

    /// Returns `true` if the tree contains no values.
    pub fn is_empty(&self) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (is_empty)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement is_empty");
}

    /// Returns an iterator that visits the nodes in the tree in order.
    fn node_iter(&self) -> NodeIter<'_, T> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (node_iter)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement node_iter");
}

    /// Returns an iterator that visits the values in the tree in ascending order.
    pub fn iter(&self) -> Iter<'_, T> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (iter)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement iter");
}
}

/// Recursive helper function for `AVLTree` insertion.
fn insert<T: Ord>(tree: &mut Option<Box<AVLNode<T>>>, value: T) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (insert)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement insert");
}

/// Recursive helper function for `AVLTree` deletion.
fn remove<T: Ord>(tree: &mut Option<Box<AVLNode<T>>>, value: &T) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (remove)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement remove");
}

/// Merges two trees and returns the root of the merged tree.
fn merge<T: Ord>(left: Box<AVLNode<T>>, right: Box<AVLNode<T>>) -> Box<AVLNode<T>> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (merge)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement merge");
}

/// Removes the smallest node from the tree, if one exists.
fn take_min<T: Ord>(tree: &mut Option<Box<AVLNode<T>>>) -> Option<Box<AVLNode<T>>> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (take_min)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement take_min");
}

impl<T: Ord> AVLNode<T> {
    /// Returns a reference to the left or right child.
    fn child(&self, side: Side) -> &Option<Box<AVLNode<T>>> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (child)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement child");
}

    /// Returns a mutable reference to the left or right child.
    fn child_mut(&mut self, side: Side) -> &mut Option<Box<AVLNode<T>>> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (child_mut)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement child_mut");
}

    /// Returns the height of the left or right subtree.
    fn height(&self, side: Side) -> usize {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (height)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement height");
}

    /// Returns the height difference between the left and right subtrees.
    fn balance_factor(&self) -> i8 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (balance_factor)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement balance_factor");
}

    /// Recomputes the `height` field.
    fn update_height(&mut self) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (update_height)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement update_height");
}

    /// Performs a left or right rotation.
    fn rotate(&mut self, side: Side) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (rotate)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement rotate");
}

    /// Performs left or right tree rotations to balance this node.
    fn rebalance(&mut self) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (rebalance)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement rebalance");
}
}

impl<T: Ord> Default for AVLTree<T> {
    fn default() -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (default)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement default");
}
}

impl Not for Side {
    type Output = Side;

    fn not(self) -> Self::Output {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (not)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement not");
}
}

impl<T: Ord> FromIterator<T> for AVLTree<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (from_iter)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement from_iter");
}
}

/// An iterator over the nodes of an `AVLTree`.
///
/// This struct is created by the `node_iter` method of `AVLTree`.
struct NodeIter<'a, T: Ord> {
    stack: Vec<&'a AVLNode<T>>,
}

impl<'a, T: Ord> Iterator for NodeIter<'a, T> {
    type Item = &'a AVLNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (next)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement next");
}
}

/// An iterator over the items of an `AVLTree`.
///
/// This struct is created by the `iter` method of `AVLTree`.
pub struct Iter<'a, T: Ord> {
    node_iter: NodeIter<'a, T>,
}

impl<'a, T: Ord> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (next)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement next");
}
}


#[cfg(test)]
mod tests {
    use super::AVLTree;

    /// Returns `true` if all nodes in the tree are balanced.
    fn is_balanced<T: Ord>(tree: &AVLTree<T>) -> bool {
        tree.node_iter()
            .all(|n| (-1..=1).contains(&n.balance_factor()))
    }

    #[test]
    fn len() {
        let tree: AVLTree<_> = (1..4).collect();
        assert_eq!(tree.len(), 3);
    }

    #[test]
    fn contains() {
        let tree: AVLTree<_> = (1..4).collect();
        assert!(tree.contains(&1));
        assert!(!tree.contains(&4));
    }

    #[test]
    fn insert() {
        let mut tree = AVLTree::new();
        // First insert succeeds
        assert!(tree.insert(1));
        // Second insert fails
        assert!(!tree.insert(1));
    }

    #[test]
    fn remove() {
        let mut tree: AVLTree<_> = (1..8).collect();
        // First remove succeeds
        assert!(tree.remove(&4));
        // Second remove fails
        assert!(!tree.remove(&4));
    }

    #[test]
    fn sorted() {
        let tree: AVLTree<_> = (1..8).rev().collect();
        assert!((1..8).eq(tree.iter().copied()));
    }

    #[test]
    fn balanced() {
        let mut tree: AVLTree<_> = (1..8).collect();
        assert!(is_balanced(&tree));
        for x in 1..8 {
            tree.remove(&x);
            assert!(is_balanced(&tree));
        }
    }
}
