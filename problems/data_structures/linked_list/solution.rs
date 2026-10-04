use std::fmt::{self, Display, Formatter};
use std::marker::PhantomData;
use std::ptr::NonNull;

pub struct Node<T> {
    pub val: T,
    pub next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}
}

pub struct LinkedList<T> {
    pub length: u32,
    pub head: Option<NonNull<Node<T>>>,
    pub tail: Option<NonNull<Node<T>>>,
    // Act like we own boxed nodes since we construct and leak them
    marker: PhantomData<Box<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (default)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement default");
}
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}

    pub fn insert_at_head(&mut self, obj: T) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (insert_at_head)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement insert_at_head");
}

    pub fn insert_at_tail(&mut self, obj: T) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (insert_at_tail)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement insert_at_tail");
}

    pub fn insert_at_ith(&mut self, index: u32, obj: T) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (insert_at_ith)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement insert_at_ith");
}

    pub fn delete_head(&mut self) -> Option<T> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (delete_head)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement delete_head");
}

    pub fn delete_tail(&mut self) -> Option<T> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (delete_tail)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement delete_tail");
}

    pub fn delete_ith(&mut self, index: u32) -> Option<T> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (delete_ith)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement delete_ith");
}

    pub fn get(&self, index: i32) -> Option<&T> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (get)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement get");
}

    fn get_ith_node(node: Option<NonNull<Node<T>>>, index: i32) -> Option<NonNull<Node<T>>> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (get_ith_node)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement get_ith_node");
}
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (drop)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement drop");
}
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (fmt)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement fmt");
}
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (fmt)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement fmt");
}
}


#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use super::LinkedList;

    #[test]
    fn insert_at_tail_works() {
        let mut list = LinkedList::<i32>::new();
        let second_value = 2;
        list.insert_at_tail(1);
        list.insert_at_tail(second_value);
        println!("Linked List is {list}");
        match list.get(1) {
            Some(val) => assert_eq!(*val, second_value),
            None => panic!("Expected to find {second_value} at index 1"),
        }
    }
    #[test]
    fn insert_at_head_works() {
        let mut list = LinkedList::<i32>::new();
        let second_value = 2;
        list.insert_at_head(1);
        list.insert_at_head(second_value);
        println!("Linked List is {list}");
        match list.get(0) {
            Some(val) => assert_eq!(*val, second_value),
            None => panic!("Expected to find {second_value} at index 0"),
        }
    }

    #[test]
    fn insert_at_ith_can_add_to_tail() {
        let mut list = LinkedList::<i32>::new();
        let second_value = 2;
        list.insert_at_ith(0, 0);
        list.insert_at_ith(1, second_value);
        println!("Linked List is {list}");
        match list.get(1) {
            Some(val) => assert_eq!(*val, second_value),
            None => panic!("Expected to find {second_value} at index 1"),
        }
    }

    #[test]
    fn insert_at_ith_can_add_to_head() {
        let mut list = LinkedList::<i32>::new();
        let second_value = 2;
        list.insert_at_ith(0, 1);
        list.insert_at_ith(0, second_value);
        println!("Linked List is {list}");
        match list.get(0) {
            Some(val) => assert_eq!(*val, second_value),
            None => panic!("Expected to find {second_value} at index 0"),
        }
    }

    #[test]
    fn insert_at_ith_can_add_to_middle() {
        let mut list = LinkedList::<i32>::new();
        let second_value = 2;
        let third_value = 3;
        list.insert_at_ith(0, 1);
        list.insert_at_ith(1, second_value);
        list.insert_at_ith(1, third_value);
        println!("Linked List is {list}");
        match list.get(1) {
            Some(val) => assert_eq!(*val, third_value),
            None => panic!("Expected to find {third_value} at index 1"),
        }

        match list.get(2) {
            Some(val) => assert_eq!(*val, second_value),
            None => panic!("Expected to find {second_value} at index 1"),
        }
    }

    #[test]
    fn insert_at_ith_and_delete_at_ith_in_the_middle() {
        // Insert and delete in the middle of the list to ensure pointers are updated correctly
        let mut list = LinkedList::<i32>::new();
        let first_value = 0;
        let second_value = 1;
        let third_value = 2;
        let fourth_value = 3;

        list.insert_at_ith(0, first_value);
        list.insert_at_ith(1, fourth_value);
        list.insert_at_ith(1, third_value);
        list.insert_at_ith(1, second_value);

        list.delete_ith(2);
        list.insert_at_ith(2, third_value);

        for (i, expected) in [
            (0, first_value),
            (1, second_value),
            (2, third_value),
            (3, fourth_value),
        ] {
            match list.get(i) {
                Some(val) => assert_eq!(*val, expected),
                None => panic!("Expected to find {expected} at index {i}"),
            }
        }
    }

    #[test]
    fn insert_at_ith_and_delete_ith_work_over_many_iterations() {
        let mut list = LinkedList::<i32>::new();
        for i in 0..100 {
            list.insert_at_ith(i, i.try_into().unwrap());
        }

        // Pop even numbers to 50
        for i in 0..50 {
            println!("list.length {}", list.length);
            if i % 2 == 0 {
                list.delete_ith(i);
            }
        }

        assert_eq!(list.length, 75);

        // Insert even numbers back
        for i in 0..50 {
            if i % 2 == 0 {
                list.insert_at_ith(i, i.try_into().unwrap());
            }
        }

        assert_eq!(list.length, 100);

        // Ensure numbers were adderd back and we're able to traverse nodes
        if let Some(val) = list.get(78) {
            assert_eq!(*val, 78);
        } else {
            panic!("Expected to find 78 at index 78");
        }
    }

    #[test]
    fn delete_tail_works() {
        let mut list = LinkedList::<i32>::new();
        let first_value = 1;
        let second_value = 2;
        list.insert_at_tail(first_value);
        list.insert_at_tail(second_value);
        match list.delete_tail() {
            Some(val) => assert_eq!(val, 2),
            None => panic!("Expected to remove {second_value} at tail"),
        }

        println!("Linked List is {list}");
        match list.get(0) {
            Some(val) => assert_eq!(*val, first_value),
            None => panic!("Expected to find {first_value} at index 0"),
        }
    }

    #[test]
    fn delete_head_works() {
        let mut list = LinkedList::<i32>::new();
        let first_value = 1;
        let second_value = 2;
        list.insert_at_tail(first_value);
        list.insert_at_tail(second_value);
        match list.delete_head() {
            Some(val) => assert_eq!(val, 1),
            None => panic!("Expected to remove {first_value} at head"),
        }

        println!("Linked List is {list}");
        match list.get(0) {
            Some(val) => assert_eq!(*val, second_value),
            None => panic!("Expected to find {second_value} at index 0"),
        }
    }

    #[test]
    fn delete_ith_can_delete_at_tail() {
        let mut list = LinkedList::<i32>::new();
        let first_value = 1;
        let second_value = 2;
        list.insert_at_tail(first_value);
        list.insert_at_tail(second_value);
        match list.delete_ith(1) {
            Some(val) => assert_eq!(val, 2),
            None => panic!("Expected to remove {second_value} at tail"),
        }

        assert_eq!(list.length, 1);
    }

    #[test]
    fn delete_ith_can_delete_at_head() {
        let mut list = LinkedList::<i32>::new();
        let first_value = 1;
        let second_value = 2;
        list.insert_at_tail(first_value);
        list.insert_at_tail(second_value);
        match list.delete_ith(0) {
            Some(val) => assert_eq!(val, 1),
            None => panic!("Expected to remove {first_value} at tail"),
        }

        assert_eq!(list.length, 1);
    }

    #[test]
    fn delete_ith_can_delete_in_middle() {
        let mut list = LinkedList::<i32>::new();
        let first_value = 1;
        let second_value = 2;
        let third_value = 3;
        list.insert_at_tail(first_value);
        list.insert_at_tail(second_value);
        list.insert_at_tail(third_value);
        match list.delete_ith(1) {
            Some(val) => assert_eq!(val, 2),
            None => panic!("Expected to remove {second_value} at tail"),
        }

        match list.get(1) {
            Some(val) => assert_eq!(*val, third_value),
            None => panic!("Expected to find {third_value} at index 1"),
        }
    }

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.insert_at_tail(1);
        list.insert_at_tail(2);
        list.insert_at_tail(3);
        println!("Linked List is {list}");
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.insert_at_tail("A".to_string());
        list_str.insert_at_tail("B".to_string());
        list_str.insert_at_tail("C".to_string());
        println!("Linked List is {list_str}");
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn get_by_index_in_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.insert_at_tail(1);
        list.insert_at_tail(2);
        println!("Linked List is {list}");
        let retrived_item = list.get(1);
        assert!(retrived_item.is_some());
        assert_eq!(2, *retrived_item.unwrap());
    }

    #[test]
    fn get_by_index_in_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.insert_at_tail("A".to_string());
        list_str.insert_at_tail("B".to_string());
        println!("Linked List is {list_str}");
        let retrived_item = list_str.get(1);
        assert!(retrived_item.is_some());
        assert_eq!("B", *retrived_item.unwrap());
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn delete_ith_panics_if_index_equals_length() {
        let mut list = LinkedList::<i32>::new();
        list.insert_at_tail(1);
        list.insert_at_tail(2);
        // length is 2, so index 2 is out of bounds
        list.delete_ith(2);
    }
}
