# Ant Colony Optimization

**Category:** `graph` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

Ant Colony Optimization (ACO) algorithm for solving the Travelling Salesman Problem (TSP).

The Travelling Salesman Problem asks: "Given a list of cities and the distances between
each pair of cities, what is the shortest possible route that visits each city exactly
once and returns to the origin city?"

The ACO algorithm uses artificial ants that build solutions iteratively. Each ant constructs
a tour by probabilistically choosing the next city based on pheromone trails and heuristic
information (distance). After all ants complete their tours, pheromone trails are updated,
with stronger pheromones deposited on shorter routes. Over multiple iterations, this process
converges toward finding good solutions to the TSP.

# References
- [Ant Colony Optimization Algorithms](https://en.wikipedia.org/wiki/Ant_colony_optimization_algorithms)
- [Travelling Salesman Problem](https://en.wikipedia.org/wiki/Travelling_salesman_problem)

### Original Rust Signatures
```rust
pub fn ant_colony_optimization(cities: Vec<(f64, f64);
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`ant_colony_optimization.rs`](../../../src/graph/ant_colony_optimization.rs).

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
cargo test --lib graph::ant_colony_optimization
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`ant_colony_optimization.rs`](../../../src/graph/ant_colony_optimization.rs).
