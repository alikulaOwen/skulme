# Diffie Hellman

**Category:** `ciphers` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

Based on the TheAlgorithms/Python
RFC 3526 - More Modular Exponential (MODP) Diffie-Hellman groups for
Internet Key Exchange (IKE) https://tools.ietf.org/html/rfc3526

### Original Rust Signatures
```rust
pub fn new(group: Option<u8>) -> Self;
pub fn get_private_key(&self) -> String;
pub fn generate_public_key(&mut self) -> String;
pub fn is_valid_public_key(&self, key_str: &str) -> bool;
pub fn generate_shared_key(self, other_key_str: &str) -> Option<String>;
```

### Complexity
- **Time Complexity:** `:{SystemTime, UNIX_EPOCH},`
- **Space Complexity:** `O(1)`

---

## Java Interview Strategy & Tips

- Analyze time and space complexity before coding.
- Consider edge cases: empty input, single element, negative numbers, extreme values.
- Write clean, idiomatic Java with proper class naming and methods.

### Rust vs. Java Perspective
- Compare memory management: Rust ownership/borrowing vs Java garbage-collected references.
- Compare error handling: Rust `Result`/`Option` vs Java exceptions/`null`.
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`diffie_hellman.rs`](../../../src/ciphers/diffie_hellman.rs).

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
cargo test --lib ciphers::diffie_hellman
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`diffie_hellman.rs`](../../../src/ciphers/diffie_hellman.rs).
