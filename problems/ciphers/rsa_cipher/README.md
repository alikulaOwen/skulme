# RSA Cipher

**Category:** `ciphers` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

RSA Cipher Implementation

This module provides a basic implementation of the RSA (Rivest-Shamir-Adleman) encryption algorithm.
RSA is an asymmetric cryptographic algorithm that uses a pair of keys: public and private.

# Warning

This is an educational implementation and should NOT be used for production cryptography.
Use established cryptographic libraries like `ring` or `rust-crypto` for real-world applications.

# Examples

```
use the_algorithms_rust::ciphers::{generate_keypair, encrypt, decrypt};

let (public_key, private_key) = generate_keypair(61, 53);
let message = 65;
let encrypted = encrypt(message, &public_key);
let decrypted = decrypt(encrypted, &private_key);
assert_eq!(message, decrypted);
```

### Original Rust Signatures
```rust
pub fn generate_keypair(p: u64, q: u64) -> (PublicKey, PrivateKey);
pub fn encrypt(message: u64, public_key: &PublicKey) -> u64;
pub fn decrypt(ciphertext: u64, private_key: &PrivateKey) -> u64;
pub fn encrypt_text(message: &str, public_key: &PublicKey) -> Vec<u64>;
pub fn decrypt_text(ciphertext: &[u64], private_key: &PrivateKey) -> String;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`rsa_cipher.rs`](../../../src/ciphers/rsa_cipher.rs).

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
cargo test --lib ciphers::rsa_cipher
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`rsa_cipher.rs`](../../../src/ciphers/rsa_cipher.rs).
