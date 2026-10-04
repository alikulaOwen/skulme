use std::collections::HashSet;

/// A data-structure that, given a forest, allows dynamic-connectivity queries.
/// Meaning deletion of an edge (u,v) and checking whether two vertecies are still connected.
///
/// # Complexity
/// The preprocessing phase runs in O(n) time, where n is the number of vertecies in the forest.
/// Deletion runs in O(log n) and checking for connectivity runs in O(1) time.
///
/// # Sources
/// used Wikipedia as reference: <https://en.wikipedia.org/wiki/Dynamic_connectivity>
pub struct DecrementalConnectivity {
    adjacent: Vec<HashSet<usize>>,
    component: Vec<usize>,
    count: usize,
    visited: Vec<usize>,
    dfs_id: usize,
}
impl DecrementalConnectivity {
    //expects the parent of a root to be itself
    pub fn new(adjacent: Vec<HashSet<usize>>) -> Result<Self, String> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}

    pub fn connected(&self, u: usize, v: usize) -> Option<bool> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (connected)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement connected");
}

    pub fn delete(&mut self, u: usize, v: usize) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (delete)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement delete");
}

    fn calc_component(&mut self) -> Vec<usize> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (calc_component)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement calc_component");
}

    fn is_smaller(&mut self, u: usize, v: usize) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (is_smaller)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement is_smaller");
}

    fn dfs_step(&mut self, queue: &mut Vec<usize>, dfs_id: usize) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (dfs_step)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement dfs_step");
}
}

// checks whether the given graph is a forest
// also checks for all adjacent vertices a,b if adjacent[a].contains(b) && adjacent[b].contains(a)
fn is_forest(adjacent: &Vec<HashSet<usize>>) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (is_forest)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement is_forest");
}

fn has_cycle(
    adjacent: &Vec<HashSet<usize>>,
    visited: &mut Vec<bool>,
    node: usize,
    parent: usize,
) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (has_cycle)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement has_cycle");
}


#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    // test forest (remember the assumptoin that roots are adjacent to themselves)
    //              _              _
    //             \ /            \ /
    //              0              7
    //            / | \            |
    //           1  2  3           8
    //         /   / \
    //        4   5   6
    #[test]
    fn construction_test() {
        let mut adjacent = vec![
            HashSet::from([0, 1, 2, 3]),
            HashSet::from([0, 4]),
            HashSet::from([0, 5, 6]),
            HashSet::from([0]),
            HashSet::from([1]),
            HashSet::from([2]),
            HashSet::from([2]),
            HashSet::from([7, 8]),
            HashSet::from([7]),
        ];
        let dec_con = super::DecrementalConnectivity::new(adjacent.clone()).unwrap();
        assert_eq!(dec_con.component, vec![0, 0, 0, 0, 0, 0, 0, 1, 1]);

        // add a cycle to the tree
        adjacent[2].insert(4);
        adjacent[4].insert(2);
        assert!(super::DecrementalConnectivity::new(adjacent.clone()).is_err());
    }
    #[test]
    #[should_panic(expected = "2 -> 4 exists")]
    fn non_bidirectional_test() {
        let adjacent = vec![
            HashSet::from([0, 1, 2, 3]),
            HashSet::from([0, 4]),
            HashSet::from([0, 5, 6, 4]),
            HashSet::from([0]),
            HashSet::from([1]),
            HashSet::from([2]),
            HashSet::from([2]),
            HashSet::from([7, 8]),
            HashSet::from([7]),
        ];

        // should panic now since our graph is not bidirectional
        super::DecrementalConnectivity::new(adjacent).unwrap();
    }

    #[test]
    #[should_panic(expected = "delete called on the edge (2, 4)")]
    fn delete_panic_test() {
        let adjacent = vec![
            HashSet::from([0, 1, 2, 3]),
            HashSet::from([0, 4]),
            HashSet::from([0, 5, 6]),
            HashSet::from([0]),
            HashSet::from([1]),
            HashSet::from([2]),
            HashSet::from([2]),
            HashSet::from([7, 8]),
            HashSet::from([7]),
        ];
        let mut dec_con = super::DecrementalConnectivity::new(adjacent).unwrap();
        dec_con.delete(2, 4);
    }

    #[test]
    fn query_test() {
        let adjacent = vec![
            HashSet::from([0, 1, 2, 3]),
            HashSet::from([0, 4]),
            HashSet::from([0, 5, 6]),
            HashSet::from([0]),
            HashSet::from([1]),
            HashSet::from([2]),
            HashSet::from([2]),
            HashSet::from([7, 8]),
            HashSet::from([7]),
        ];
        let mut dec_con1 = super::DecrementalConnectivity::new(adjacent.clone()).unwrap();
        assert!(dec_con1.connected(3, 4).unwrap());
        assert!(dec_con1.connected(5, 0).unwrap());
        assert!(!dec_con1.connected(2, 7).unwrap());
        assert!(dec_con1.connected(0, 9).is_none());
        dec_con1.delete(0, 2);
        assert!(dec_con1.connected(3, 4).unwrap());
        assert!(!dec_con1.connected(5, 0).unwrap());
        assert!(dec_con1.connected(5, 6).unwrap());
        assert!(dec_con1.connected(8, 7).unwrap());
        dec_con1.delete(7, 8);
        assert!(!dec_con1.connected(8, 7).unwrap());
        dec_con1.delete(1, 4);
        assert!(!dec_con1.connected(1, 4).unwrap());

        let mut dec_con2 = super::DecrementalConnectivity::new(adjacent.clone()).unwrap();
        dec_con2.delete(4, 1);
        assert!(!dec_con2.connected(1, 4).unwrap());

        let mut dec_con3 = super::DecrementalConnectivity::new(adjacent).unwrap();
        dec_con3.delete(1, 4);
        assert!(!dec_con3.connected(4, 1).unwrap());
    }
}
