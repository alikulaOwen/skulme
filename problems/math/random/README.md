# Random

**Category:** `math` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

/ `stream` should be less than 1 << 63

### Original Rust Signatures
```rust
pub fn new(seed: u64, multiplier: u64, stream: u64) -> Self;
pub fn new_default(seed: u64) -> Self;
pub fn next(&mut self);
pub fn advance(&mut self, mut delta: u64);
pub fn get_u32(&mut self) -> u32;
pub fn get_u64(&mut self) -> u64;
pub fn get_u16(&mut self) -> (u16, u16);
pub fn get_u8(&mut self) -> (u8, u8, u8, u8);
pub fn get_state(&self) -> u64;
pub fn iter_mut(&mut self) -> IterMut<'_>;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`random.rs`](../../../src/math/random.rs).

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
cargo test --lib math::random
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`random.rs`](../../../src/math/random.rs).
