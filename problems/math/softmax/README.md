# Softmax

**Category:** `math` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

# Softmax Function

The `softmax` function computes the softmax values of a given array of f32 numbers.

The softmax operation is often used in machine learning for converting a vector of real numbers into a
probability distribution. It exponentiates each element in the input array, and then normalizes the
results so that they sum to 1.

## Formula

For a given input array `x`, the softmax function computes the output `y` as follows:

`y_i = e^(x_i) / sum(e^(x_j) for all j)`

## Softmax Function Implementation

This implementation uses the `std::f32::consts::E` constant for the base of the exponential function. and
f32 vectors to compute the values. The function creates a new vector and not altering the input vector.

### Original Rust Signatures
```rust
pub fn softmax(array: Vec<f32>) -> Vec<f32>;
```

### Complexity
- **Time Complexity:** `O(N)`
- **Space Complexity:** `O(1)`

---

## Java Interview Strategy & Tips

- Watch out for 32-bit integer overflow: `(a + b)` and `(a * b)` can easily exceed `Integer.MAX_VALUE` (2^31 - 1). Cast to `long` before multiplication: `(long) a * b % MOD`.
- For arbitrary precision arithmetic, use `java.math.BigInteger` and `java.math.BigDecimal`.

### Rust vs. Java Perspective
- Rust panics on debug integer overflow and wraps in release mode (or offers `checked_add`, `saturating_mul`).
- Java silently overflows integer operations without exception unless `Math.addExact()` is explicitly used.
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`softmax.rs`](../../../src/math/softmax.rs).

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
cargo test --lib math::softmax
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`softmax.rs`](../../../src/math/softmax.rs).
