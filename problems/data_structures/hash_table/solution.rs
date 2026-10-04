use std::collections::LinkedList;

pub struct HashTable<K, V> {
    elements: Vec<LinkedList<(K, V)>>,
    count: usize,
}

impl<K: Hashable + std::cmp::PartialEq, V> Default for HashTable<K, V> {
    fn default() -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (default)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement default");
}
}

pub trait Hashable {
    fn hash(&self) -> usize;
}

impl<K: Hashable + std::cmp::PartialEq, V> HashTable<K, V> {
    pub fn new() -> HashTable<K, V> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}

    pub fn insert(&mut self, key: K, value: V) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (insert)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement insert");
}

    pub fn search(&self, key: K) -> Option<&V> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (search)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement search");
}

    fn resize(&mut self) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (resize)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement resize");
}
}


#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    struct TestKey(usize);

    impl Hashable for TestKey {
        fn hash(&self) -> usize {
            self.0
        }
    }

    #[test]
    fn test_insert_and_search() {
        let mut hash_table = HashTable::new();
        let key = TestKey(1);
        let value = TestKey(10);

        hash_table.insert(key, value);
        let result = hash_table.search(TestKey(1));

        assert_eq!(result, Some(&TestKey(10)));
    }

    #[test]
    fn test_resize() {
        let mut hash_table = HashTable::new();
        let initial_capacity = hash_table.elements.capacity();

        for i in 0..=initial_capacity * 3 / 4 {
            hash_table.insert(TestKey(i), TestKey(i + 10));
        }

        assert!(hash_table.elements.capacity() > initial_capacity);
    }

    #[test]
    fn test_search_nonexistent() {
        let mut hash_table = HashTable::new();
        let key = TestKey(1);
        let value = TestKey(10);

        hash_table.insert(key, value);
        let result = hash_table.search(TestKey(2));

        assert_eq!(result, None);
    }

    #[test]
    fn test_multiple_inserts_and_searches() {
        let mut hash_table = HashTable::new();
        for i in 0..10 {
            hash_table.insert(TestKey(i), TestKey(i + 100));
        }

        for i in 0..10 {
            let result = hash_table.search(TestKey(i));
            assert_eq!(result, Some(&TestKey(i + 100)));
        }
    }

    #[test]
    fn test_not_overwrite_existing_key() {
        let mut hash_table = HashTable::new();
        hash_table.insert(TestKey(1), TestKey(100));
        hash_table.insert(TestKey(1), TestKey(200));

        let result = hash_table.search(TestKey(1));
        assert_eq!(result, Some(&TestKey(100)));
    }

    #[test]
    fn test_empty_search() {
        let hash_table: HashTable<TestKey, TestKey> = HashTable::new();
        let result = hash_table.search(TestKey(1));

        assert_eq!(result, None);
    }
}
