# Page Rank

**Category:** `graph` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

Calculates the PageRank for each node in a graph.

The graph is represented as an adjacency list: `HashMap<Node, Vec<Node>>`,
where each key is a source node pointing to a vector of destination nodes.

# Parameters
* `graph` - The adjacency list of the graph.
* `damping_factor` - The probability that a surfer continues clicking links should be between 0 and 1 (typically 0.85).
* `max_iterations` - The maximum number of iterations to perform (typically 100).
* `convergence_threshold` - The L1 difference threshold to stop iterations early (typically 1e-5).

### Original Rust Signatures
```rust
pub fn page_rank(graph: &HashMap<Node, Vec<Node>>,
    damping_factor: f64,
    max_iterations: usize,
    convergence_threshold: f64,) -> HashMap<Node, f64>;
```

### Complexity
- **Time Complexity:** `O(N)`
- **Space Complexity:** `O(1)`

---

## Java Interview Strategy & Tips

- Adjacency lists are typically represented as `List<List<Integer>>` or `Map<Integer, List<Integer>>`.
- Use `computeIfAbsent(u, k -> new ArrayList<>()).add(v)` for concise graph construction.
- For BFS: ALWAYS use `Queue<Integer> q = new ArrayDeque<>()` (never `LinkedList`, which incurs heavy node allocation overhead).
- For Dijkstra: Use `PriorityQueue<int[]> pq = new PriorityQueue<>(Comparator.comparingInt(a -> a[1]))` where `a[0]` is node and `a[1]` is distance.

### Rust vs. Java Perspective
- Rust graphs often use `BTreeMap<V, BTreeMap<V, E>>` or adjacency lists with explicit vertex indexing.
- In Rust, graph traversal requires explicit borrowing or node index IDs to avoid borrow checker conflicts with circular references.
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`page_rank.rs`](../../../src/graph/page_rank.rs).

---

## How to Spin & Run

### 1. Java (Target Interview Language)
Run directly as a single-file application with built-in tests:
```bash
java Solution.java
```

### 2. Rust (Original Ground Truth Answer)
Run the crate unit tests for this module from the repository root:
```bash
cargo test --lib graph::page_rank
```

### 3. Python (Rapid Prototyping)
```bash
python3 solution.py
```

### 4. TypeScript (Industry Standard)
```bash
bun solution.ts
# or using Deno:
deno run solution.ts
```

---

## Self-Evaluation Checklist
- [ ] Understand the problem constraints and edge cases (e.g. empty input, bounds, duplicates).
- [ ] Implement the optimal solution in `Solution.java`.
- [ ] Verify correctness using `java Solution.java`.
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`page_rank.rs`](../../../src/graph/page_rank.rs).
