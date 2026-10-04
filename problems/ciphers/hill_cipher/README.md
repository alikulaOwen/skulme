# Hill Cipher

**Category:** `ciphers` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

Hill Cipher

The Hill Cipher is a polygraphic substitution cipher based on linear algebra.

# Algorithm

Let the order of the encryption key be N (as it is a square matrix).
The text is divided into batches of length N and converted to numerical vectors
by a simple mapping starting with A=0 and so on.

The key matrix is multiplied with the batch vector to obtain the encoded vector.
After multiplication, modular 36 calculations map results to alphanumerics.

For decryption, the modular inverse of the encryption key is computed and used
with the same process to recover the original message.

# Constraints

The determinant of the encryption key matrix must be coprime with 36.

# Note

- Only alphanumeric characters are considered
- Text is padded to a multiple of the key size using the last character
- Decrypted text may have padding characters at the end

# References

- <https://apprendre-en-ligne.net/crypto/hill/Hillciph.pdf>
- <https://www.youtube.com/watch?v=kfmNeskzs2o>

### Original Rust Signatures
```rust
pub fn new(mut encrypt_key: Vec<Vec<i32>>) -> Result<Self, String>;
pub fn encrypt(&self, text: &str) -> String;
pub fn decrypt(&self, text: &str) -> String;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`hill_cipher.rs`](../../../src/ciphers/hill_cipher.rs).

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
cargo test --lib ciphers::hill_cipher
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`hill_cipher.rs`](../../../src/ciphers/hill_cipher.rs).
