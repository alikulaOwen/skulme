use std::collections::HashMap;
use std::collections::VecDeque;
use std::hash::Hash;

#[derive(Debug, Eq, PartialEq)]
pub enum TopoligicalSortError {
    CycleDetected,
}

type TopologicalSortResult<Node> = Result<Vec<Node>, TopoligicalSortError>;

/// Given a directed graph, modeled as a list of edges from source to destination
/// Uses Kahn's algorithm to either:
///     return the topological sort of the graph
///     or detect if there's any cycle
pub fn topological_sort<Node: Hash + Eq + Copy>(
    edges: &Vec<(Node, Node)>,
) -> TopologicalSortResult<Node> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (topological_sort)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement topological_sort");
}


#[cfg(test)]
mod tests {
    use super::topological_sort;
    use crate::graph::topological_sort::TopoligicalSortError;

    fn is_valid_sort<Node: Eq>(sorted: &[Node], graph: &[(Node, Node)]) -> bool {
        for (source, dest) in graph {
            let source_pos = sorted.iter().position(|node| node == source);
            let dest_pos = sorted.iter().position(|node| node == dest);
            match (source_pos, dest_pos) {
                (Some(src), Some(dst)) if src < dst => {}
                _ => {
                    return false;
                }
            };
        }
        true
    }

    #[test]
    fn it_works() {
        let graph = vec![(1, 2), (1, 3), (2, 3), (3, 4), (4, 5), (5, 6), (6, 7)];
        let sort = topological_sort(&graph);
        assert!(sort.is_ok());
        let sort = sort.unwrap();
        assert!(is_valid_sort(&sort, &graph));
        assert_eq!(sort, vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_wikipedia_example() {
        let graph = vec![
            (5, 11),
            (7, 11),
            (7, 8),
            (3, 8),
            (3, 10),
            (11, 2),
            (11, 9),
            (11, 10),
            (8, 9),
        ];
        let sort = topological_sort(&graph);
        assert!(sort.is_ok());
        let sort = sort.unwrap();
        assert!(is_valid_sort(&sort, &graph));
    }

    #[test]
    fn test_cyclic_graph() {
        let graph = vec![(1, 2), (2, 3), (3, 4), (4, 5), (4, 2)];
        let sort = topological_sort(&graph);
        assert!(sort.is_err());
        assert_eq!(sort.err().unwrap(), TopoligicalSortError::CycleDetected);
    }
}
