# Salsa

**Category:** `ciphers` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

This is a `Salsa20` implementation based on <https://en.wikipedia.org/wiki/Salsa20>\
`Salsa20` is a stream cipher developed by Daniel J. Bernstein.\
To use it, the `salsa20` function should be called with appropriate parameters and the
output of the function should be XORed with plain text.

`salsa20` function takes as input an array of 16 32-bit integers (512 bits)
of which 128 bits is the constant 'expand 32-byte k', 256 bits is the key,
and 128 bits are nonce and counter. It is up to the user to determine how
many bits each of nonce and counter take, but a default of 64 bits each
seems to be a sane choice.

The 16 input numbers can be thought of as the elements of a 4x4 matrix like
the one below, on which we do the main operations of the cipher.

```text
+----+----+----+----+
| 00 | 01 | 02 | 03 |
+----+----+----+----+
| 04 | 05 | 06 | 07 |
+----+----+----+----+
| 08 | 09 | 10 | 11 |
+----+----+----+----+
| 12 | 13 | 14 | 15 |
+----+----+----+----+
```

As per the diagram below, `input[0, 5, 10, 15]` are the constants mentioned
above, `input[1, 2, 3, 4, 11, 12, 13, 14]` is filled with the key, and
`input[6, 7, 8, 9]` should be filled with nonce and counter values. The output
of the function is stored in `output` variable and can be XORed with the
plain text to produce the cipher text.

```text
+------+------+------+------+
|      |      |      |      |
| C[0] | key1 | key2 | key3 |
|      |      |      |      |
+------+------+------+------+
|      |      |      |      |
| key4 | C[1] | no1  | no2  |
|      |      |      |      |
+------+------+------+------+
|      |      |      |      |
| ctr1 | ctr2 | C[2] | key5 |
|      |      |      |      |
+------+------+------+------+
|      |      |      |      |
| key6 | key7 | key8 | C[3] |
|      |      |      |      |
+------+------+------+------+
```

### Original Rust Signatures
```rust
pub fn salsa20(input: &[u32; 16], output: &mut [u32; 16]);
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`salsa.rs`](../../../src/ciphers/salsa.rs).

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
cargo test --lib ciphers::salsa
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`salsa.rs`](../../../src/ciphers/salsa.rs).
