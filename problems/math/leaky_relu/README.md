# Leaky Relu

**Category:** `math` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

# Leaky ReLU Function

The `leaky_relu` function computes the Leaky Rectified Linear Unit (ReLU) values of a given vector
of f64 numbers with a specified alpha parameter.

The Leaky ReLU activation function is commonly used in neural networks to introduce a small negative
slope (controlled by the alpha parameter) for the negative input values, preventing neurons from dying
during training.

## Formula

For a given input vector `x` and an alpha parameter `alpha`, the Leaky ReLU function computes the output
`y` as follows:

`y_i = { x_i if x_i >= 0, alpha * x_i if x_i < 0 }`

## Leaky ReLU Function Implementation

This implementation takes a reference to a vector of f64 values and an alpha parameter, and returns a new
vector with the Leaky ReLU transformation applied to each element. The input vector is not altered.

### Original Rust Signatures
```rust
pub fn leaky_relu(vector: &Vec<f64>, alpha: f64) -> Vec<f64>;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`leaky_relu.rs`](../../../src/math/leaky_relu.rs).

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
cargo test --lib math::leaky_relu
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`leaky_relu.rs`](../../../src/math/leaky_relu.rs).
