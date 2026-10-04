// The public struct can hide the implementation detail
pub struct Stack<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> Stack<T> {
    // Self is an alias for Stack
    // We implement associated function name new for single-linked-list
    pub fn new() -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}

    // Here are the primary forms that self can take are: self, &mut self and &self.
    // Since push will modify the linked list, we need a mutable reference `&mut`.
    // The push method which the signature's first parameter is self
    pub fn push(&mut self, elem: T) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (push)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement push");
}

    /// The pop function removes the head and returns its value.
    ///
    /// To do so, we'll need to match the `head` of the list, which is of enum type `Option<T>`.\
    /// It has two variants: `Some(T)` and `None`.
    /// * `None` - the list is empty:
    ///   * return an enum `Result` of variant `Err()`, as there is nothing to pop.
    /// * `Some(node)` - the list is not empty:
    ///   * remove the head of the list,
    ///   * relink the list's head `head` to its following node `next`,
    ///   * return `Ok(elem)`.
    pub fn pop(&mut self) -> Result<T, &str> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (pop)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement pop");
}

    pub fn is_empty(&self) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (is_empty)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement is_empty");
}

    pub fn peek(&self) -> Option<&T> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (peek)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement peek");
}

    pub fn peek_mut(&mut self) -> Option<&mut T> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (peek_mut)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement peek_mut");
}

    pub fn into_iter_for_stack(self) -> IntoIter<T> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (into_iter_for_stack)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement into_iter_for_stack");
}
    pub fn iter(&self) -> Iter<'_, T> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (iter)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement iter");
}
    // '_ is the "explicitly elided lifetime" syntax of Rust
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (iter_mut)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement iter_mut");
}
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (default)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement default");
}
}

/// The drop method of singly linked list.
///
/// Here's a question: *Do we need to worry about cleaning up our list?*\
/// With the help of the ownership mechanism, the type `List` will be cleaned up automatically (dropped) after it goes out of scope.\
/// The Rust Compiler does so automacally. In other words, the `Drop` trait is implemented automatically.\
///
/// The `Drop` trait is implemented for our type `List` with the following order: `List->Link->Box<Node>->Node`.\
/// The `.drop()` method is tail recursive and will clean the element one by one, this recursion will stop at `Box<Node>`\
/// <https://rust-unofficial.github.io/too-many-lists/first-drop.html>
///
/// We wouldn't be able to drop the contents contained by the box after deallocating, so we need to manually write the iterative drop.
impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (drop)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement drop");
}
}

// Rust has nothing like a yield statement, and there are actually 3 different iterator traits to be implemented

// Collections are iterated in Rust using the Iterator trait, we define a struct implement Iterator
pub struct IntoIter<T>(Stack<T>);

impl<T> Iterator for IntoIter<T> {
    // This is declaring that every implementation of iterator has an associated type called Item
    type Item = T;
    // the reason iterator yield Option<self::Item> is because the interface coalesces the `has_next` and `get_next` concepts
    fn next(&mut self) -> Option<Self::Item> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (next)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement next");
}
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (next)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement next");
}
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (next)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement next");
}
}


#[cfg(test)]
mod test_stack {

    use super::*;

    #[test]
    fn basics() {
        let mut list = Stack::new();
        assert_eq!(list.pop(), Err("Stack is empty"));

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Ok(3));
        assert_eq!(list.pop(), Ok(2));

        list.push(4);
        list.push(5);

        assert!(!list.is_empty());

        assert_eq!(list.pop(), Ok(5));
        assert_eq!(list.pop(), Ok(4));

        assert_eq!(list.pop(), Ok(1));
        assert_eq!(list.pop(), Err("Stack is empty"));

        assert!(list.is_empty());
    }

    #[test]
    fn peek() {
        let mut list = Stack::new();
        assert_eq!(list.peek(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        match list.peek_mut() {
            None => (),
            Some(value) => *value = 42,
        };

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Ok(42));
    }

    #[test]
    fn into_iter() {
        let mut list = Stack::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter_for_stack();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = Stack::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list = Stack::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}
