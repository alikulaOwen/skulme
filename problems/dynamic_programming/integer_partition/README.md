# Integer Partition

**Category:** `dynamic_programming` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

Integer partition using dynamic programming

The number of partitions of a number n into at least k parts equals the number of
partitions into exactly k parts plus the number of partitions into at least k-1 parts.
Subtracting 1 from each part of a partition of n into k parts gives a partition of n-k
into k parts. These two facts together are used for this algorithm.

More info:
* <https://en.wikipedia.org/wiki/Partition_(number_theory)>
* <https://en.wikipedia.org/wiki/Partition_function_(number_theory)>

### Original Rust Signatures
```rust
pub fn partition(m: i32) -> u128;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`integer_partition.rs`](../../../src/dynamic_programming/integer_partition.rs).

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
cargo test --lib dynamic_programming::integer_partition
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`integer_partition.rs`](../../../src/dynamic_programming/integer_partition.rs).
