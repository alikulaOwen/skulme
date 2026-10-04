# Binary Exponentiation

**Category:** `math` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

Binary exponentiation is an algorithm to compute a power in O(logN) where N is the power.

For example, to naively compute n^100, we multiply n 99 times for a O(N) algorithm.

With binary exponentiation we can reduce the number of muliplications by only finding the binary
exponents. n^100 = n^64 * n^32 * n^4. We can compute n^64 by ((((n^2)^2)^2)...), which is
logN multiplications.

We know which binary exponents to add by looking at the set bits in the power. For 100, we know
the bits for 64, 32, and 4 are set.

Computes n^p

### Original Rust Signatures
```rust
pub fn binary_exponentiation(mut n: u64, mut p: u32) -> u64;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`binary_exponentiation.rs`](../../../src/math/binary_exponentiation.rs).

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
cargo test --lib math::binary_exponentiation
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`binary_exponentiation.rs`](../../../src/math/binary_exponentiation.rs).
