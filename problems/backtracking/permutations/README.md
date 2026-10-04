# Permutations

**Category:** `backtracking` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

This module provides a function to generate all possible distinct permutations
of a given collection of integers using a backtracking algorithm.

### Original Rust Signatures
```rust
pub fn permute(mut nums: Vec<isize>) -> Vec<Vec<isize>>;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`permutations.rs`](../../../src/backtracking/permutations.rs).

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
cargo test --lib backtracking::permutations
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`permutations.rs`](../../../src/backtracking/permutations.rs).
