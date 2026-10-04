/*
Tarjan's algorithm to find Strongly Connected Components (SCCs):
It runs in O(n + m) (so it is optimal) and as a by-product, it returns the
components in some (reverse) topologically sorted order.

We assume that graph is represented using (compressed) adjacency matrix
and its vertices are numbered from 1 to n. If this is not the case, one
can use `src/graph/graph_enumeration.rs` to convert their graph.
*/

pub struct StronglyConnectedComponents {
    // The number of the SCC the vertex is in, starting from 1
    pub component: Vec<usize>,

    // The discover time of the vertex with minimum discover time reachable
    // from this vertex. The MSB of the numbers are used to save whether the
    // vertex has been visited (but the MSBs are cleared after
    // the algorithm is done)
    pub state: Vec<u64>,

    // The total number of SCCs
    pub num_components: usize,

    // The stack of vertices that DFS has seen (used internally)
    stack: Vec<usize>,
    // Used internally during DFS to know the current discover time
    current_time: usize,
}

// Some functions to help with DRY and code readability
const NOT_DONE: u64 = 1 << 63;

#[inline]
fn set_done(vertex_state: &mut u64) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (set_done)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement set_done");
}

#[inline]
fn is_in_stack(vertex_state: u64) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (is_in_stack)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement is_in_stack");
}

#[inline]
fn is_unvisited(vertex_state: u64) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (is_unvisited)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement is_unvisited");
}

#[inline]
fn get_discover_time(vertex_state: u64) -> u64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (get_discover_time)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement get_discover_time");
}

impl StronglyConnectedComponents {
    pub fn new(mut num_vertices: usize) -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}
    fn dfs(&mut self, v: usize, adj: &[Vec<usize>]) -> u64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (dfs)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement dfs");
}
    pub fn find_components(&mut self, adj: &[Vec<usize>]) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (find_components)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement find_components");
}
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acyclic() {
        let mut sccs = StronglyConnectedComponents::new(5);
        let adj = vec![vec![], vec![2, 4], vec![3, 4], vec![5], vec![5], vec![]];
        sccs.find_components(&adj);
        assert_eq!(sccs.component, vec![0, 5, 4, 2, 3, 1]);
        assert_eq!(sccs.state, vec![0, 1, 2, 3, 5, 4]);
        assert_eq!(sccs.num_components, 5);
    }

    #[test]
    fn cycle() {
        let mut sccs = StronglyConnectedComponents::new(4);
        let adj = vec![vec![], vec![2], vec![3], vec![4], vec![1]];
        sccs.find_components(&adj);
        assert_eq!(sccs.component, vec![0, 1, 1, 1, 1]);
        assert_eq!(sccs.state, vec![0, 1, 2, 3, 4]);
        assert_eq!(sccs.num_components, 1);
    }

    #[test]
    fn dumbbell() {
        let mut sccs = StronglyConnectedComponents::new(6);
        let adj = vec![
            vec![],
            vec![2],
            vec![3, 4],
            vec![1],
            vec![5],
            vec![6],
            vec![4],
        ];
        sccs.find_components(&adj);
        assert_eq!(sccs.component, vec![0, 2, 2, 2, 1, 1, 1]);
        assert_eq!(sccs.state, vec![0, 1, 2, 3, 4, 5, 6]);
        assert_eq!(sccs.num_components, 2);
    }

    #[test]
    fn connected_dumbbell() {
        let mut sccs = StronglyConnectedComponents::new(6);
        let adj = vec![
            vec![],
            vec![2],
            vec![3, 4],
            vec![1],
            vec![5, 1],
            vec![6],
            vec![4],
        ];
        sccs.find_components(&adj);
        assert_eq!(sccs.component, vec![0, 1, 1, 1, 1, 1, 1]);
        assert_eq!(sccs.state, vec![0, 1, 2, 3, 4, 5, 6]);
        assert_eq!(sccs.num_components, 1);
    }
}
