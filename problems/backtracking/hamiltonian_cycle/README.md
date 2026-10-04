# Hamiltonian Cycle

**Category:** `backtracking` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

This module provides functionality to find a Hamiltonian cycle in a directed or undirected graph.
Source: [Wikipedia](https://en.wikipedia.org/wiki/Hamiltonian_path_problem)

### Original Rust Signatures
```rust
pub fn find_hamiltonian_cycle(adjacency_matrix: Vec<Vec<bool>>,
    start_vertex: usize,) -> Result<Option<Vec<usize>>, FindHamiltonianCycleError>;
```

### Complexity
- **Time Complexity:** `O(N)`
- **Space Complexity:** `O(1)`

---

## Java Interview Strategy & Tips

- The #1 Java Backtracking Bug: Adding the mutable path directly to the result list `result.add(currentPath)`. ALWAYS add a shallow copy: `result.add(new ArrayList<>(currentPath))`.
- Pattern:
  ```java
  for (Choice choice : choices) {
      if (isValid(choice)) {
          state.add(choice);      // Choose
          backtrack(state, ...); // Explore
          state.removeLast();    // Un-choose (backtrack)
      }
  }
  ```

### Rust vs. Java Perspective
- In Rust, recursion often takes `&mut Vec<T>` and explicitly calls `vec.pop()`, or passes immutable clones.
- Java's `List.remove(list.size() - 1)` or `Deque.removeLast()` is the standard backtracking pattern.
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`hamiltonian_cycle.rs`](../../../src/backtracking/hamiltonian_cycle.rs).

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
cargo test --lib backtracking::hamiltonian_cycle
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`hamiltonian_cycle.rs`](../../../src/backtracking/hamiltonian_cycle.rs).
