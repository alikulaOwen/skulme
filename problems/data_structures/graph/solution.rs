use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone)]
pub struct NodeNotInGraph;

impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (fmt)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement fmt");
}
}

pub struct DirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}

impl Graph for DirectedGraph {
    fn new() -> DirectedGraph {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (adjacency_table_mutable)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement adjacency_table_mutable");
}
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (adjacency_table)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement adjacency_table");
}
}

pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}

impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (adjacency_table_mutable)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement adjacency_table_mutable");
}
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (adjacency_table)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement adjacency_table");
}
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (add_edge)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement add_edge");
}
}

pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;

    fn add_node(&mut self, node: &str) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (add_node)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement add_node");
}

    fn add_edge(&mut self, edge: (&str, &str, i32)) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (add_edge)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement add_edge");
}

    fn neighbours(&self, node: &str) -> Result<&Vec<(String, i32)>, NodeNotInGraph> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (neighbours)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement neighbours");
}

    fn contains(&self, node: &str) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (contains)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement contains");
}

    fn nodes(&self) -> HashSet<&String> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (nodes)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement nodes");
}

    fn edges(&self) -> Vec<(&String, &String, i32)> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (edges)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement edges");
}
}


#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::UndirectedGraph;
    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();

        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));

        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
        ];
        for edge in expected_edges.iter() {
            assert!(graph.edges().contains(edge));
        }
    }

    #[test]
    fn test_neighbours() {
        let mut graph = UndirectedGraph::new();

        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));

        assert_eq!(
            graph.neighbours("a").unwrap(),
            &vec![(String::from("b"), 5), (String::from("c"), 7)]
        );
    }
}

#[cfg(test)]
mod test_directed_graph {
    use super::DirectedGraph;
    use super::Graph;

    #[test]
    fn test_add_node() {
        let mut graph = DirectedGraph::new();
        graph.add_node("a");
        graph.add_node("b");
        graph.add_node("c");
        assert_eq!(
            graph.nodes(),
            [&String::from("a"), &String::from("b"), &String::from("c")]
                .iter()
                .cloned()
                .collect()
        );
    }

    #[test]
    fn test_add_edge() {
        let mut graph = DirectedGraph::new();

        graph.add_edge(("a", "b", 5));
        graph.add_edge(("c", "a", 7));
        graph.add_edge(("b", "c", 10));

        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("b"), &String::from("c"), 10),
        ];
        for edge in expected_edges.iter() {
            assert!(graph.edges().contains(edge));
        }
    }

    #[test]
    fn test_neighbours() {
        let mut graph = DirectedGraph::new();

        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));

        assert_eq!(
            graph.neighbours("a").unwrap(),
            &vec![(String::from("b"), 5)]
        );
    }

    #[test]
    fn test_contains() {
        let mut graph = DirectedGraph::new();
        graph.add_node("a");
        graph.add_node("b");
        graph.add_node("c");
        assert!(graph.contains("a"));
        assert!(graph.contains("b"));
        assert!(graph.contains("c"));
        assert!(!graph.contains("d"));
    }
}
