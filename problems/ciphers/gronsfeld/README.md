# Gronsfeld

**Category:** `ciphers` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

Gronsfeld Cipher

# Algorithm

A variant of the Vigenère cipher where the key is a sequence of digits (0–9).
Each ASCII alphabetic character in the plaintext is shifted forward (encrypt) or
backward (decrypt) by the value of the corresponding key digit, cycling
through the key. Non-alphabetic characters are passed through unchanged.

### Original Rust Signatures
```rust
pub fn gronsfeld_encrypt(text: &str, key: &str) -> Result<String, &'static str>;
pub fn gronsfeld_decrypt(text: &str, key: &str) -> Result<String, &'static str>;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`gronsfeld.rs`](../../../src/ciphers/gronsfeld.rs).

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
cargo test --lib ciphers::gronsfeld
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`gronsfeld.rs`](../../../src/ciphers/gronsfeld.rs).
