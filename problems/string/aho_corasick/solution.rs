use std::cell::RefCell;
use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::rc::{Rc, Weak};

#[derive(Default)]
struct ACNode {
    trans: BTreeMap<char, Rc<RefCell<ACNode>>>,
    suffix: Weak<RefCell<ACNode>>, // the suffix(fail) link
    lengths: Vec<usize>,           // lengths of matched patterns ended at this node
}

#[derive(Default)]
pub struct AhoCorasick {
    root: Rc<RefCell<ACNode>>,
}

impl AhoCorasick {
    pub fn new(words: &[&str]) -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}

    fn build_suffix(root: Rc<RefCell<ACNode>>) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (build_suffix)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement build_suffix");
}

    pub fn search<'a>(&self, s: &'a str) -> Vec<&'a str> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (search)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement search");
}
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aho_corasick() {
        let dict = ["abc", "abcd", "xyz", "acxy", "efg", "123", "678", "6543"];
        let ac = AhoCorasick::new(&dict);
        let res = ac.search("ababcxyzacxy12678acxy6543");
        assert_eq!(res, ["abc", "xyz", "acxy", "678", "acxy", "6543",]);
    }

    #[test]
    fn test_aho_corasick_with_utf8() {
        let dict = [
            "abc",
            "中文",
            "abc中",
            "abcd",
            "xyz",
            "acxy",
            "efg",
            "123",
            "678",
            "6543",
            "ハンバーガー",
        ];
        let ac = AhoCorasick::new(&dict);
        let res = ac.search("ababc中xyzacxy12678acxyハンバーガー6543中文");
        assert_eq!(
            res,
            [
                "abc",
                "abc中",
                "xyz",
                "acxy",
                "678",
                "acxy",
                "ハンバーガー",
                "6543",
                "中文"
            ]
        );
    }
}
