// Kosaraju algorithm, a linear-time algorithm to find the strongly connected components (SCCs) of a directed graph, in Rust.
pub struct Graph {
    vertices: usize,
    adj_list: Vec<Vec<usize>>,
    transpose_adj_list: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new(vertices: usize) -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}

    pub fn add_edge(&mut self, u: usize, v: usize) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (add_edge)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement add_edge");
}

    pub fn dfs(&self, node: usize, visited: &mut Vec<bool>, stack: &mut Vec<usize>) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (dfs)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement dfs");
}

    pub fn dfs_scc(&self, node: usize, visited: &mut Vec<bool>, scc: &mut Vec<usize>) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (dfs_scc)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement dfs_scc");
}
}

pub fn kosaraju(graph: &Graph) -> Vec<Vec<usize>> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (kosaraju)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement kosaraju");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kosaraju_single_sccs() {
        let vertices = 5;
        let mut graph = Graph::new(vertices);

        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(2, 4);
        graph.add_edge(3, 0);
        graph.add_edge(4, 2);

        let sccs = kosaraju(&graph);
        assert_eq!(sccs.len(), 1);
        assert!(sccs.contains(&vec![0, 3, 2, 1, 4]));
    }

    #[test]
    fn test_kosaraju_multiple_sccs() {
        let vertices = 8;
        let mut graph = Graph::new(vertices);

        graph.add_edge(1, 0);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);
        graph.add_edge(4, 5);
        graph.add_edge(5, 6);
        graph.add_edge(6, 7);
        graph.add_edge(4, 7);
        graph.add_edge(6, 4);

        let sccs = kosaraju(&graph);
        assert_eq!(sccs.len(), 4);
        assert!(sccs.contains(&vec![0, 1, 2]));
        assert!(sccs.contains(&vec![3]));
        assert!(sccs.contains(&vec![4, 6, 5]));
        assert!(sccs.contains(&vec![7]));
    }

    #[test]
    fn test_kosaraju_multiple_sccs1() {
        let vertices = 8;
        let mut graph = Graph::new(vertices);
        graph.add_edge(0, 2);
        graph.add_edge(1, 0);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);
        graph.add_edge(4, 7);
        graph.add_edge(5, 2);
        graph.add_edge(5, 6);
        graph.add_edge(6, 5);
        graph.add_edge(7, 6);

        let sccs = kosaraju(&graph);
        assert_eq!(sccs.len(), 3);
        assert!(sccs.contains(&vec![0]));
        assert!(sccs.contains(&vec![1]));
        assert!(sccs.contains(&vec![2, 5, 6, 7, 4, 3]));
    }

    #[test]
    fn test_kosaraju_no_scc() {
        let vertices = 4;
        let mut graph = Graph::new(vertices);

        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);

        let sccs = kosaraju(&graph);
        assert_eq!(sccs.len(), 4);
        for (i, _) in sccs.iter().enumerate().take(vertices) {
            assert_eq!(sccs[i], vec![i]);
        }
    }

    #[test]
    fn test_kosaraju_empty_graph() {
        let vertices = 0;
        let graph = Graph::new(vertices);

        let sccs = kosaraju(&graph);
        assert_eq!(sccs.len(), 0);
    }
}
