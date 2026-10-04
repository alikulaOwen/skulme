# Huber Loss

**Category:** `math` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

# Huber Loss Function

The `huber_loss` function calculates the Huber loss, which is a robust loss function used in machine learning, particularly in regression problems.

Huber loss combines the benefits of mean squared error (MSE) and mean absolute error (MAE). It behaves like MSE when the difference between actual and predicted values is small (less than a specified `delta`), and like MAE when the difference is large.

## Formula

For a pair of actual and predicted values, represented as vectors `actual` and `predicted`, and a specified `delta` value, the Huber loss is calculated as:

- If the absolute difference between `actual[i]` and `predicted[i]` is less than or equal to `delta`, the loss is `0.5 * (actual[i] - predicted[i])^2`.
- If the absolute difference is greater than `delta`, the loss is `delta * |actual[i] - predicted[i]| - 0.5 * delta`.

The total loss is the sum of individual losses over all elements.

## Huber Loss Function Implementation

This implementation takes two references to vectors of f64 values, `actual` and `predicted`, and a `delta` value. It returns the Huber loss between them, providing a robust measure of dissimilarity between actual and predicted values.

### Original Rust Signatures
```rust
pub fn huber_loss(actual: &[f64], predicted: &[f64], delta: f64) -> f64;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`huber_loss.rs`](../../../src/math/huber_loss.rs).

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
cargo test --lib math::huber_loss
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`huber_loss.rs`](../../../src/math/huber_loss.rs).
