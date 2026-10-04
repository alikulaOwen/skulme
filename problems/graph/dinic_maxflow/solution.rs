use std::collections::VecDeque;
use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

// We assume that graph vertices are numbered from 1 to n.

/// Adjacency matrix
type Graph = Vec<Vec<usize>>;

/// We assume that T::default() gives "zero" flow and T supports negative values
pub struct FlowEdge<T> {
    pub sink: usize,
    pub capacity: T,
    pub flow: T,
}

pub struct FlowResultEdge<T> {
    pub source: usize,
    pub sink: usize,
    pub flow: T,
}

impl<T: Clone + Copy + Add + AddAssign + Sub<Output = T> + SubAssign + Ord + Neg + Default>
    FlowEdge<T>
{
    pub fn new(sink: usize, capacity: T) -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}
}

pub struct DinicMaxFlow<T> {
    /// BFS Level of each vertex. starts from 1
    level: Vec<usize>,

    /// The index of the last visited edge connected to each vertex
    pub last_edge: Vec<usize>,

    /// Holds wether the solution has already been calculated
    network_solved: bool,

    pub source: usize,
    pub sink: usize,

    /// Number of edges added to the residual network
    pub num_edges: usize,
    pub num_vertices: usize,

    pub adj: Graph,

    /// The list of flow edges
    pub edges: Vec<FlowEdge<T>>,
}

impl<T: Clone + Copy + Add + AddAssign + Sub<Output = T> + SubAssign + Neg + Ord + Default>
    DinicMaxFlow<T>
{
    pub fn new(source: usize, sink: usize, num_vertices: usize) -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}
    #[inline]
    pub fn add_edge(&mut self, source: usize, sink: usize, capacity: T) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (add_edge)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement add_edge");
}

    fn bfs(&mut self) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (bfs)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement bfs");
}

    fn dfs(&mut self, v: usize, pushed: T) -> T {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (dfs)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement dfs");
}

    pub fn find_maxflow(&mut self, infinite_flow: T) -> T {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (find_maxflow)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement find_maxflow");
}

    pub fn get_flow_edges(&mut self, infinite_flow: T) -> Vec<FlowResultEdge<T>> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (get_flow_edges)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement get_flow_edges");
}
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn small_graph() {
        let mut flow: DinicMaxFlow<i32> = DinicMaxFlow::new(1, 6, 6);
        flow.add_edge(1, 2, 16);
        flow.add_edge(1, 4, 13);
        flow.add_edge(2, 3, 12);
        flow.add_edge(3, 4, 9);
        flow.add_edge(3, 6, 20);
        flow.add_edge(4, 2, 4);
        flow.add_edge(4, 5, 14);
        flow.add_edge(5, 3, 7);
        flow.add_edge(5, 6, 4);

        let max_flow = flow.find_maxflow(i32::MAX);
        assert_eq!(max_flow, 23);

        let mut sm_out = [0; 7];
        let mut sm_in = [0; 7];

        let flow_edges = flow.get_flow_edges(i32::MAX);
        for e in flow_edges {
            sm_out[e.source] += e.flow;
            sm_in[e.sink] += e.flow;
        }
        for i in 2..=5 {
            assert_eq!(sm_in[i], sm_out[i]);
        }
        assert_eq!(sm_in[1], 0);
        assert_eq!(sm_out[1], max_flow);
        assert_eq!(sm_in[6], max_flow);
        assert_eq!(sm_out[6], 0);
    }
}
