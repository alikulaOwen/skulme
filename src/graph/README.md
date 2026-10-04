# Graph - Algorithm Practice & Interview Guide

Graph algorithms explore networks of nodes connected by edges, including traversal, shortest path, and connectivity.

## Key Java Interview Takeaways
- Adjacency lists are typically represented as `List<List<Integer>>` or `Map<Integer, List<Integer>>`.
- Use `computeIfAbsent(u, k -> new ArrayList<>()).add(v)` for concise graph construction.
- For BFS: ALWAYS use `Queue<Integer> q = new ArrayDeque<>()` (never `LinkedList`, which incurs heavy node allocation overhead).
- For Dijkstra: Use `PriorityQueue<int[]> pq = new PriorityQueue<>(Comparator.comparingInt(a -> a[1]))` where `a[0]` is node and `a[1]` is distance.

## Comparison: Rust vs Java
- Rust graphs often use `BTreeMap<V, BTreeMap<V, E>>` or adjacency lists with explicit vertex indexing.
- In Rust, graph traversal requires explicit borrowing or node index IDs to avoid borrow checker conflicts with circular references.

---

## Problems & Practice Workspaces (29 Problems)

Each problem has a dedicated workspace with problem statements, runnable Java apps (`java Solution.java`), Python (`python3 solution.py`), TypeScript (`bun solution.ts`), and comparison to the original Rust implementation.

| Problem | Rust Source | Java Executable | Rust Reference Test |
| :--- | :--- | :--- | :--- |
| [Ant Colony Optimization](../../problems/graph/ant_colony_optimization/README.md) | [`ant_colony_optimization.rs`](./ant_colony_optimization.rs) | `java Solution.java` | `cargo test --lib graph::ant_colony_optimization` |
| [Astar](../../problems/graph/astar/README.md) | [`astar.rs`](./astar.rs) | `java Solution.java` | `cargo test --lib graph::astar` |
| [Bellman Ford](../../problems/graph/bellman_ford/README.md) | [`bellman_ford.rs`](./bellman_ford.rs) | `java Solution.java` | `cargo test --lib graph::bellman_ford` |
| [Bipartite Matching](../../problems/graph/bipartite_matching/README.md) | [`bipartite_matching.rs`](./bipartite_matching.rs) | `java Solution.java` | `cargo test --lib graph::bipartite_matching` |
| [Breadth First Search](../../problems/graph/breadth_first_search/README.md) | [`breadth_first_search.rs`](./breadth_first_search.rs) | `java Solution.java` | `cargo test --lib graph::breadth_first_search` |
| [Centroid Decomposition](../../problems/graph/centroid_decomposition/README.md) | [`centroid_decomposition.rs`](./centroid_decomposition.rs) | `java Solution.java` | `cargo test --lib graph::centroid_decomposition` |
| [Decremental Connectivity](../../problems/graph/decremental_connectivity/README.md) | [`decremental_connectivity.rs`](./decremental_connectivity.rs) | `java Solution.java` | `cargo test --lib graph::decremental_connectivity` |
| [Depth First Search](../../problems/graph/depth_first_search/README.md) | [`depth_first_search.rs`](./depth_first_search.rs) | `java Solution.java` | `cargo test --lib graph::depth_first_search` |
| [Depth First Search Tic Tac Toe](../../problems/graph/depth_first_search_tic_tac_toe/README.md) | [`depth_first_search_tic_tac_toe.rs`](./depth_first_search_tic_tac_toe.rs) | `java Solution.java` | `cargo test --lib graph::depth_first_search_tic_tac_toe` |
| [Detect Cycle](../../problems/graph/detect_cycle/README.md) | [`detect_cycle.rs`](./detect_cycle.rs) | `java Solution.java` | `cargo test --lib graph::detect_cycle` |
| [Dijkstra](../../problems/graph/dijkstra/README.md) | [`dijkstra.rs`](./dijkstra.rs) | `java Solution.java` | `cargo test --lib graph::dijkstra` |
| [Dinic Maxflow](../../problems/graph/dinic_maxflow/README.md) | [`dinic_maxflow.rs`](./dinic_maxflow.rs) | `java Solution.java` | `cargo test --lib graph::dinic_maxflow` |
| [Disjoint Set Union](../../problems/graph/disjoint_set_union/README.md) | [`disjoint_set_union.rs`](./disjoint_set_union.rs) | `java Solution.java` | `cargo test --lib graph::disjoint_set_union` |
| [Eulerian Path](../../problems/graph/eulerian_path/README.md) | [`eulerian_path.rs`](./eulerian_path.rs) | `java Solution.java` | `cargo test --lib graph::eulerian_path` |
| [Floyd Warshall](../../problems/graph/floyd_warshall/README.md) | [`floyd_warshall.rs`](./floyd_warshall.rs) | `java Solution.java` | `cargo test --lib graph::floyd_warshall` |
| [Ford Fulkerson](../../problems/graph/ford_fulkerson/README.md) | [`ford_fulkerson.rs`](./ford_fulkerson.rs) | `java Solution.java` | `cargo test --lib graph::ford_fulkerson` |
| [Graph Enumeration](../../problems/graph/graph_enumeration/README.md) | [`graph_enumeration.rs`](./graph_enumeration.rs) | `java Solution.java` | `cargo test --lib graph::graph_enumeration` |
| [Heavy Light Decomposition](../../problems/graph/heavy_light_decomposition/README.md) | [`heavy_light_decomposition.rs`](./heavy_light_decomposition.rs) | `java Solution.java` | `cargo test --lib graph::heavy_light_decomposition` |
| [Kosaraju](../../problems/graph/kosaraju/README.md) | [`kosaraju.rs`](./kosaraju.rs) | `java Solution.java` | `cargo test --lib graph::kosaraju` |
| [Lee Breadth First Search](../../problems/graph/lee_breadth_first_search/README.md) | [`lee_breadth_first_search.rs`](./lee_breadth_first_search.rs) | `java Solution.java` | `cargo test --lib graph::lee_breadth_first_search` |
| [Lowest Common Ancestor](../../problems/graph/lowest_common_ancestor/README.md) | [`lowest_common_ancestor.rs`](./lowest_common_ancestor.rs) | `java Solution.java` | `cargo test --lib graph::lowest_common_ancestor` |
| [Minimum Spanning Tree](../../problems/graph/minimum_spanning_tree/README.md) | [`minimum_spanning_tree.rs`](./minimum_spanning_tree.rs) | `java Solution.java` | `cargo test --lib graph::minimum_spanning_tree` |
| [Page Rank](../../problems/graph/page_rank/README.md) | [`page_rank.rs`](./page_rank.rs) | `java Solution.java` | `cargo test --lib graph::page_rank` |
| [Prim](../../problems/graph/prim/README.md) | [`prim.rs`](./prim.rs) | `java Solution.java` | `cargo test --lib graph::prim` |
| [Prufer Code](../../problems/graph/prufer_code/README.md) | [`prufer_code.rs`](./prufer_code.rs) | `java Solution.java` | `cargo test --lib graph::prufer_code` |
| [Strongly Connected Components](../../problems/graph/strongly_connected_components/README.md) | [`strongly_connected_components.rs`](./strongly_connected_components.rs) | `java Solution.java` | `cargo test --lib graph::strongly_connected_components` |
| [Tarjans Ssc](../../problems/graph/tarjans_ssc/README.md) | [`tarjans_ssc.rs`](./tarjans_ssc.rs) | `java Solution.java` | `cargo test --lib graph::tarjans_ssc` |
| [Topological Sort](../../problems/graph/topological_sort/README.md) | [`topological_sort.rs`](./topological_sort.rs) | `java Solution.java` | `cargo test --lib graph::topological_sort` |
| [Two Satisfiability](../../problems/graph/two_satisfiability/README.md) | [`two_satisfiability.rs`](./two_satisfiability.rs) | `java Solution.java` | `cargo test --lib graph::two_satisfiability` |

---
*Generated for Java Software Engineering Interview Preparation.*
