# 🎮 GitLeet Playground: Backtracking Track

Welcome to the **Interactive Backtracking Playground**!

This module provides a structured, hands-on curriculum of **10 classic backtracking problems** implemented in Java. Each problem is designed as an interactive sandbox:
- **`README.md`**: Visual mental model, step-by-step intuition, beginner concepts, and progressive hints.
- **`Solution.java`**: Clean skeleton with rich educational comments (WHAT, WHY, HOW it connects to bigger problems, and BENEFIT). The implementation is left unsolved for **you** to write and test!
- **Built-in Test Harness**: Every problem runs as a standalone Java file with comprehensive tests reporting `[PASS]` and `[FAIL]`.

---

## The Universal Backtracking Formula

Every single problem in this track follows the exact same 3-step dance:

```
                      ┌──────────────────────┐
                      │    1. CHOOSE         │
                      │ (Add candidate move) │
                      └──────────┬───────────┘
                                 │
                                 ▼
                      ┌──────────────────────┐
                      │    2. EXPLORE        │
                      │ (Recurse to next step│
                      └──────────┬───────────┘
                                 │
                                 ▼
                      ┌──────────────────────┐
                      │  3. UN-CHOOSE        │
                      │(Backtrack to restore)│
                      └──────────────────────┘
```

---

## 🗺️ Curriculum Roadmap (Easiest to Hardest)

Work through the problems in order. Each stage introduces a new algorithmic concept:

### Stage 1: Combinations, Permutations & Subsets (Core Recursion)
Master moving forward with pointers, using boolean trackers, and handling Catalan invariants.

| # | Problem | LeetCode Equivalent | Key Concept | Test Command |
|---|---|---|---|---|
| 1 | [All Combinations of Size K](all_combination_of_size_k/README.md) | [LC #77](https://leetcode.com/problems/combinations/) | Loop `start` pointer, increasing order, reference trap | `java all_combination_of_size_k/Solution.java` |
| 2 | [Distinct Permutations](permutations/README.md) | [LC #46](https://leetcode.com/problems/permutations/) / [47](https://leetcode.com/problems/permutations-ii/) | `boolean[] used`, skipping duplicate branches | `java permutations/Solution.java` |
| 3 | [Parentheses Generator](parentheses_generator/README.md) | [LC #22](https://leetcode.com/problems/generate-parentheses/) | Counting invariants (`open < n`, `close < open`), Catalan numbers | `java parentheses_generator/Solution.java` |
| 4 | [Subset Sum](subset_sum/README.md) | [LC #416](https://leetcode.com/problems/partition-equal-subset-sum/) | Binary decision tree (0/1 choice: Include vs Exclude) | `java subset_sum/Solution.java` |

---

### Stage 2: 2D Spatial & Grid Navigation
Apply backtracking across matrices and chessboards with directional moves and boundary checks.

| # | Problem | LeetCode Equivalent | Key Concept | Test Command |
|---|---|---|---|---|
| 5 | [Rat in a Maze](rat_in_maze/README.md) | Classic Grid DFS | 4-directional moves (Right, Down, Left, Up), obstacle collision | `java rat_in_maze/Solution.java` |
| 6 | [N-Queens](n_queens/README.md) | [LC #51](https://leetcode.com/problems/n-queens/) | Row-by-row placement, column & diagonal safety checks | `java n_queens/Solution.java` |
| 7 | [Knight's Tour](knight_tour/README.md) | Hamiltonian Path | 8 L-shaped moves, traversing all squares on board | `java knight_tour/Solution.java` |

---

### Stage 3: Constraint Satisfaction & Graph Topology
Advanced constraint propagation, adjacency matrices, and NP-complete problems.

| # | Problem | LeetCode Equivalent | Key Concept | Test Command |
|---|---|---|---|---|
| 8 | [Sudoku Solver](sudoku/README.md) | [LC #37](https://leetcode.com/problems/sudoku-solver/) | Finding empty cell, row/column/3x3 box constraint validation | `java sudoku/Solution.java` |
| 9 | [Hamiltonian Cycle](hamiltonian_cycle/README.md) | TSP Foundation | Visiting all graph vertices once and closing the cycle | `java hamiltonian_cycle/Solution.java` |
| 10 | [Graph Coloring](graph_coloring/README.md) | $M$-Coloring | Vertex coloring, neighbor clash checks, compiler register allocation | `java graph_coloring/Solution.java` |

---

## ⚡ The 3 Golden Java Rules for Backtracking

1. **The #1 Java Bug (The Reference Trap):**
   ```java
   // ❌ WRONG: Stores a reference to a mutable list that gets emptied
   result.add(currentPath);

   // ✅ CORRECT: Take an independent snapshot
   result.add(new ArrayList<>(currentPath));
   ```

2. **Stack Mutation (Choose & Un-Choose):**
   ```java
   currentPath.add(item);                         // Choose
   backtrack(...);                                // Explore
   currentPath.remove(currentPath.size() - 1);    // Un-choose (Backtrack)
   ```

3. **Pruning (Stop early when impossible):**
   Never explore branches that cannot mathematically lead to a valid answer. Checking validity **before** recursing keeps search trees small and fast.

---

## 🚀 How to Play

1. Pick a problem directory (e.g. `cd problems/backtracking/all_combination_of_size_k`).
2. Run `java Solution.java` to see the automated test harness.
3. Open `Solution.java`, follow the guided in-code hints, and write the recursive logic.
4. Re-run `java Solution.java` until all tests pass with `🎉 [PASS]`!
