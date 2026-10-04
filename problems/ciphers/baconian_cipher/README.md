# Baconian Cipher

**Category:** `ciphers` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

Author : cyrixninja
Program to encode and decode Baconian or Bacon's Cipher
Wikipedia reference : https://en.wikipedia.org/wiki/Bacon%27s_cipher
Bacon's cipher or the Baconian cipher is a method of steganographic message encoding devised by Francis Bacon in 1605.
A message is concealed in the presentation of text, rather than its content. Bacon cipher is categorized as both a substitution cipher (in plain code) and a concealment cipher (using the two typefaces).

Encode Baconian Cipher

### Original Rust Signatures
```rust
pub fn baconian_encode(message: &str) -> String;
pub fn baconian_decode(encoded: &str) -> String;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`baconian_cipher.rs`](../../../src/ciphers/baconian_cipher.rs).

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
cargo test --lib ciphers::baconian_cipher
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`baconian_cipher.rs`](../../../src/ciphers/baconian_cipher.rs).
