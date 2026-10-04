# Quadratic Residue

**Category:** `math` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

/ Cipolla algorithm
/
/ Solving quadratic residue problem:
/     x^2 = a (mod p) , p is an odd prime
/ with O(M*log(n)) time complexity, M depends on the complexity of complex numbers multiplication.
/
/ Wikipedia reference: https://en.wikipedia.org/wiki/Cipolla%27s_algorithm
/ When a is the primitive root modulo n, the answer is unique.
/ Otherwise it will return the smallest positive solution

### Original Rust Signatures
```rust
pub fn new(modulus: u64, i_square: u64) -> Self;
pub fn new(real: u64, imag: u64, f: Rc<CustomFiniteField>) -> Self;
pub fn mult_other(&mut self, rhs: &Self);
pub fn mult_self(&mut self);
pub fn fast_power(mut base: Self, mut power: u64) -> Self;
pub fn legendre_symbol(a: u64, odd_prime: u64) -> i64;
pub fn cipolla(a: u32, p: u32, seed: Option<u64>) -> Option<(u32, u32)>;
pub fn tonelli_shanks(a: i64, odd_prime: u64) -> Option<u64>;
```

### Complexity
- **Time Complexity:** `:{SystemTime, UNIX_EPOCH};`
- **Space Complexity:** `O(1)`

---

## Java Interview Strategy & Tips

- Watch out for 32-bit integer overflow: `(a + b)` and `(a * b)` can easily exceed `Integer.MAX_VALUE` (2^31 - 1). Cast to `long` before multiplication: `(long) a * b % MOD`.
- For arbitrary precision arithmetic, use `java.math.BigInteger` and `java.math.BigDecimal`.

### Rust vs. Java Perspective
- Rust panics on debug integer overflow and wraps in release mode (or offers `checked_add`, `saturating_mul`).
- Java silently overflows integer operations without exception unless `Math.addExact()` is explicitly used.
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`quadratic_residue.rs`](../../../src/math/quadratic_residue.rs).

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
cargo test --lib math::quadratic_residue
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`quadratic_residue.rs`](../../../src/math/quadratic_residue.rs).
