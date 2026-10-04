# Exponential Linear Unit

**Category:** `math` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

# Exponential Linear Unit (ELU) Function

The `exponential_linear_unit` function computes the Exponential Linear Unit (ELU) values of a given vector
of f64 numbers with a specified alpha parameter.

The ELU activation function is commonly used in neural networks as an alternative to the Leaky ReLU function.
It introduces a small negative slope (controlled by the alpha parameter) for the negative input values and has
an exponential growth for positive values, which can help mitigate the vanishing gradient problem.

## Formula

For a given input vector `x` and an alpha parameter `alpha`, the ELU function computes the output
`y` as follows:

`y_i = { x_i if x_i >= 0, alpha * (e^x_i - 1) if x_i < 0 }`

Where `e` is the mathematical constant (approximately 2.71828).

## Exponential Linear Unit (ELU) Function Implementation

This implementation takes a reference to a vector of f64 values and an alpha parameter, and returns a new
vector with the ELU transformation applied to each element. The input vector is not altered.

### Original Rust Signatures
```rust
pub fn exponential_linear_unit(vector: &Vec<f64>, alpha: f64) -> Vec<f64>;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`exponential_linear_unit.rs`](../../../src/math/exponential_linear_unit.rs).

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
cargo test --lib math::exponential_linear_unit
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`exponential_linear_unit.rs`](../../../src/math/exponential_linear_unit.rs).
