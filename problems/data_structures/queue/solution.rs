//! This module provides a generic `Queue` data structure, implemented using
//! Rust's `LinkedList` from the standard library. The queue follows the FIFO
//! (First-In-First-Out) principle, where elements are added to the back of
//! the queue and removed from the front.

use std::collections::LinkedList;

#[derive(Debug)]
pub struct Queue<T> {
    elements: LinkedList<T>,
}

impl<T> Queue<T> {
    // Creates a new empty Queue
    pub fn new() -> Queue<T> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}

    // Adds an element to the back of the queue
    pub fn enqueue(&mut self, value: T) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (enqueue)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement enqueue");
}

    // Removes and returns the front element from the queue, or None if empty
    pub fn dequeue(&mut self) -> Option<T> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (dequeue)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement dequeue");
}

    // Returns a reference to the front element of the queue, or None if empty
    pub fn peek_front(&self) -> Option<&T> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (peek_front)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement peek_front");
}

    // Returns a reference to the back element of the queue, or None if empty
    pub fn peek_back(&self) -> Option<&T> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (peek_back)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement peek_back");
}

    // Returns the number of elements in the queue
    pub fn len(&self) -> usize {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (len)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement len");
}

    // Checks if the queue is empty
    pub fn is_empty(&self) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (is_empty)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement is_empty");
}

    // Clears all elements from the queue
    pub fn drain(&mut self) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (drain)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement drain");
}
}

// Implementing the Default trait for Queue
impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (default)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement default");
}
}


#[cfg(test)]
mod tests {
    use super::Queue;

    #[test]
    fn test_queue_functionality() {
        let mut queue: Queue<usize> = Queue::default();

        assert!(queue.is_empty());
        queue.enqueue(8);
        queue.enqueue(16);
        assert!(!queue.is_empty());
        assert_eq!(queue.len(), 2);

        assert_eq!(queue.peek_front(), Some(&8));
        assert_eq!(queue.peek_back(), Some(&16));

        assert_eq!(queue.dequeue(), Some(8));
        assert_eq!(queue.len(), 1);
        assert_eq!(queue.peek_front(), Some(&16));
        assert_eq!(queue.peek_back(), Some(&16));

        queue.drain();
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
        assert_eq!(queue.dequeue(), None);
    }
}
