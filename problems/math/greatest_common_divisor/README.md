# Greatest Common Divisor

**Category:** `math` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

/ Greatest Common Divisor.
/
/ greatest_common_divisor(num1, num2) returns the greatest number of num1 and num2.
/
/ Wikipedia reference: https://en.wikipedia.org/wiki/Greatest_common_divisor
/ gcd(a, b) = gcd(a, -b) = gcd(-a, b) = gcd(-a, -b) by definition of divisibility

### Original Rust Signatures
```rust
pub fn greatest_common_divisor_recursive(a: i64, b: i64) -> i64;
pub fn greatest_common_divisor_iterative(mut a: i64, mut b: i64) -> i64;
pub fn greatest_common_divisor_stein(a: u64, b: u64) -> u64;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`greatest_common_divisor.rs`](../../../src/math/greatest_common_divisor.rs).

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
cargo test --lib math::greatest_common_divisor
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`greatest_common_divisor.rs`](../../../src/math/greatest_common_divisor.rs).
