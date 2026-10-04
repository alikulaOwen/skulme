use std::{
    cmp::Ordering,
    iter::FromIterator,
    mem,
    ops::Not,
    time::{SystemTime, UNIX_EPOCH},
};

/// An internal node of an `Treap`.
struct TreapNode<T: Ord> {
    value: T,
    priority: usize,
    left: Option<Box<TreapNode<T>>>,
    right: Option<Box<TreapNode<T>>>,
}

/// A set based on a Treap (Randomized Binary Search Tree).
///
/// A Treap is a self-balancing binary search tree. It matains a priority value for each node, such
/// that for every node, its children will have lower priority than itself. So, by just looking at
/// the priority, it is like a heap, and this is where the name, Treap, comes from, Tree + Heap.
pub struct Treap<T: Ord> {
    root: Option<Box<TreapNode<T>>>,
    length: usize,
}

/// Refers to the left or right subtree of a `Treap`.
#[derive(Clone, Copy)]
enum Side {
    Left,
    Right,
}

impl<T: Ord> Treap<T> {
    pub fn new() -> Treap<T> {
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

    /// Adds a value to the tree
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

/// Generating random number, should use rand::Rng if possible.
fn rand() -> usize {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (rand)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement rand");
}

/// Recursive helper function for `Treap` insertion.
fn insert<T: Ord>(tree: &mut Option<Box<TreapNode<T>>>, value: T) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (insert)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement insert");
}

/// Recursive helper function for `Treap` deletion
fn remove<T: Ord>(tree: &mut Option<Box<TreapNode<T>>>, value: &T) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (remove)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement remove");
}

impl<T: Ord> TreapNode<T> {
    /// Returns a reference to the left or right child.
    fn child(&self, side: Side) -> &Option<Box<TreapNode<T>>> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (child)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement child");
}

    /// Returns a mutable reference to the left or right child.
    fn child_mut(&mut self, side: Side) -> &mut Option<Box<TreapNode<T>>> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (child_mut)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement child_mut");
}

    /// Returns the priority of the left or right subtree.
    fn priority(&self, side: Side) -> usize {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (priority)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement priority");
}

    /// Performs a left or right rotation
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

    
#[cfg(test)]
    fn is_valid(&self) -> bool {
        self.priority >= self.priority(Side::Left) && self.priority >= self.priority(Side::Right)
    }
}

impl<T: Ord> Default for Treap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl Not for Side {
    type Output = Side;

    fn not(self) -> Self::Output {
        match self {
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }
}

impl<T: Ord> FromIterator<T> for Treap<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut tree = Treap::new();
        for value in iter {
            tree.insert(value);
        }
        tree
    }
}

/// An iterator over the nodes of an `Treap`.
///
/// This struct is created by the `node_iter` method of `Treap`.
struct NodeIter<'a, T: Ord> {
    stack: Vec<&'a TreapNode<T>>,
}

impl<'a, T: Ord> Iterator for NodeIter<'a, T> {
    type Item = &'a TreapNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            // Push left path of right subtree to stack
            let mut child = &node.right;
            while let Some(subtree) = child {
                self.stack.push(subtree.as_ref());
                child = &subtree.left;
            }
            Some(node)
        } else {
            None
        }
    }
}

/// An iterator over the items of an `Treap`.
///
/// This struct is created by the `iter` method of `Treap`.
pub struct Iter<'a, T: Ord> {
    node_iter: NodeIter<'a, T>,
}

impl<'a, T: Ord> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        match self.node_iter.next() {
            Some(node) => Some(&node.value),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Treap;

    /// Returns `true` if all nodes in the tree are valid.
    fn is_valid<T: Ord>(tree: &Treap<T>) -> bool {
        tree.node_iter().all(|n| n.is_valid())
    }

    #[test]
    fn len() {
        let tree: Treap<_> = (1..4).collect();
        assert_eq!(tree.len(), 3);
    }

    #[test]
    fn contains() {
        let tree: Treap<_> = (1..4).collect();
        assert!(tree.contains(&1));
        assert!(!tree.contains(&4));
    }

    #[test]
    fn insert() {
        let mut tree = Treap::new();
        // First insert succeeds
        assert!(tree.insert(1));
        // Second insert fails
        assert!(!tree.insert(1));
    }

    #[test]
    fn remove() {
        let mut tree: Treap<_> = (1..8).collect();
        // First remove succeeds
        assert!(tree.remove(&4));
        // Second remove fails
        assert!(!tree.remove(&4));
    }

    #[test]
    fn sorted() {
        let tree: Treap<_> = (1..8).rev().collect();
        assert!((1..8).eq(tree.iter().copied()));
    }

    #[test]
    fn valid() {
        let mut tree: Treap<_> = (1..8).collect();
        assert!(is_valid(&tree));
        for x in 1..8 {
            tree.remove(&x);
            assert!(is_valid(&tree));
        }
    }
}
