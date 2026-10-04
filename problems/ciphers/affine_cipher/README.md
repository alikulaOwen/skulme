# Affine Cipher

**Category:** `ciphers` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

Affine Cipher

The affine cipher is a type of monoalphabetic substitution cipher where each
character in the alphabet is mapped to its numeric equivalent, encrypted using
a mathematical function, and converted back to a character.

# Algorithm

The encryption function is: `E(x) = (ax + b) mod m`
The decryption function is: `D(x) = a^(-1)(x - b) mod m`

Where:
- `x` is the numeric position of the character
- `a` and `b` are the keys (key_a and key_b)
- `m` is the size of the symbol set
- `a^(-1)` is the modular multiplicative inverse of `a` modulo `m`

# Key Requirements

- `key_a` must be coprime with the symbol set size (gcd(key_a, m) = 1)
- `key_a` must not be 1 (cipher becomes too weak)
- `key_b` must not be 0 (cipher becomes too weak)
- `key_b` must be between 0 and symbol set size - 1

# References

- [Affine Cipher - Wikipedia](https://en.wikipedia.org/wiki/Affine_cipher)

### Original Rust Signatures
```rust
pub fn affine_encrypt(key: usize, message: &str) -> Result<String, String>;
pub fn affine_decrypt(key: usize, message: &str) -> Result<String, String>;
pub fn affine_generate_key() -> usize;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`affine_cipher.rs`](../../../src/ciphers/affine_cipher.rs).

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
cargo test --lib ciphers::affine_cipher
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`affine_cipher.rs`](../../../src/ciphers/affine_cipher.rs).
