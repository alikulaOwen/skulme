# Sha2

**Category:** `hashing` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

SHA-2 (Secure Hash Algorithm 2) family of cryptographic hash functions.

Designed by the NSA and published by NIST in 2001 (FIPS PUB 180-4).
Built on the Merkle–Damgård construction with a Davies–Meyer compression
function. The family includes six variants differentiated by digest size
and internal word width:

| Function    | Word | Rounds | Digest |
|-------------|------|--------|--------|
| SHA-224     |  32  |   64   |  224   |
| SHA-256     |  32  |   64   |  256   |
| SHA-384     |  64  |   80   |  384   |
| SHA-512     |  64  |   80   |  512   |
| SHA-512/224 |  64  |   80   |  224   |
| SHA-512/256 |  64  |   80   |  256   |

Reference: <https://doi.org/10.6028/NIST.FIPS.180-4>

### Original Rust Signatures
```rust
pub fn sha256(msg: &[u8]) -> [u8; 32];
pub fn sha224(msg: &[u8]) -> [u8; 28];
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`sha2.rs`](../../../src/hashing/sha2.rs).

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
cargo test --lib hashing::sha2
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`sha2.rs`](../../../src/hashing/sha2.rs).
