// Adjacency List
use std::collections::VecDeque;
type Graph = Vec<Vec<usize>>;

pub struct BipartiteMatching {
    pub adj: Graph,
    pub num_vertices_grp1: usize,
    pub num_vertices_grp2: usize,
    // mt1[i] = v is the matching of i in grp1 to v in grp2
    pub mt1: Vec<i32>,
    pub mt2: Vec<i32>,
    pub used: Vec<bool>,
}
impl BipartiteMatching {
    pub fn new(num_vertices_grp1: usize, num_vertices_grp2: usize) -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}
    #[inline]
    // Add an directed edge u->v in the graph
    pub fn add_edge(&mut self, u: usize, v: usize) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (add_edge)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement add_edge");
}

    fn try_kuhn(&mut self, cur: usize) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (try_kuhn)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement try_kuhn");
}
    // Note: It does not modify self.mt1, it only works on self.mt2
    pub fn kuhn(&mut self) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (kuhn)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement kuhn");
}
    pub fn print_matching(&self) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (print_matching)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement print_matching");
}
    fn bfs(&self, dist: &mut [i32]) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (bfs)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement bfs");
}
    fn dfs(&mut self, u: i32, dist: &mut Vec<i32>) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (dfs)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement dfs");
}
    pub fn hopcroft_karp(&mut self) -> i32 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (hopcroft_karp)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement hopcroft_karp");
}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn small_graph_kuhn() {
        let n1 = 6;
        let n2 = 6;
        let mut g = BipartiteMatching::new(n1, n2);
        // vertex 1 in grp1 to vertex 1 in grp 2
        // denote the ith grp2 vertex as n1+i
        g.add_edge(1, 2);
        g.add_edge(1, 3);
        // 2 is not connected to any vertex
        g.add_edge(3, 4);
        g.add_edge(3, 1);
        g.add_edge(4, 3);
        g.add_edge(5, 3);
        g.add_edge(5, 4);
        g.add_edge(6, 6);
        g.kuhn();
        g.print_matching();
        let answer: Vec<i32> = vec![-1, 2, -1, 1, 3, 4, 6];
        for i in 1..g.mt2.len() {
            if g.mt2[i] == -1 {
                // 5 in group2 has no pair
                assert_eq!(i, 5);
                continue;
            }
            // 2 in group1 has no pair
            assert!(g.mt2[i] != 2);
            assert_eq!(i as i32, answer[g.mt2[i] as usize]);
        }
    }
    #[test]
    fn small_graph_hopcroft() {
        let n1 = 6;
        let n2 = 6;
        let mut g = BipartiteMatching::new(n1, n2);
        // vertex 1 in grp1 to vertex 1 in grp 2
        // denote the ith grp2 vertex as n1+i
        g.add_edge(1, 2);
        g.add_edge(1, 3);
        // 2 is not connected to any vertex
        g.add_edge(3, 4);
        g.add_edge(3, 1);
        g.add_edge(4, 3);
        g.add_edge(5, 3);
        g.add_edge(5, 4);
        g.add_edge(6, 6);
        let x = g.hopcroft_karp();
        assert_eq!(x, 5);
        g.print_matching();
        let answer: Vec<i32> = vec![-1, 2, -1, 1, 3, 4, 6];
        for i in 1..g.mt2.len() {
            if g.mt2[i] == -1 {
                // 5 in group2 has no pair
                assert_eq!(i, 5);
                continue;
            }
            // 2 in group1 has no pair
            assert!(g.mt2[i] != 2);
            assert_eq!(i as i32, answer[g.mt2[i] as usize]);
        }
    }
    #[test]
    fn super_small_graph_kuhn() {
        let n1 = 1;
        let n2 = 1;
        let mut g = BipartiteMatching::new(n1, n2);
        g.add_edge(1, 1);
        g.kuhn();
        g.print_matching();
        assert_eq!(g.mt2[1], 1);
    }
    #[test]
    fn super_small_graph_hopcroft() {
        let n1 = 1;
        let n2 = 1;
        let mut g = BipartiteMatching::new(n1, n2);
        g.add_edge(1, 1);
        let x = g.hopcroft_karp();
        assert_eq!(x, 1);
        g.print_matching();
        assert_eq!(g.mt2[1], 1);
        assert_eq!(g.mt1[1], 1);
    }

    #[test]
    fn only_one_vertex_graph_kuhn() {
        let n1 = 10;
        let n2 = 10;
        let mut g = BipartiteMatching::new(n1, n2);
        g.add_edge(1, 1);
        g.add_edge(2, 1);
        g.add_edge(3, 1);
        g.add_edge(4, 1);
        g.add_edge(5, 1);
        g.add_edge(6, 1);
        g.add_edge(7, 1);
        g.add_edge(8, 1);
        g.add_edge(9, 1);
        g.add_edge(10, 1);
        g.kuhn();
        g.print_matching();
        assert_eq!(g.mt2[1], 1);
        for i in 2..g.mt2.len() {
            assert_eq!(g.mt2[i], -1);
        }
    }
    #[test]
    fn only_one_vertex_graph_hopcroft() {
        let n1 = 10;
        let n2 = 10;
        let mut g = BipartiteMatching::new(n1, n2);
        g.add_edge(1, 1);
        g.add_edge(2, 1);
        g.add_edge(3, 1);
        g.add_edge(4, 1);
        g.add_edge(5, 1);
        g.add_edge(6, 1);
        g.add_edge(7, 1);
        g.add_edge(8, 1);
        g.add_edge(9, 1);
        g.add_edge(10, 1);
        let x = g.hopcroft_karp();
        assert_eq!(x, 1);
        g.print_matching();
        assert_eq!(g.mt2[1], 1);
        for i in 2..g.mt2.len() {
            assert_eq!(g.mt2[i], -1);
        }
    }
}
