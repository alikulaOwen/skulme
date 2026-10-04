use std::collections::{HashMap, HashSet, VecDeque};

use crate::data_structures::{graph::Graph, DirectedGraph, UndirectedGraph};

pub trait DetectCycle {
    fn detect_cycle_dfs(&self) -> bool;
    fn detect_cycle_bfs(&self) -> bool;
}

// Helper function to detect cycle in an undirected graph using DFS graph traversal
fn undirected_graph_detect_cycle_dfs<'a>(
    graph: &'a UndirectedGraph,
    visited_node: &mut HashSet<&'a String>,
    parent: Option<&'a String>,
    u: &'a String,
) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (undirected_graph_detect_cycle_dfs)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement undirected_graph_detect_cycle_dfs");
}

// Helper function to detect cycle in an undirected graph using BFS graph traversal
fn undirected_graph_detect_cycle_bfs<'a>(
    graph: &'a UndirectedGraph,
    visited_node: &mut HashSet<&'a String>,
    u: &'a String,
) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (undirected_graph_detect_cycle_bfs)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement undirected_graph_detect_cycle_bfs");
}

impl DetectCycle for UndirectedGraph {
    fn detect_cycle_dfs(&self) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (detect_cycle_dfs)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement detect_cycle_dfs");
}

    fn detect_cycle_bfs(&self) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (detect_cycle_bfs)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement detect_cycle_bfs");
}
}

// Helper function to detect cycle in a directed graph using DFS graph traversal
fn directed_graph_detect_cycle_dfs<'a>(
    graph: &'a DirectedGraph,
    visited_node: &mut HashSet<&'a String>,
    in_stack_visited_node: &mut HashSet<&'a String>,
    u: &'a String,
) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (directed_graph_detect_cycle_dfs)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement directed_graph_detect_cycle_dfs");
}

impl DetectCycle for DirectedGraph {
    fn detect_cycle_dfs(&self) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (detect_cycle_dfs)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement detect_cycle_dfs");
}

    // detect cycle in a the graph using Kahn's algorithm
    // https://www.geeksforgeeks.org/detect-cycle-in-a-directed-graph-using-bfs/
    fn detect_cycle_bfs(&self) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (detect_cycle_bfs)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement detect_cycle_bfs");
}
}


#[cfg(test)]
mod test {
    use super::DetectCycle;
    use crate::data_structures::{graph::Graph, DirectedGraph, UndirectedGraph};
    fn get_undirected_single_node_with_loop() -> UndirectedGraph {
        let mut res = UndirectedGraph::new();
        res.add_edge(("a", "a", 1));
        res
    }
    fn get_directed_single_node_with_loop() -> DirectedGraph {
        let mut res = DirectedGraph::new();
        res.add_edge(("a", "a", 1));
        res
    }
    fn get_undirected_two_nodes_connected() -> UndirectedGraph {
        let mut res = UndirectedGraph::new();
        res.add_edge(("a", "b", 1));
        res
    }
    fn get_directed_two_nodes_connected() -> DirectedGraph {
        let mut res = DirectedGraph::new();
        res.add_edge(("a", "b", 1));
        res.add_edge(("b", "a", 1));
        res
    }
    fn get_directed_two_nodes() -> DirectedGraph {
        let mut res = DirectedGraph::new();
        res.add_edge(("a", "b", 1));
        res
    }
    fn get_undirected_triangle() -> UndirectedGraph {
        let mut res = UndirectedGraph::new();
        res.add_edge(("a", "b", 1));
        res.add_edge(("b", "c", 1));
        res.add_edge(("c", "a", 1));
        res
    }
    fn get_directed_triangle() -> DirectedGraph {
        let mut res = DirectedGraph::new();
        res.add_edge(("a", "b", 1));
        res.add_edge(("b", "c", 1));
        res.add_edge(("c", "a", 1));
        res
    }
    fn get_undirected_triangle_with_tail() -> UndirectedGraph {
        let mut res = get_undirected_triangle();
        res.add_edge(("c", "d", 1));
        res.add_edge(("d", "e", 1));
        res.add_edge(("e", "f", 1));
        res.add_edge(("g", "h", 1));
        res
    }
    fn get_directed_triangle_with_tail() -> DirectedGraph {
        let mut res = get_directed_triangle();
        res.add_edge(("c", "d", 1));
        res.add_edge(("d", "e", 1));
        res.add_edge(("e", "f", 1));
        res.add_edge(("g", "h", 1));
        res
    }
    fn get_undirected_graph_with_cycle() -> UndirectedGraph {
        let mut res = UndirectedGraph::new();
        res.add_edge(("a", "b", 1));
        res.add_edge(("a", "c", 1));
        res.add_edge(("b", "c", 1));
        res.add_edge(("b", "d", 1));
        res.add_edge(("c", "d", 1));
        res
    }
    fn get_undirected_graph_without_cycle() -> UndirectedGraph {
        let mut res = UndirectedGraph::new();
        res.add_edge(("a", "b", 1));
        res.add_edge(("a", "c", 1));
        res.add_edge(("b", "d", 1));
        res.add_edge(("c", "e", 1));
        res
    }
    fn get_directed_graph_with_cycle() -> DirectedGraph {
        let mut res = DirectedGraph::new();
        res.add_edge(("b", "a", 1));
        res.add_edge(("c", "a", 1));
        res.add_edge(("b", "c", 1));
        res.add_edge(("c", "d", 1));
        res.add_edge(("d", "b", 1));
        res
    }
    fn get_directed_graph_without_cycle() -> DirectedGraph {
        let mut res = DirectedGraph::new();
        res.add_edge(("b", "a", 1));
        res.add_edge(("c", "a", 1));
        res.add_edge(("b", "c", 1));
        res.add_edge(("c", "d", 1));
        res.add_edge(("b", "d", 1));
        res
    }
    macro_rules! test_detect_cycle {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (graph, has_cycle) = $test_case;
                    println!("detect_cycle_dfs: {}", graph.detect_cycle_dfs());
                    println!("detect_cycle_bfs: {}", graph.detect_cycle_bfs());
                    assert_eq!(graph.detect_cycle_dfs(), has_cycle);
                    assert_eq!(graph.detect_cycle_bfs(), has_cycle);
                }
            )*
        };
    }
    test_detect_cycle! {
        undirected_empty: (UndirectedGraph::new(), false),
        directed_empty: (DirectedGraph::new(), false),
        undirected_single_node_with_loop: (get_undirected_single_node_with_loop(), true),
        directed_single_node_with_loop: (get_directed_single_node_with_loop(), true),
        undirected_two_nodes_connected: (get_undirected_two_nodes_connected(), false),
        directed_two_nodes_connected: (get_directed_two_nodes_connected(), true),
        directed_two_nodes: (get_directed_two_nodes(), false),
        undirected_triangle: (get_undirected_triangle(), true),
        undirected_triangle_with_tail: (get_undirected_triangle_with_tail(), true),
        directed_triangle: (get_directed_triangle(), true),
        directed_triangle_with_tail: (get_directed_triangle_with_tail(), true),
        undirected_graph_with_cycle: (get_undirected_graph_with_cycle(), true),
        undirected_graph_without_cycle: (get_undirected_graph_without_cycle(), false),
        directed_graph_with_cycle: (get_directed_graph_with_cycle(), true),
        directed_graph_without_cycle: (get_directed_graph_without_cycle(), false),
    }
}
