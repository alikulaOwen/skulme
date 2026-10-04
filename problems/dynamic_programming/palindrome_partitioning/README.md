# Palindrome Partitioning

**Category:** `dynamic_programming` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

Finds the minimum cuts needed for a palindrome partitioning of a string

Given a string s, partition s such that every substring of the partition is a palindrome.
This function returns the minimum number of cuts needed.

Time Complexity: O(n^2)
Space Complexity: O(n^2)

# Arguments

* `s` - The input string to partition

# Returns

The minimum number of cuts needed

# Examples

```
use the_algorithms_rust::dynamic_programming::minimum_palindrome_partitions;

assert_eq!(minimum_palindrome_partitions("aab"), 1);
assert_eq!(minimum_palindrome_partitions("aaa"), 0);
assert_eq!(minimum_palindrome_partitions("ababbbabbababa"), 3);
```

# Algorithm Explanation

The algorithm uses dynamic programming with two key data structures:
- `cut[i]`: minimum cuts needed for substring from index 0 to i
- `is_palindromic[j][i]`: whether substring from index j to i is a palindrome

For each position i, we check all possible starting positions j to determine
if the substring s[j..=i] is a palindrome. If it is, we update the minimum
cut count accordingly.

Reference: <https://www.youtube.com/watch?v=_H8V5hJUGd0>

### Original Rust Signatures
```rust
pub fn minimum_palindrome_partitions(s: &str) -> usize;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`palindrome_partitioning.rs`](../../../src/dynamic_programming/palindrome_partitioning.rs).

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
cargo test --lib dynamic_programming::palindrome_partitioning
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`palindrome_partitioning.rs`](../../../src/dynamic_programming/palindrome_partitioning.rs).
