/*
Heavy Light Decomposition:
It partitions a tree into disjoint paths such that:
1. Each path is a part of some leaf's path to root
2. The number of paths from any vertex to the root is of O(lg(n))
Such a decomposition can be used to answer many types of queries about vertices
or edges on a particular path. It is often used with some sort of binary tree
to handle different operations on the paths, for example segment tree or
fenwick tree.

Many members of this struct are made public, because they can either be
supplied by the developer, or can be useful for other parts of the code.

The implementation assumes that the tree vertices are numbered from 1 to n
and it is represented using (compressed) adjacency matrix. If this is not true,
maybe `graph_enumeration.rs` can help.
*/

type Adj = [Vec<usize>];

pub struct HeavyLightDecomposition {
    // Each vertex is assigned a number from 1 to n. For `v` and `u` such that
    // u is parent of v, and both are in path `p`, it is true that:
    // position[u] = position[v] - 1
    pub position: Vec<usize>,

    // The first (closest to root) vertex of the path containing each vertex
    pub head: Vec<usize>,

    // The "heaviest" child of each vertex, its subtree is at least as big as
    // the other ones. If `v` is a leaf, big_child[v] = 0
    pub big_child: Vec<usize>,

    // Used internally to fill `position` Vec
    current_position: usize,
}

impl HeavyLightDecomposition {
    pub fn new(mut num_vertices: usize) -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}
    fn dfs(&mut self, v: usize, parent: usize, adj: &Adj) -> usize {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (dfs)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement dfs");
}
    pub fn decompose(&mut self, root: usize, adj: &Adj) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (decompose)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement decompose");
}
    fn decompose_path(&mut self, v: usize, parent: usize, head: usize, adj: &Adj) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (decompose_path)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement decompose_path");
}
}


#[cfg(test)]
mod tests {
    use super::*;

    struct LinearCongruenceGenerator {
        // modulus as 2 ^ 32
        multiplier: u32,
        increment: u32,
        state: u32,
    }

    impl LinearCongruenceGenerator {
        fn new(multiplier: u32, increment: u32, state: u32) -> Self {
            Self {
                multiplier,
                increment,
                state,
            }
        }
        fn next(&mut self) -> u32 {
            self.state =
                (self.multiplier as u64 * self.state as u64 + self.increment as u64) as u32;
            self.state
        }
    }

    fn get_num_paths(
        hld: &HeavyLightDecomposition,
        mut v: usize,
        parent: &[usize],
    ) -> (usize, usize) {
        // Return height and number of paths
        let mut ans = 0usize;
        let mut height = 0usize;
        let mut prev_head = 0usize;
        loop {
            height += 1;
            let head = hld.head[v];
            if head != prev_head {
                ans += 1;
                prev_head = head;
            }
            v = parent[v];
            if v == 0 {
                break;
            }
        }
        (ans, height)
    }

    #[test]
    fn single_path() {
        let mut adj = vec![vec![], vec![2], vec![3], vec![4], vec![5], vec![6], vec![]];
        let mut hld = HeavyLightDecomposition::new(6);
        hld.decompose(1, &adj);
        assert_eq!(hld.head, vec![0, 1, 1, 1, 1, 1, 1]);
        assert_eq!(hld.position, vec![0, 1, 2, 3, 4, 5, 6]);
        assert_eq!(hld.big_child, vec![0, 2, 3, 4, 5, 6, 0]);

        adj[3].push(2);
        adj[2].push(1);
        hld.decompose(3, &adj);
        assert_eq!(hld.head, vec![0, 2, 2, 3, 3, 3, 3]);
        assert_eq!(hld.position, vec![0, 6, 5, 1, 2, 3, 4]);
        assert_eq!(hld.big_child, vec![0, 0, 1, 4, 5, 6, 0]);
    }

    #[test]
    fn random_tree() {
        // Let it have 1e4 vertices. It should finish under 100ms even with
        // 1e5 vertices
        let n = 1e4 as usize;
        let threshold = 14; // 2 ^ 14 = 16384 > n
        let mut adj: Vec<Vec<usize>> = vec![vec![]; n + 1];
        let mut parent: Vec<usize> = vec![0; n + 1];
        let mut hld = HeavyLightDecomposition::new(n);
        let mut lcg = LinearCongruenceGenerator::new(1103515245, 12345, 314);
        parent[2] = 1;
        adj[1].push(2);
        #[allow(clippy::needless_range_loop)]
        for i in 3..=n {
            // randomly determine the parent of each vertex.
            // There will be modulus bias, but it isn't important
            let par_max = i - 1;
            let par_min = (10 * par_max + 1) / 11;
            // Bring par_min closer to par_max to increase expected tree height
            let par = (lcg.next() as usize % (par_max - par_min + 1)) + par_min;
            adj[par].push(i);
            parent[i] = par;
        }
        // let's get a few leaves
        let leaves: Vec<usize> = (1..=n)
            .rev()
            .filter(|&v| adj[v].is_empty())
            .take(100)
            .collect();
        hld.decompose(1, &adj);
        for l in leaves {
            let (p, _h) = get_num_paths(&hld, l, &parent);
            assert!(p <= threshold);
        }
    }
}
