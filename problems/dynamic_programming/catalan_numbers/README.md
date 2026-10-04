# Catalan Numbers

**Category:** `dynamic_programming` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

Catalan Numbers using Dynamic Programming

The Catalan numbers are a sequence of positive integers that appear in many
counting problems in combinatorics. Such problems include counting:
- The number of Dyck words of length 2n
- The number of well-formed expressions with n pairs of parentheses
(e.g., `()()` is valid but `())(` is not)
- The number of different ways n + 1 factors can be completely parenthesized
(e.g., for n = 2, C(n) = 2 and (ab)c and a(bc) are the two valid ways)
- The number of full binary trees with n + 1 leaves

A Catalan number satisfies the following recurrence relation:
- C(0) = C(1) = 1
- C(n) = sum(C(i) * C(n-i-1)), from i = 0 to n-1

Sources:
- [Brilliant.org](https://brilliant.org/wiki/catalan-numbers/)
- [Wikipedia](https://en.wikipedia.org/wiki/Catalan_number)

### Original Rust Signatures
```rust
pub fn catalan_numbers(upper_limit: usize) -> Vec<u64>;
```

### Complexity
- **Time Complexity:** `O(n²)`
- **Space Complexity:** `O(n)`

---

## Java Interview Strategy & Tips

- Multi-dimensional arrays `int[][] dp = new int[m][n]` in Java are arrays of heap references; consider flat arrays `int[m * n]` or rolling 1D arrays for cache locality.
- Watch for integer overflow when initializing memoization tables with `Integer.MAX_VALUE` (adding 1 wraps around to negative). Use `1_000_000_000` or check for sentinel before adding.
- Identify: State definition, Base cases, Transition relation, and Evaluation order.

### Rust vs. Java Perspective
- Rust guarantees memory safety and bounds checks, but idiomatically uses flat vectors `Vec<T>` with 1D indexing.
- Java relies on JVM GC for allocated DP tables, so minimize object allocations inside DP loops.
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`catalan_numbers.rs`](../../../src/dynamic_programming/catalan_numbers.rs).

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
cargo test --lib dynamic_programming::catalan_numbers
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`catalan_numbers.rs`](../../../src/dynamic_programming/catalan_numbers.rs).
