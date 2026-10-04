# Fast Fourier Transform

**Category:** `math` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

f64 complex

### Original Rust Signatures
```rust
pub fn new(re: f64, im: f64) -> Self;
pub fn square_norm(&self) -> f64;
pub fn norm(&self) -> f64;
pub fn inverse(&self) -> Complex64;
pub fn fast_fourier_transform_input_permutation(length: usize) -> Vec<usize>;
pub fn fast_fourier_transform(input: &[f64], input_permutation: &[usize]) -> Vec<Complex64>;
pub fn inverse_fast_fourier_transform(input: &[Complex64],
    input_permutation: &[usize],) -> Vec<f64>;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`fast_fourier_transform.rs`](../../../src/math/fast_fourier_transform.rs).

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
cargo test --lib math::fast_fourier_transform
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`fast_fourier_transform.rs`](../../../src/math/fast_fourier_transform.rs).
