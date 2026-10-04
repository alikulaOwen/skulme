/*
 Note: We will assume that here tree vertices are numbered from 1 to n.
If a tree is not enumerated that way or its vertices are not represented
using numbers, it can trivially be converted using Depth First Search
manually or by using `src/graph/graph_enumeration.rs`

 Here we implement two different algorithms:
- The online one is implemented using Sparse Table and has O(n.lg(n))
time complexity and memory usage. It answers each query in O(lg(n)).
- The offline algorithm was discovered by Robert Tarjan. At first each
query should be determined and saved. Then, vertices are visited in
Depth First Search order and queries are answered using Disjoint
Set Union algorithm. The time complexity is O(n.alpha(n) + q) and
memory usage is O(n + q), but time complexity can be considered to be O(n + q),
because alpha(n) < 5 for n < 10 ^ 600
 */

use super::DisjointSetUnion;
pub struct LowestCommonAncestorOnline {
    // Make members public to allow the user to fill them themself.
    pub parents_sparse_table: Vec<Vec<usize>>,
    pub height: Vec<usize>,
}

impl LowestCommonAncestorOnline {
    // Should be called once as:
    // fill_sparse_table(tree_root, 0, 0, adjacency_list)
    #[inline]
    fn get_parent(&self, v: usize, i: usize) -> usize {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (get_parent)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement get_parent");
}
    #[inline]
    fn num_parents(&self, v: usize) -> usize {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (num_parents)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement num_parents");
}
    pub fn new(num_vertices: usize) -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}
    pub fn fill_sparse_table(
        &mut self,
        vertex: usize,
        parent: usize,
        height: usize,
        adj: &[Vec<usize>],
    ) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (fill_sparse_table)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement fill_sparse_table");
}

    pub fn get_ancestor(&self, mut v: usize, mut u: usize) -> usize {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (get_ancestor)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement get_ancestor");
}
}

#[derive(Clone, Copy)]
pub struct LCAQuery {
    other: usize,
    query_id: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct QueryAnswer {
    query_id: usize,
    answer: usize,
}

pub struct LowestCommonAncestorOffline {
    pub queries: Vec<Vec<LCAQuery>>,
    dsu: DisjointSetUnion,
    /*
    The LSB of dsu_parent[v] determines whether it was visited or not.
    The rest of the number determines the vertex that represents a
    particular set in DSU.
    */
    dsu_parent: Vec<u64>,
}

impl LowestCommonAncestorOffline {
    pub fn new(num_vertices: usize) -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}
    pub fn add_query(&mut self, u: usize, v: usize, query_id: usize) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (add_query)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement add_query");
}

    fn calculate_answers(
        &mut self,
        vertex: usize,
        parent: usize,
        adj: &[Vec<usize>],
        answers: &mut Vec<QueryAnswer>,
    ) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (calculate_answers)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement calculate_answers");
}
    pub fn answer_queries(&mut self, root: usize, adj: &[Vec<usize>]) -> Vec<QueryAnswer> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (answer_queries)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement answer_queries");
}
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn small_binary_tree() {
        let num_verts = 127;
        let mut tree: Vec<Vec<usize>> = vec![vec![]; num_verts + 1];
        for i in 1..=num_verts >> 1 {
            let left_child = i << 1;
            let right_child = left_child + 1;
            tree[i].push(left_child);
            tree[i].push(right_child);
            tree[left_child].push(i);
            tree[right_child].push(i);
        }
        let mut online_answers: Vec<QueryAnswer> = Vec::new();
        let mut online = LowestCommonAncestorOnline::new(num_verts);
        let mut offline = LowestCommonAncestorOffline::new(num_verts);
        let mut query_id = 314; // A random number, doesn't matter
        online.fill_sparse_table(1, 0, 0, &tree);
        for i in 1..=num_verts {
            for j in 1..i {
                // Query every possible pair
                online_answers.push(QueryAnswer {
                    query_id,
                    answer: online.get_ancestor(i, j),
                });
                offline.add_query(i, j, query_id);
                query_id += 1;
            }
        }
        let mut offline_answers = offline.answer_queries(1, &tree);
        offline_answers.sort_unstable_by_key(|a| a.query_id);
        assert_eq!(offline_answers, online_answers);
    }
}
