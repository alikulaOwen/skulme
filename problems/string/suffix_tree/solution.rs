// In computer science, a suffix tree (also called PAT tree or, in an earlier form, position tree)
// is a compressed trie containing all the suffixes of the given text as their keys and positions
// in the text as their values. Suffix trees allow particularly fast implementations of many
// important string operations. Source: https://en.wikipedia.org/wiki/Suffix_tree

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    pub sub: String,    // substring of input string
    pub ch: Vec<usize>, // vector of child nodes
}

impl Node {
    fn new(sub: String, children: Vec<usize>) -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}
    pub fn empty() -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (empty)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement empty");
}
}

pub struct SuffixTree {
    pub nodes: Vec<Node>,
}

impl SuffixTree {
    pub fn new(s: &str) -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}
    fn add_suffix(&mut self, suf: &str) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (add_suffix)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement add_suffix");
}
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suffix_tree() {
        let suf_tree = SuffixTree::new("banana$");
        assert_eq!(
            suf_tree.nodes,
            vec![
                Node {
                    sub: "".to_string(),
                    ch: vec![1, 8, 6, 10]
                },
                Node {
                    sub: "banana$".to_string(),
                    ch: vec![]
                },
                Node {
                    sub: "na$".to_string(),
                    ch: vec![]
                },
                Node {
                    sub: "na$".to_string(),
                    ch: vec![]
                },
                Node {
                    sub: "na".to_string(),
                    ch: vec![2, 5]
                },
                Node {
                    sub: "$".to_string(),
                    ch: vec![]
                },
                Node {
                    sub: "na".to_string(),
                    ch: vec![3, 7]
                },
                Node {
                    sub: "$".to_string(),
                    ch: vec![]
                },
                Node {
                    sub: "a".to_string(),
                    ch: vec![4, 9]
                },
                Node {
                    sub: "$".to_string(),
                    ch: vec![]
                },
                Node {
                    sub: "$".to_string(),
                    ch: vec![]
                }
            ]
        );
    }
}
