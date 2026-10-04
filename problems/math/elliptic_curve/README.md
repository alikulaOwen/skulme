# Elliptic Curve

**Category:** `math` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

/ Elliptic curve defined by `y^2 = x^3 + Ax + B` over a prime field `F` of
/ characteristic != 2, 3
/
/ The coefficients of the elliptic curve are the constant parameters `A` and `B`.
/
/ Points form an abelian group with the neutral element [`EllipticCurve::infinity`]. The points
/ are represented via affine coordinates ([`EllipticCurve::new`]) except for the points
/ at infinity ([`EllipticCurve::infinity`]).
/
/ # Example
/
/ ```
/ use the_algorithms_rust::math::{EllipticCurve, PrimeField};
/ type E = EllipticCurve<PrimeField<7>, 1, 0>;
/ let P = E::new(0, 0).expect("not on curve E");
/ assert_eq!(P + P, E::infinity());
/ ```

### Original Rust Signatures
```rust
pub fn infinity() -> Self;
pub fn new(x: impl Into<F>, y: impl Into<F>) -> Option<Self>;
pub fn is_infinity(&self) -> bool;
pub fn x(&self) -> &F;
pub fn y(&self) -> &F;
pub fn points() -> impl Iterator<Item = Self>;
pub fn cardinality() -> usize;
pub fn cardinality_counted_table() -> usize;
pub fn cardinality_counted_legendre() -> usize;
```

### Complexity
- **Time Complexity:** `O(P) <br>`
- **Space Complexity:** `O(P)`

---

## Java Interview Strategy & Tips

- Watch out for 32-bit integer overflow: `(a + b)` and `(a * b)` can easily exceed `Integer.MAX_VALUE` (2^31 - 1). Cast to `long` before multiplication: `(long) a * b % MOD`.
- For arbitrary precision arithmetic, use `java.math.BigInteger` and `java.math.BigDecimal`.

### Rust vs. Java Perspective
- Rust panics on debug integer overflow and wraps in release mode (or offers `checked_add`, `saturating_mul`).
- Java silently overflows integer operations without exception unless `Math.addExact()` is explicitly used.
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`elliptic_curve.rs`](../../../src/math/elliptic_curve.rs).

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
cargo test --lib math::elliptic_curve
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`elliptic_curve.rs`](../../../src/math/elliptic_curve.rs).
