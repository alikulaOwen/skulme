# Vernam

**Category:** `ciphers` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

Vernam Cipher

The Vernam cipher is a symmetric stream cipher where plaintext is combined
with a random or pseudorandom stream of data (the key) of the same length.
This implementation uses the alphabet A-Z with modular arithmetic.

# Algorithm

For encryption: C = (P + K) mod 26
For decryption: P = (C - K) mod 26

Where P is plaintext, K is key, and C is ciphertext (all converted to 0-25 range)

### Original Rust Signatures
```rust
pub fn vernam_encrypt(plaintext: &str, key: &str) -> String;
pub fn vernam_decrypt(ciphertext: &str, key: &str) -> String;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`vernam.rs`](../../../src/ciphers/vernam.rs).

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
cargo test --lib ciphers::vernam
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`vernam.rs`](../../../src/ciphers/vernam.rs).
