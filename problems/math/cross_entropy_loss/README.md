# Cross Entropy Loss

**Category:** `math` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

# Cross-Entropy Loss Function

The `cross_entropy_loss` function calculates the cross-entropy loss between the actual and predicted probability distributions.

Cross-entropy loss is commonly used in machine learning and deep learning to measure the dissimilarity between two probability distributions. It is often used in classification problems.

## Formula

For a pair of actual and predicted probability distributions represented as vectors `actual` and `predicted`, the cross-entropy loss is calculated as:

`L = -Σ(actual[i] * ln(predicted[i]))` for all `i` in the range of the vectors

Where `ln` is the natural logarithm function, and `Σ` denotes the summation over all elements of the vectors.

## Cross-Entropy Loss Function Implementation

This implementation takes two references to vectors of f64 values, `actual` and `predicted`, and returns the cross-entropy loss between them.

### Original Rust Signatures
```rust
pub fn cross_entropy_loss(actual: &[f64], predicted: &[f64]) -> f64;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`cross_entropy_loss.rs`](../../../src/math/cross_entropy_loss.rs).

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
cargo test --lib math::cross_entropy_loss
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`cross_entropy_loss.rs`](../../../src/math/cross_entropy_loss.rs).
