# Modular Exponential

**Category:** `math` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

Calculate the greatest common divisor (GCD) of two numbers and the
coefficients of Bézout's identity using the Extended Euclidean Algorithm.

# Arguments

* `a` - One of the numbers to find the GCD of
* `m` - The other number to find the GCD of

# Returns

A tuple (gcd, x1, x2) such that:
gcd - the greatest common divisor of a and m.
x1, x2 - the coefficients such that `a * x1 + m * x2` is equivalent to `gcd` modulo `m`.

### Original Rust Signatures
```rust
pub fn gcd_extended(a: i64, m: i64) -> (i64, i64, i64);
pub fn mod_inverse(b: i64, m: i64) -> i64;
pub fn modular_exponential(base: i64, mut power: i64, modulus: i64) -> i64;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`modular_exponential.rs`](../../../src/math/modular_exponential.rs).

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
cargo test --lib math::modular_exponential
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`modular_exponential.rs`](../../../src/math/modular_exponential.rs).
