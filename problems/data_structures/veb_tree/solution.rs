// This struct implements Van Emde Boas tree (VEB tree). It stores integers in range [0, U), where
// O is any integer that is a power of 2. It supports operations such as insert, search,
// predecessor, and successor in O(log(log(U))) time. The structure takes O(U) space.
pub struct VebTree {
    size: u32,
    child_size: u32, // Set to square root of size. Cache here to avoid recomputation.
    min: u32,
    max: u32,
    summary: Option<Box<VebTree>>,
    cluster: Vec<VebTree>,
}

impl VebTree {
    /// Create a new, empty VEB tree. The tree will contain number of elements equal to size
    /// rounded up to the nearest power of two.
    pub fn new(size: u32) -> VebTree {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}

    fn high(&self, value: u32) -> u32 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (high)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement high");
}

    fn low(&self, value: u32) -> u32 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (low)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement low");
}

    fn index(&self, cluster: u32, offset: u32) -> u32 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (index)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement index");
}

    pub fn min(&self) -> u32 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (min)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement min");
}

    pub fn max(&self) -> u32 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (max)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement max");
}

    pub fn iter(&self) -> VebTreeIter<'_> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (iter)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement iter");
}

    // A VEB tree is empty if the min is greater than the max.
    pub fn empty(&self) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (empty)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement empty");
}

    // Returns true if value is in the tree, false otherwise.
    pub fn search(&self, value: u32) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (search)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement search");
}

    fn insert_empty(&mut self, value: u32) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (insert_empty)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement insert_empty");
}

    // Inserts value into the tree.
    pub fn insert(&mut self, mut value: u32) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (insert)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement insert");
}

    // Returns the next greatest value(successor) in the tree after pred. Returns
    // `None` if there is no successor.
    pub fn succ(&self, pred: u32) -> Option<u32> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (succ)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement succ");
}

    // Returns the next smallest value(predecessor) in the tree after succ. Returns
    // `None` if there is no predecessor. pred() is almost a mirror of succ().
    // Differences are noted in comments.
    pub fn pred(&self, succ: u32) -> Option<u32> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (pred)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement pred");
}
}

pub struct VebTreeIter<'a> {
    tree: &'a VebTree,
    curr: Option<u32>,
}

impl<'a> VebTreeIter<'a> {
    pub fn new(tree: &'a VebTree) -> VebTreeIter<'a> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}
}

impl Iterator for VebTreeIter<'_> {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (next)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement next");
}
}


#[cfg(test)]
mod test {
    use super::VebTree;
    use rand::{rngs::StdRng, RngExt, SeedableRng};

    fn test_veb_tree(size: u32, mut elements: Vec<u32>, exclude: Vec<u32>) {
        // Insert elements
        let mut tree = VebTree::new(size);
        for element in elements.iter() {
            tree.insert(*element);
        }

        // Test search
        for element in elements.iter() {
            assert!(tree.search(*element));
        }
        for element in exclude {
            assert!(!tree.search(element));
        }

        // Test iterator and successor, and predecessor
        elements.sort();
        elements.dedup();
        for (i, element) in tree.iter().enumerate() {
            assert_eq!(elements[i], element);
        }
        for i in 1..elements.len() {
            assert_eq!(tree.succ(elements[i - 1]), Some(elements[i]));
            assert_eq!(tree.pred(elements[i]), Some(elements[i - 1]));
        }
    }

    #[test]
    fn test_empty() {
        test_veb_tree(16, Vec::new(), (0..16).collect());
    }

    #[test]
    fn test_single() {
        test_veb_tree(16, Vec::from([5]), (0..16).filter(|x| *x != 5).collect());
    }

    #[test]
    fn test_two() {
        test_veb_tree(
            16,
            Vec::from([4, 9]),
            (0..16).filter(|x| *x != 4 && *x != 9).collect(),
        );
    }

    #[test]
    fn test_repeat_insert() {
        let mut tree = VebTree::new(16);
        for _ in 0..5 {
            tree.insert(10);
        }
        assert!(tree.search(10));
        let elements: Vec<u32> = (0..16).filter(|x| *x != 10).collect();
        for element in elements {
            assert!(!tree.search(element));
        }
    }

    #[test]
    fn test_linear() {
        test_veb_tree(16, (0..10).collect(), (10..16).collect());
    }

    fn test_full(size: u32) {
        test_veb_tree(size, (0..size).collect(), Vec::new());
    }

    #[test]
    fn test_full_small() {
        test_full(8);
        test_full(10);
        test_full(16);
        test_full(20);
        test_full(32);
    }

    #[test]
    fn test_full_256() {
        test_full(256);
    }

    #[test]
    fn test_10_256() {
        let mut rng = StdRng::seed_from_u64(0);
        let elements: Vec<u32> = (0..10).map(|_| rng.random_range(0..255)).collect();
        test_veb_tree(256, elements, Vec::new());
    }

    #[test]
    fn test_100_256() {
        let mut rng = StdRng::seed_from_u64(0);
        let elements: Vec<u32> = (0..100).map(|_| rng.random_range(0..255)).collect();
        test_veb_tree(256, elements, Vec::new());
    }

    #[test]
    fn test_100_300() {
        let mut rng = StdRng::seed_from_u64(0);
        let elements: Vec<u32> = (0..100).map(|_| rng.random_range(0..255)).collect();
        test_veb_tree(300, elements, Vec::new());
    }
}
