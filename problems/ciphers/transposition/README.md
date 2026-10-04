# Transposition

**Category:** `ciphers` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

Transposition Cipher

The Transposition Cipher is a method of encryption by which a message is shifted
according to a regular system, so that the ciphertext is a rearrangement of the
original message. The most commonly referred to Transposition Cipher is the
COLUMNAR TRANSPOSITION cipher, which is demonstrated below.

### Original Rust Signatures
```rust
pub fn transposition(decrypt_mode: bool, msg: &str, key: &str) -> String;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`transposition.rs`](../../../src/ciphers/transposition.rs).

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
cargo test --lib ciphers::transposition
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`transposition.rs`](../../../src/ciphers/transposition.rs).
