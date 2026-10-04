use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

type Graph<V> = BTreeMap<V, Vec<V>>;

pub fn prufer_encode<V: Ord + Copy>(tree: &Graph<V>) -> Vec<V> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (prufer_encode)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement prufer_encode");
}

#[inline]
fn add_directed_edge<V: Ord + Copy>(tree: &mut Graph<V>, a: V, b: V) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (add_directed_edge)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement add_directed_edge");
}

#[inline]
fn add_edge<V: Ord + Copy>(tree: &mut Graph<V>, a: V, b: V) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (add_edge)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement add_edge");
}

pub fn prufer_decode<V: Ord + Copy>(code: &[V], vertex_list: &[V]) -> Graph<V> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (prufer_decode)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement prufer_decode");
}


#[cfg(test)]
mod tests {
    use super::{add_edge, prufer_decode, prufer_encode, Graph};

    fn equal_graphs<V: Ord + Copy>(g1: &mut Graph<V>, g2: &mut Graph<V>) -> bool {
        for adj in g1.values_mut() {
            adj.sort();
        }
        for adj in g2.values_mut() {
            adj.sort();
        }
        g1 == g2
    }

    #[test]
    fn small_trees() {
        let mut g: Graph<u32> = Graph::new();
        // Binary tree with 7 vertices
        let edges = vec![(1, 2), (1, 3), (2, 4), (2, 5), (3, 6), (3, 7)];
        for (u, v) in edges {
            add_edge(&mut g, u, v);
        }
        let code = prufer_encode(&g);
        let vertices = g.keys().cloned().collect::<Vec<u32>>();
        let mut decoded = prufer_decode(&code, &vertices);
        assert_eq!(code, vec![3, 3, 2, 2, 1]);
        assert!(equal_graphs(&mut g, &mut decoded));

        g.clear();
        // A path of length 10
        for v in 2..=9 {
            g.insert(v, vec![v - 1, v + 1]);
        }
        g.insert(1, vec![2]);
        g.insert(10, vec![9]);
        let code = prufer_encode(&g);
        let vertices = g.keys().cloned().collect::<Vec<u32>>();
        let mut decoded = prufer_decode(&code, &vertices);
        assert_eq!(code, vec![9, 8, 7, 6, 5, 4, 3, 2]);
        assert!(equal_graphs(&mut g, &mut decoded));

        g.clear();
        // 7-5-3-1-2-4-6
        let edges = vec![(1, 2), (2, 4), (4, 6), (1, 3), (3, 5), (5, 7)];
        for (u, v) in edges {
            add_edge(&mut g, u, v);
        }
        let code = prufer_encode(&g);
        let vertices = g.keys().cloned().collect::<Vec<u32>>();
        let mut decoded = prufer_decode(&code, &vertices);
        assert_eq!(code, vec![5, 4, 3, 2, 1]);
        assert!(equal_graphs(&mut g, &mut decoded));
    }
}
