use std::collections::HashSet;
use std::collections::VecDeque;

// Perform a Depth First Search Algorithm to find a element in a graph
//
// Return a Optional with a vector with history of vertex visiteds
// or a None if the element not exists on the graph
pub fn depth_first_search(graph: &Graph, root: Vertex, objective: Vertex) -> Option<Vec<u32>> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (depth_first_search)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement depth_first_search");
}

// Data Structures

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vertex(u32);
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Edge(u32, u32);
#[derive(Clone)]
pub struct Graph {
    #[allow(dead_code)]
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
}

impl Graph {
    pub fn new(vertices: Vec<Vertex>, edges: Vec<Edge>) -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}
}

impl From<u32> for Vertex {
    fn from(item: u32) -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (from)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement from");
}
}

impl Vertex {
    pub fn value(&self) -> u32 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (value)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement value");
}

    pub fn neighbors(&self, graph: &Graph) -> VecDeque<Vertex> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (neighbors)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement neighbors");
}
}

impl From<(u32, u32)> for Edge {
    fn from(item: (u32, u32)) -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (from)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement from");
}
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_1_fail() {
        let vertices = vec![1, 2, 3, 4, 5, 6, 7];
        let edges = vec![(1, 2), (1, 3), (2, 4), (2, 5), (3, 6), (3, 7)];

        let root = 1;
        let objective = 99;

        let graph = Graph::new(
            vertices.into_iter().map(|v| v.into()).collect(),
            edges.into_iter().map(|e| e.into()).collect(),
        );

        assert_eq!(
            depth_first_search(&graph, root.into(), objective.into()),
            None
        );
    }

    #[test]
    fn find_1_sucess() {
        let vertices = vec![1, 2, 3, 4, 5, 6, 7];
        let edges = vec![(1, 2), (1, 3), (2, 4), (2, 5), (3, 6), (3, 7)];

        let root = 1;
        let objective = 7;

        let correct_path = vec![1, 2, 4, 5, 3, 6, 7];

        let graph = Graph::new(
            vertices.into_iter().map(|v| v.into()).collect(),
            edges.into_iter().map(|e| e.into()).collect(),
        );

        assert_eq!(
            depth_first_search(&graph, root.into(), objective.into()),
            Some(correct_path)
        );
    }

    #[test]
    fn find_2_sucess() {
        let vertices = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let edges = vec![
            (0, 1),
            (1, 3),
            (3, 2),
            (2, 1),
            (3, 4),
            (4, 5),
            (5, 7),
            (7, 6),
            (6, 4),
        ];

        let root = 0;
        let objective = 6;

        let correct_path = vec![0, 1, 3, 2, 4, 5, 7, 6];

        let graph = Graph::new(
            vertices.into_iter().map(|v| v.into()).collect(),
            edges.into_iter().map(|e| e.into()).collect(),
        );

        assert_eq!(
            depth_first_search(&graph, root.into(), objective.into()),
            Some(correct_path)
        );
    }

    #[test]
    fn find_3_sucess() {
        let vertices = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let edges = vec![
            (0, 1),
            (1, 3),
            (3, 2),
            (2, 1),
            (3, 4),
            (4, 5),
            (5, 7),
            (7, 6),
            (6, 4),
        ];

        let root = 0;
        let objective = 4;

        let correct_path = vec![0, 1, 3, 2, 4];

        let graph = Graph::new(
            vertices.into_iter().map(|v| v.into()).collect(),
            edges.into_iter().map(|e| e.into()).collect(),
        );

        assert_eq!(
            depth_first_search(&graph, root.into(), objective.into()),
            Some(correct_path)
        );
    }
}
