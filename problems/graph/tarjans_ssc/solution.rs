pub struct Graph {
    n: usize,
    adj_list: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new(n: usize) -> Self {
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
}
pub fn tarjan_scc(graph: &Graph) -> Vec<Vec<usize>> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (tarjan_scc)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement tarjan_scc");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tarjan_scc() {
        // Test 1: A graph with multiple strongly connected components
        let n_vertices = 11;
        let edges = vec![
            (0, 1),
            (0, 3),
            (1, 2),
            (1, 4),
            (2, 0),
            (2, 6),
            (3, 2),
            (4, 5),
            (4, 6),
            (5, 6),
            (5, 7),
            (5, 8),
            (5, 9),
            (6, 4),
            (7, 9),
            (8, 9),
            (9, 8),
        ];
        let mut graph = Graph::new(n_vertices);

        for &(u, v) in &edges {
            graph.add_edge(u, v);
        }

        let components = tarjan_scc(&graph);
        assert_eq!(
            components,
            vec![
                vec![8, 9],
                vec![7],
                vec![5, 4, 6],
                vec![3, 2, 1, 0],
                vec![10],
            ]
        );

        // Test 2: A graph with no edges
        let n_vertices = 5;
        let edges: Vec<(usize, usize)> = vec![];
        let mut graph = Graph::new(n_vertices);

        for &(u, v) in &edges {
            graph.add_edge(u, v);
        }

        let components = tarjan_scc(&graph);

        // Each node is its own SCC
        assert_eq!(
            components,
            vec![vec![0], vec![1], vec![2], vec![3], vec![4]]
        );

        // Test 3: A graph with single strongly connected component
        let n_vertices = 5;
        let edges = vec![(0, 1), (1, 2), (2, 3), (2, 4), (3, 0), (4, 2)];
        let mut graph = Graph::new(n_vertices);

        for &(u, v) in &edges {
            graph.add_edge(u, v);
        }

        let components = tarjan_scc(&graph);
        assert_eq!(components, vec![vec![4, 3, 2, 1, 0]]);

        // Test 4: A graph with multiple strongly connected component
        let n_vertices = 7;
        let edges = vec![
            (0, 1),
            (1, 2),
            (2, 0),
            (1, 3),
            (1, 4),
            (1, 6),
            (3, 5),
            (4, 5),
        ];
        let mut graph = Graph::new(n_vertices);

        for &(u, v) in &edges {
            graph.add_edge(u, v);
        }

        let components = tarjan_scc(&graph);
        assert_eq!(
            components,
            vec![vec![5], vec![3], vec![4], vec![6], vec![2, 1, 0],]
        );
    }
}
