# Backtracking - Algorithm Practice & Interview Guide

Backtracking incrementally builds candidates for solutions and abandons ('backtracks') as soon as candidate cannot yield valid solution.

## Key Java Interview Takeaways
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

## Comparison: Rust vs Java
- In Rust, recursion often takes `&mut Vec<T>` and explicitly calls `vec.pop()`, or passes immutable clones.
- Java's `List.remove(list.size() - 1)` or `Deque.removeLast()` is the standard backtracking pattern.

---

## Problems & Practice Workspaces (10 Problems)

Each problem has a dedicated workspace with problem statements, runnable Java apps (`java Solution.java`), Python (`python3 solution.py`), TypeScript (`bun solution.ts`), and comparison to the original Rust implementation.

| Problem | Rust Source | Java Executable | Rust Reference Test |
| :--- | :--- | :--- | :--- |
| [All Combination Of Size K](../../problems/backtracking/all_combination_of_size_k/README.md) | [`all_combination_of_size_k.rs`](./all_combination_of_size_k.rs) | `java Solution.java` | `cargo test --lib backtracking::all_combination_of_size_k` |
| [Graph Coloring](../../problems/backtracking/graph_coloring/README.md) | [`graph_coloring.rs`](./graph_coloring.rs) | `java Solution.java` | `cargo test --lib backtracking::graph_coloring` |
| [Hamiltonian Cycle](../../problems/backtracking/hamiltonian_cycle/README.md) | [`hamiltonian_cycle.rs`](./hamiltonian_cycle.rs) | `java Solution.java` | `cargo test --lib backtracking::hamiltonian_cycle` |
| [Knight Tour](../../problems/backtracking/knight_tour/README.md) | [`knight_tour.rs`](./knight_tour.rs) | `java Solution.java` | `cargo test --lib backtracking::knight_tour` |
| [N Queens](../../problems/backtracking/n_queens/README.md) | [`n_queens.rs`](./n_queens.rs) | `java Solution.java` | `cargo test --lib backtracking::n_queens` |
| [Parentheses Generator](../../problems/backtracking/parentheses_generator/README.md) | [`parentheses_generator.rs`](./parentheses_generator.rs) | `java Solution.java` | `cargo test --lib backtracking::parentheses_generator` |
| [Permutations](../../problems/backtracking/permutations/README.md) | [`permutations.rs`](./permutations.rs) | `java Solution.java` | `cargo test --lib backtracking::permutations` |
| [Rat In Maze](../../problems/backtracking/rat_in_maze/README.md) | [`rat_in_maze.rs`](./rat_in_maze.rs) | `java Solution.java` | `cargo test --lib backtracking::rat_in_maze` |
| [Subset Sum](../../problems/backtracking/subset_sum/README.md) | [`subset_sum.rs`](./subset_sum.rs) | `java Solution.java` | `cargo test --lib backtracking::subset_sum` |
| [Sudoku](../../problems/backtracking/sudoku/README.md) | [`sudoku.rs`](./sudoku.rs) | `java Solution.java` | `cargo test --lib backtracking::sudoku` |

---
*Generated for Java Software Engineering Interview Preparation.*
