# N Queens

**Category:** `backtracking` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

This module provides functionality to solve the N-Queens problem.

The N-Queens problem is a classic chessboard puzzle where the goal is to
place N queens on an NxN chessboard so that no two queens threaten each
other. Queens can attack each other if they share the same row, column, or
diagonal.

This implementation solves the N-Queens problem using a backtracking algorithm.
It starts with an empty chessboard and iteratively tries to place queens in
different rows, ensuring they do not conflict with each other. If a valid
solution is found, it's added to the list of solutions.

### Original Rust Signatures
```rust
pub fn n_queens_solver(n: usize) -> Vec<Vec<String>>;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`n_queens.rs`](../../../src/backtracking/n_queens.rs).

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
cargo test --lib backtracking::n_queens
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`n_queens.rs`](../../../src/backtracking/n_queens.rs).
