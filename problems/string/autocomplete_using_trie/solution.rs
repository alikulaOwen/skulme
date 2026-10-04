/*
    It autocomplete by prefix using added words.

    word List => ["apple", "orange", "oregano"]
    prefix => "or"
    matches => ["orange", "oregano"]
*/

use std::collections::HashMap;

const END: char = '#';

#[derive(Debug)]
struct Trie(HashMap<char, Box<Trie>>);

impl Trie {
    fn new() -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}

    fn insert(&mut self, text: &str) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (insert)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement insert");
}

    fn find(&self, prefix: &str) -> Vec<String> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (find)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement find");
}

    fn _elements(map: &Trie) -> Vec<String> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (_elements)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement _elements");
}
}

pub struct Autocomplete {
    trie: Trie,
}

impl Autocomplete {
    fn new() -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}

    pub fn insert_words<T: AsRef<str>>(&mut self, words: &[T]) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (insert_words)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement insert_words");
}

    pub fn find_words(&self, prefix: &str) -> Vec<String> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (find_words)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement find_words");
}
}

impl Default for Autocomplete {
    fn default() -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (default)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement default");
}
}


#[cfg(test)]
mod tests {
    use super::Autocomplete;

    #[test]
    fn test_autocomplete() {
        let words = vec!["apple", "orange", "oregano"];

        let mut auto_complete = Autocomplete::new();
        auto_complete.insert_words(&words);

        let prefix = "app";
        let mut auto_completed_words = auto_complete.find_words(prefix);

        let mut apple = vec!["apple"];
        apple.sort();

        auto_completed_words.sort();
        assert_eq!(auto_completed_words, apple);

        let prefix = "or";
        let mut auto_completed_words = auto_complete.find_words(prefix);

        let mut prefix_or = vec!["orange", "oregano"];
        prefix_or.sort();

        auto_completed_words.sort();
        assert_eq!(auto_completed_words, prefix_or);
    }
}
