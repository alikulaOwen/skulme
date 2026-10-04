use std::boxed::Box;
use std::cmp::{Ord, Ordering};
use std::iter::Iterator;
use std::ptr::null_mut;

#[derive(Copy, Clone)]
enum Color {
    Red,
    Black,
}

pub struct RBNode<K: Ord, V> {
    key: K,
    value: V,
    color: Color,
    parent: *mut RBNode<K, V>,
    left: *mut RBNode<K, V>,
    right: *mut RBNode<K, V>,
}

impl<K: Ord, V> RBNode<K, V> {
    fn new(key: K, value: V) -> RBNode<K, V> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}
}

pub struct RBTree<K: Ord, V> {
    root: *mut RBNode<K, V>,
}

impl<K: Ord, V> Default for RBTree<K, V> {
    fn default() -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (default)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement default");
}
}

impl<K: Ord, V> RBTree<K, V> {
    pub fn new() -> RBTree<K, V> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}

    pub fn find(&self, key: &K) -> Option<&V> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (find)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement find");
}

    pub fn insert(&mut self, key: K, value: V) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (insert)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement insert");
}

    pub fn delete(&mut self, key: &K) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (delete)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement delete");
}

    pub fn iter<'a>(&self) -> RBTreeIterator<'a, K, V> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (iter)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement iter");
}
}

#[inline]
unsafe fn insert_fixup<K: Ord, V>(tree: &mut RBTree<K, V>, mut node: *mut RBNode<K, V>) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (insert_fixup)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement insert_fixup");
}

#[inline]
unsafe fn delete_fixup<K: Ord, V>(tree: &mut RBTree<K, V>, mut parent: *mut RBNode<K, V>) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (delete_fixup)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement delete_fixup");
}

#[inline]
unsafe fn left_rotate<K: Ord, V>(tree: &mut RBTree<K, V>, x: *mut RBNode<K, V>) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (left_rotate)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement left_rotate");
}

#[inline]
unsafe fn right_rotate<K: Ord, V>(tree: &mut RBTree<K, V>, x: *mut RBNode<K, V>) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (right_rotate)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement right_rotate");
}

#[inline]
unsafe fn replace_node<K: Ord, V>(
    tree: &mut RBTree<K, V>,
    parent: *mut RBNode<K, V>,
    node: *mut RBNode<K, V>,
    new: *mut RBNode<K, V>,
) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (replace_node)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement replace_node");
}

pub struct RBTreeIterator<'a, K: Ord, V> {
    stack: Vec<&'a RBNode<K, V>>,
}

impl<'a, K: Ord, V> Iterator for RBTreeIterator<'a, K, V> {
    type Item = &'a RBNode<K, V>;
    fn next(&mut self) -> Option<Self::Item> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (next)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement next");
}
}


#[cfg(test)]
mod tests {
    use super::RBTree;

    #[test]
    fn find() {
        let mut tree = RBTree::<usize, char>::new();
        for (k, v) in "hello, world!".chars().enumerate() {
            tree.insert(k, v);
        }
        assert_eq!(*tree.find(&3).unwrap_or(&'*'), 'l');
        assert_eq!(*tree.find(&6).unwrap_or(&'*'), ' ');
        assert_eq!(*tree.find(&8).unwrap_or(&'*'), 'o');
        assert_eq!(*tree.find(&12).unwrap_or(&'*'), '!');
    }

    #[test]
    fn insert() {
        let mut tree = RBTree::<usize, char>::new();
        for (k, v) in "hello, world!".chars().enumerate() {
            tree.insert(k, v);
        }
        let s: String = tree.iter().map(|x| x.value).collect();
        assert_eq!(s, "hello, world!");
    }

    #[test]
    fn delete() {
        let mut tree = RBTree::<usize, char>::new();
        for (k, v) in "hello, world!".chars().enumerate() {
            tree.insert(k, v);
        }
        tree.delete(&1);
        tree.delete(&3);
        tree.delete(&5);
        tree.delete(&7);
        tree.delete(&11);
        let s: String = tree.iter().map(|x| x.value).collect();
        assert_eq!(s, "hlo orl!");
    }

    #[test]
    fn delete_edge_case_null_pointer_guard() {
        let mut tree = RBTree::<i8, i8>::new();
        tree.insert(4, 4);
        tree.insert(2, 2);
        tree.insert(5, 5);
        tree.insert(0, 0);
        tree.insert(3, 3);
        tree.insert(-1, -1);
        tree.insert(1, 1);
        tree.insert(-2, -2);
        tree.insert(6, 6);
        tree.insert(7, 7);
        tree.insert(8, 8);
        tree.delete(&1);
        tree.delete(&3);
        tree.delete(&-1);
    }
}
