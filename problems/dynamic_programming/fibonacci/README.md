# Fibonacci

**Category:** `dynamic_programming` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

fibonacci(n) returns the nth fibonacci number
This function uses the definition of Fibonacci where:
F(0) = F(1) = 1 and F(n+1) = F(n) + F(n-1) for n>0

Warning: This will overflow the 128-bit unsigned integer at n=186

### Original Rust Signatures
```rust
pub fn fibonacci(n: u32) -> u128;
pub fn recursive_fibonacci(n: u32) -> u128;
pub fn classical_fibonacci(n: u32) -> u128;
pub fn logarithmic_fibonacci(n: u32) -> u128;
pub fn memoized_fibonacci(n: u32) -> u128;
pub fn matrix_fibonacci(n: u32) -> u128;
pub fn binary_lifting_fibonacci(n: u32) -> u128;
pub fn nth_fibonacci_number_modulo_m(n: i64, m: i64) -> i128;
pub fn last_digit_of_the_sum_of_nth_fibonacci_number(n: i64) -> i64;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`fibonacci.rs`](../../../src/dynamic_programming/fibonacci.rs).

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
cargo test --lib dynamic_programming::fibonacci
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`fibonacci.rs`](../../../src/dynamic_programming/fibonacci.rs).
