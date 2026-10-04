# Chacha

**Category:** `ciphers` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

ChaCha20 implementation based on RFC8439

ChaCha20 is a stream cipher developed independently by Daniel J. Bernstein.\
To use it, the `chacha20` function should be called with appropriate
parameters and the output of the function should be XORed with plain text.

`chacha20` function takes as input an array of 16 32-bit integers (512 bits)
of which 128 bits is the constant 'expand 32-byte k', 256 bits is the key,
and 128 bits are nonce and counter. According to RFC8439, the nonce should
be 96 bits long, which leaves 32 bits for the counter. Given that the block
length is 512 bits, this leaves enough counter values to encrypt 256GB of
data.

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

As per the diagram below, `input[0, 1, 2, 3]` are the constants mentioned
above, `input[4..=11]` is filled with the key, and `input[6..=9]` should be
filled with nonce and counter values. The output of the function is stored
in `output` variable and can be XORed with the plain text to produce the
cipher text.

```text
+------+------+------+------+
|      |      |      |      |
| C[0] | C[1] | C[2] | C[3] |
|      |      |      |      |
+------+------+------+------+
|      |      |      |      |
| key0 | key1 | key2 | key3 |
|      |      |      |      |
+------+------+------+------+
|      |      |      |      |
| key4 | key5 | key6 | key7 |
|      |      |      |      |
+------+------+------+------+
|      |      |      |      |
| ctr0 | no.0 | no.1 | no.2 |
|      |      |      |      |
+------+------+------+------+
```

Note that the constants, the key, and the nonce should be written in
little-endian order, meaning that for example if the key is 01:02:03:04
(in hex), it corresponds to the integer `0x04030201`. It is important to
know that the hex value of the counter is meaningless, and only its integer
value matters, and it should start with (for example) `0x00000000`, and then
`0x00000001` and so on until `0xffffffff`. Keep in mind that as soon as we get
from bytes to words, we stop caring about their representation in memory,
and we only need the math to be correct.

The output of the function can be used without any change, as long as the
plain text has the same endianness. For example if the plain text is
"hello world", and the first word of the output is `0x01020304`, then the
first byte of plain text ('h') should be XORed with the least-significant
byte of `0x01020304`, which is `0x04`.

### Original Rust Signatures
```rust
pub fn chacha20(input: &[u32; 16], output: &mut [u32; 16]);
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`chacha.rs`](../../../src/ciphers/chacha.rs).

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
cargo test --lib ciphers::chacha
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`chacha.rs`](../../../src/ciphers/chacha.rs).
