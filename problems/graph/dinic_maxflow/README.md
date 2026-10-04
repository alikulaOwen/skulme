# Dinic Maxflow

**Category:** `graph` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

We assume that T::default() gives "zero" flow and T supports negative values

### Original Rust Signatures
```rust
pub fn new(sink: usize, capacity: T) -> Self;
pub fn new(source: usize, sink: usize, num_vertices: usize) -> Self;
pub fn add_edge(&mut self, source: usize, sink: usize, capacity: T);
pub fn find_maxflow(&mut self, infinite_flow: T) -> T;
pub fn get_flow_edges(&mut self, infinite_flow: T) -> Vec<FlowResultEdge<T>>;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`dinic_maxflow.rs`](../../../src/graph/dinic_maxflow.rs).

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
cargo test --lib graph::dinic_maxflow
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`dinic_maxflow.rs`](../../../src/graph/dinic_maxflow.rs).
