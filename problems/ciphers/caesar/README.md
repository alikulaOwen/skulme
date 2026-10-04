# Caesar

**Category:** `ciphers` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

Encrypts a given text using the Caesar cipher technique.

In cryptography, a Caesar cipher, also known as Caesar's cipher, the shift cipher, Caesar's code,
or Caesar shift, is one of the simplest and most widely known encryption techniques.
It is a type of substitution cipher in which each letter in the plaintext is replaced by a letter
some fixed number of positions down the alphabet.

# Arguments

* `text` - The text to be encrypted.
* `rotation` - The number of rotations (shift) to be applied. It should be within the range [0, 25].

# Returns

Returns a `Result` containing the encrypted string if successful, or an error message if the rotation
is out of the valid range.

# Errors

Returns an error if the rotation value is out of the valid range [0, 25]

### Original Rust Signatures
```rust
pub fn caesar(text: &str, rotation: isize) -> Result<String, &'static str>;
```

### Complexity
- **Time Complexity:** `O(N)`
- **Space Complexity:** `("Hello, World!", 13, "Uryyb, Jbeyq!"),`

---

## Java Interview Strategy & Tips

- Analyze time and space complexity before coding.
- Consider edge cases: empty input, single element, negative numbers, extreme values.
- Write clean, idiomatic Java with proper class naming and methods.

### Rust vs. Java Perspective
- Compare memory management: Rust ownership/borrowing vs Java garbage-collected references.
- Compare error handling: Rust `Result`/`Option` vs Java exceptions/`null`.
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`caesar.rs`](../../../src/ciphers/caesar.rs).

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
cargo test --lib ciphers::caesar
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`caesar.rs`](../../../src/ciphers/caesar.rs).
