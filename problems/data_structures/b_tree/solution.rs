use std::convert::TryFrom;
use std::fmt::Debug;
use std::mem;

struct Node<T> {
    keys: Vec<T>,
    children: Vec<Node<T>>,
}

pub struct BTree<T> {
    root: Node<T>,
    props: BTreeProps,
}

// Why to need a different Struct for props...
// Check - http://smallcultfollowing.com/babysteps/blog/2018/11/01/after-nll-interprocedural-conflicts/#fnref:improvement
struct BTreeProps {
    degree: usize,
    max_keys: usize,
    mid_key_index: usize,
}

impl<T> Node<T>
where
    T: Ord,
{
    fn new(degree: usize, _keys: Option<Vec<T>>, _children: Option<Vec<Node<T>>>) -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}

    fn is_leaf(&self) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (is_leaf)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement is_leaf");
}
}

impl BTreeProps {
    fn new(degree: usize) -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}

    fn is_maxed_out<T: Ord + Copy>(&self, node: &Node<T>) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (is_maxed_out)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement is_maxed_out");
}

    // Split Child expects the Child Node to be full
    /// Move the middle_key to parent node and split the child_node's
    /// keys/chilren_nodes into half
    fn split_child<T: Ord + Copy + Default>(&self, parent: &mut Node<T>, child_index: usize) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (split_child)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement split_child");
}

    fn insert_non_full<T: Ord + Copy + Default>(&mut self, node: &mut Node<T>, key: T) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (insert_non_full)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement insert_non_full");
}
    fn traverse_node<T: Ord + Debug>(node: &Node<T>, depth: usize) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (traverse_node)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement traverse_node");
}
}

impl<T> BTree<T>
where
    T: Ord + Copy + Debug + Default,
{
    pub fn new(branch_factor: usize) -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}

    pub fn insert(&mut self, key: T) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (insert)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement insert");
}

    pub fn traverse(&self) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (traverse)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement traverse");
}

    pub fn search(&self, key: T) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (search)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement search");
}
}


#[cfg(test)]
mod test {
    use super::BTree;

    macro_rules! test_search {
        ($($name:ident: $number_of_children:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let mut tree = BTree::new($number_of_children);
                tree.insert(10);
                tree.insert(20);
                tree.insert(30);
                tree.insert(5);
                tree.insert(6);
                tree.insert(7);
                tree.insert(11);
                tree.insert(12);
                tree.insert(15);
                assert!(!tree.search(4));
                assert!(tree.search(5));
                assert!(tree.search(6));
                assert!(tree.search(7));
                assert!(!tree.search(8));
                assert!(!tree.search(9));
                assert!(tree.search(10));
                assert!(tree.search(11));
                assert!(tree.search(12));
                assert!(!tree.search(13));
                assert!(!tree.search(14));
                assert!(tree.search(15));
                assert!(!tree.search(16));
            }
        )*
        }
    }

    test_search! {
        children_2: 2,
        children_3: 3,
        children_4: 4,
        children_5: 5,
        children_10: 10,
        children_60: 60,
        children_101: 101,
    }
}
