//! A generic heap data structure.
//!
//! This module provides a `Heap` implementation that can function as either a
//! min-heap or a max-heap. It supports common heap operations such as adding,
//! removing, and iterating over elements. The heap can also be created from
//! an unsorted vector and supports custom comparators for flexible sorting
//! behavior.

use std::{cmp::Ord, slice::Iter};

/// A heap data structure that can be used as a min-heap, max-heap or with
/// custom comparators.
///
/// This struct manages a collection of items where the heap property is maintained.
/// The heap can be configured to order elements based on a provided comparator function,
/// allowing for both min-heap and max-heap functionalities, as well as custom sorting orders.
pub struct Heap<T> {
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T> {
    /// Creates a new, empty heap with a custom comparator function.
    ///
    /// # Parameters
    /// - `comparator`: A function that defines the heap's ordering.
    ///
    /// # Returns
    /// A new `Heap` instance.
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}

    /// Creates a heap from a vector and a custom comparator function.
    ///
    /// # Parameters
    /// - `items`: A vector of items to be turned into a heap.
    /// - `comparator`: A function that defines the heap's ordering.
    ///
    /// # Returns
    /// A `Heap` instance with the elements from the provided vector.
    pub fn from_vec(items: Vec<T>, comparator: fn(&T, &T) -> bool) -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (from_vec)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement from_vec");
}

    /// Constructs the heap from an unsorted vector by applying the heapify process.
    fn build_heap(&mut self) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (build_heap)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement build_heap");
}

    /// Returns the number of elements in the heap.
    ///
    /// # Returns
    /// The number of elements in the heap.
    pub fn len(&self) -> usize {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (len)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement len");
}

    /// Checks if the heap is empty.
    ///
    /// # Returns
    /// `true` if the heap is empty, `false` otherwise.
    pub fn is_empty(&self) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (is_empty)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement is_empty");
}

    /// Adds a new element to the heap and maintains the heap property.
    ///
    /// # Parameters
    /// - `value`: The value to add to the heap.
    pub fn add(&mut self, value: T) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (add)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement add");
}

    /// Removes and returns the root element from the heap.
    ///
    /// # Returns
    /// The root element if the heap is not empty, otherwise `None`.
    pub fn pop(&mut self) -> Option<T> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (pop)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement pop");
}

    /// Returns an iterator over the elements in the heap.
    ///
    /// # Returns
    /// An iterator over the elements in the heap, in their internal order.
    pub fn iter(&self) -> Iter<'_, T> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (iter)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement iter");
}

    /// Moves an element upwards to restore the heap property.
    ///
    /// # Parameters
    /// - `idx`: The index of the element to heapify up.
    fn heapify_up(&mut self, mut idx: usize) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (heapify_up)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement heapify_up");
}

    /// Moves an element downwards to restore the heap property.
    ///
    /// # Parameters
    /// - `idx`: The index of the element to heapify down.
    fn heapify_down(&mut self, mut idx: usize) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (heapify_down)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement heapify_down");
}

    /// Returns the index of the parent of the element at `idx`.
    ///
    /// # Parameters
    /// - `idx`: The index of the element.
    ///
    /// # Returns
    /// The index of the parent element if it exists, otherwise `None`.
    fn parent_idx(&self, idx: usize) -> Option<usize> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (parent_idx)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement parent_idx");
}

    /// Checks if the element at `idx` has children.
    ///
    /// # Parameters
    /// - `idx`: The index of the element.
    ///
    /// # Returns
    /// `true` if the element has children, `false` otherwise.
    fn children_present(&self, idx: usize) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (children_present)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement children_present");
}

    /// Returns the index of the left child of the element at `idx`.
    ///
    /// # Parameters
    /// - `idx`: The index of the element.
    ///
    /// # Returns
    /// The index of the left child.
    fn left_child_idx(&self, idx: usize) -> usize {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (left_child_idx)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement left_child_idx");
}

    /// Returns the index of the right child of the element at `idx`.
    ///
    /// # Parameters
    /// - `idx`: The index of the element.
    ///
    /// # Returns
    /// The index of the right child.
    fn right_child_idx(&self, idx: usize) -> usize {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (right_child_idx)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement right_child_idx");
}
}

impl<T> Heap<T>
where
    T: Ord,
{
    /// Creates a new min-heap.
    ///
    /// # Returns
    /// A new `Heap` instance configured as a min-heap.
    pub fn new_min() -> Heap<T> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new_min)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new_min");
}

    /// Creates a new max-heap.
    ///
    /// # Returns
    /// A new `Heap` instance configured as a max-heap.
    pub fn new_max() -> Heap<T> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new_max)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new_max");
}

    /// Creates a min-heap from an unsorted vector.
    ///
    /// # Parameters
    /// - `items`: A vector of items to be turned into a min-heap.
    ///
    /// # Returns
    /// A `Heap` instance configured as a min-heap.
    pub fn from_vec_min(items: Vec<T>) -> Heap<T> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (from_vec_min)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement from_vec_min");
}

    /// Creates a max-heap from an unsorted vector.
    ///
    /// # Parameters
    /// - `items`: A vector of items to be turned into a max-heap.
    ///
    /// # Returns
    /// A `Heap` instance configured as a max-heap.
    pub fn from_vec_max(items: Vec<T>) -> Heap<T> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (from_vec_max)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement from_vec_max");
}
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_heap() {
        let mut heap: Heap<i32> = Heap::new_max();
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = Heap::new_min();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.pop(), Some(9));
        heap.add(1);
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(11));
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_max_heap() {
        let mut heap = Heap::new_max();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.pop(), Some(11));
        assert_eq!(heap.pop(), Some(9));
        assert_eq!(heap.pop(), Some(4));
        heap.add(1);
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_iter_heap() {
        let mut heap = Heap::new_min();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);

        let mut iter = heap.iter();
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&9));
        assert_eq!(iter.next(), Some(&11));
        assert_eq!(iter.next(), None);

        assert_eq!(heap.len(), 4);
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.pop(), Some(9));
        assert_eq!(heap.pop(), Some(11));
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_from_vec_min() {
        let vec = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
        let mut heap = Heap::from_vec_min(vec);
        assert_eq!(heap.len(), 9);
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(2));
        heap.add(0);
        assert_eq!(heap.pop(), Some(0));
    }

    #[test]
    fn test_from_vec_max() {
        let vec = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
        let mut heap = Heap::from_vec_max(vec);
        assert_eq!(heap.len(), 9);
        assert_eq!(heap.pop(), Some(9));
        assert_eq!(heap.pop(), Some(6));
        assert_eq!(heap.pop(), Some(5));
        heap.add(10);
        assert_eq!(heap.pop(), Some(10));
    }
}
