use rand::random_range;
use std::{cmp::Ordering, marker::PhantomData, ptr::null_mut};

struct Node<K: Ord, V> {
    key: Option<K>,
    value: Option<V>,
    forward: Vec<*mut Node<K, V>>,
}

impl<K: Ord, V> Node<K, V> {
    pub fn new() -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}

    pub fn make_node(capacity: usize, key: K, value: V) -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (make_node)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement make_node");
}
}

/// A probabilistic data structure that maintains a sorted collection of key-value pairs.
///
/// A skip list is a data structure that allows O(log n) search, insertion, and deletion
/// on average by maintaining multiple levels of linked lists with probabilistic balancing.
pub struct SkipList<K: Ord, V> {
    header: *mut Node<K, V>,
    level: usize,
    max_level: usize,
    marker: PhantomData<Node<K, V>>,
}

impl<K: Ord, V> SkipList<K, V> {
    pub fn new(max_level: usize) -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}

    pub fn search(&self, searched_key: K) -> Option<&V> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (search)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement search");
}

    pub fn insert(&mut self, searched_key: K, new_value: V) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (insert)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement insert");
}

    pub fn delete(&mut self, searched_key: K) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (delete)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement delete");
}

    pub fn iter(&self) -> Iter<'_, K, V> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (iter)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement iter");
}
}

impl<K: Ord, V> Drop for SkipList<K, V> {
    fn drop(&mut self) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (drop)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement drop");
}
}

fn random_value(max: usize) -> usize {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (random_value)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement random_value");
}

pub struct Iter<'a, K: Ord, V> {
    current_node: *mut Node<K, V>,
    _marker: PhantomData<&'a SkipList<K, V>>,
}

impl<'a, K: Ord, V> Iter<'a, K, V> {
    pub fn new(skip_list: &'a SkipList<K, V>) -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}
}

impl<'a, K: Ord, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (next)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement next");
}
}


#[cfg(test)]
mod test {
    #[test]
    fn insert_and_delete() {
        let mut skip_list = super::SkipList::<&'static str, i32>::new(8);
        skip_list.insert("a", 10);
        skip_list.insert("b", 12);

        {
            let result = skip_list.search("b");
            assert!(result.is_some());
            assert_eq!(result, Some(&12));
        }

        {
            skip_list.delete("b");
            let result = skip_list.search("b");
            assert!(result.is_none());
        }

        skip_list.delete("a");
    }

    #[test]
    fn iterator() {
        let mut skip_list = super::SkipList::<&'static str, i32>::new(8);
        skip_list.insert("h", 22);
        skip_list.insert("a", 12);
        skip_list.insert("c", 11);

        let result: Vec<(&&'static str, &i32)> = skip_list.iter().collect();
        assert_eq!(result, vec![(&"a", &12), (&"c", &11), (&"h", &22)]);
    }

    #[test]
    fn cannot_search() {
        let mut skip_list = super::SkipList::<&'static str, i32>::new(8);

        {
            let result = skip_list.search("h");
            assert!(result.is_none());
        }

        skip_list.insert("h", 10);

        {
            let result = skip_list.search("a");
            assert!(result.is_none());
        }
    }

    #[test]
    fn delete_unsuccessfully() {
        let mut skip_list = super::SkipList::<&'static str, i32>::new(8);

        {
            let result = skip_list.delete("a");
            assert!(!result);
        }

        skip_list.insert("a", 10);

        {
            let result = skip_list.delete("b");
            assert!(!result);
        }
    }

    #[test]
    fn update_value_with_insert_operation() {
        let mut skip_list = super::SkipList::<&'static str, i32>::new(8);
        skip_list.insert("a", 10);

        {
            let result = skip_list.search("a");
            assert_eq!(result, Some(&10));
        }

        skip_list.insert("a", 100);

        {
            let result = skip_list.search("a");
            assert_eq!(result, Some(&100));
        }
    }
}
