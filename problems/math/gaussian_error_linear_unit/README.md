# Gaussian Error Linear Unit

**Category:** `math` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

# Gaussian Error Linear Unit (GELU) Function

The `gaussian_error_linear_unit` function computes the Gaussian Error Linear Unit (GELU) values of a given f64 number or a vector of f64 numbers.

GELU is an activation function used in neural networks that introduces a smooth approximation of the rectifier function (ReLU).
It is defined using the Gaussian cumulative distribution function and can help mitigate the vanishing gradient problem.

## Formula

For a given input value `x`, the GELU function computes the output `y` as follows:

`y = 0.5 * (1.0 + tanh(2.0 / sqrt(π) * (x + 0.044715 * x^3)))`

Where `tanh` is the hyperbolic tangent function and `π` is the mathematical constant (approximately 3.14159).

## Gaussian Error Linear Unit (GELU) Function Implementation

This implementation takes either a single f64 value or a reference to a vector of f64 values and returns the GELU transformation applied to each element. The input values are not altered.

### Original Rust Signatures
```rust
pub fn gaussian_error_linear_unit(vector: &Vec<f64>) -> Vec<f64>;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`gaussian_error_linear_unit.rs`](../../../src/math/gaussian_error_linear_unit.rs).

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
cargo test --lib math::gaussian_error_linear_unit
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`gaussian_error_linear_unit.rs`](../../../src/math/gaussian_error_linear_unit.rs).
