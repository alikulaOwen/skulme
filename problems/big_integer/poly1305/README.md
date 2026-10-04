# Poly1305

**Category:** `big_integer` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

/ The accumulator

### Original Rust Signatures
```rust
pub fn new() -> Self;
pub fn clamp_r(&mut self);
pub fn set_key(&mut self, key: &[u8; 32]);
pub fn add_msg(&mut self, msg: &[u8; 16], msg_bytes: u64);
pub fn get_tag(&self) -> Vec<u8>;
```

### Complexity
- **Time Complexity:** `O(N)`
- **Space Complexity:** `O(1)`

---

## Java Interview Strategy & Tips

- Analyze time and space complexity before coding.
- Consider edge cases: empty input, single element, negative numbers, extreme values.
- Write clean, idiomatic Java with proper class naming and methods.

### Rust vs. Java Perspective
- Compare memory management: Rust ownership/borrowing vs Java garbage-collected references.
- Compare error handling: Rust `Result`/`Option` vs Java exceptions/`null`.
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`poly1305.rs`](../../../src/big_integer/poly1305.rs).

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
cargo test --lib big_integer::poly1305
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`poly1305.rs`](../../../src/big_integer/poly1305.rs).
