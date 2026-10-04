# Smith Waterman

**Category:** `dynamic_programming` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

This module contains the Smith-Waterman algorithm implementation for local sequence alignment.

The Smith-Waterman algorithm is a dynamic programming algorithm used for determining
similar regions between two sequences (nucleotide or protein sequences). It is particularly
useful in bioinformatics for identifying optimal local alignments.

# Algorithm Overview

The algorithm works by:
1. Creating a scoring matrix where each cell represents the maximum alignment score
ending at that position
2. Using match, mismatch, and gap penalties to calculate scores
3. Allowing scores to reset to 0 (ensuring local rather than global alignment)
4. Tracing back from the highest scoring position to reconstruct the alignment

# Time Complexity

O(m * n) where m and n are the lengths of the two sequences

# Space Complexity

O(m * n) for the scoring matrix

# References

- [Smith, T.F., Waterman, M.S. (1981). "Identification of Common Molecular Subsequences"](https://doi.org/10.1016/0022-2836(81)90087-5)
- [Wikipedia: Smith-Waterman algorithm](https://en.wikipedia.org/wiki/Smith%E2%80%93Waterman_algorithm)

### Original Rust Signatures
```rust
pub fn score_function(source_char: char,
    target_char: char,
    match_score: i32,
    mismatch_score: i32,
    gap_score: i32,) -> i32;
pub fn smith_waterman(query: &str,
    subject: &str,
    match_score: i32,
    mismatch_score: i32,
    gap_score: i32,) -> Vec<Vec<i32>>;
pub fn traceback(score: &[Vec<i32>],
    query: &str,
    subject: &str,
    match_score: i32,
    mismatch_score: i32,
    gap_score: i32,) -> String;
```

### Complexity
- **Time Complexity:** `O(N)`
- **Space Complexity:** `O(1)`

---

## Java Interview Strategy & Tips

- Multi-dimensional arrays `int[][] dp = new int[m][n]` in Java are arrays of heap references; consider flat arrays `int[m * n]` or rolling 1D arrays for cache locality.
- Watch for integer overflow when initializing memoization tables with `Integer.MAX_VALUE` (adding 1 wraps around to negative). Use `1_000_000_000` or check for sentinel before adding.
- Identify: State definition, Base cases, Transition relation, and Evaluation order.

### Rust vs. Java Perspective
- Rust guarantees memory safety and bounds checks, but idiomatically uses flat vectors `Vec<T>` with 1D indexing.
- Java relies on JVM GC for allocated DP tables, so minimize object allocations inside DP loops.
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`smith_waterman.rs`](../../../src/dynamic_programming/smith_waterman.rs).

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
cargo test --lib dynamic_programming::smith_waterman
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`smith_waterman.rs`](../../../src/dynamic_programming/smith_waterman.rs).
