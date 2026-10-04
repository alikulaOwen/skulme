# LZ77

**Category:** `compression` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

LZ77 Compression Algorithm

LZ77 is a lossless data compression algorithm published by Abraham Lempel and Jacob Ziv in 1977.
Also known as LZ1 or sliding-window compression, it forms the basis for many variations
including LZW, LZSS, LZMA and others.

# Algorithm Overview

It uses a "sliding window" method where the window contains:
- Search buffer: previously seen data that can be referenced
- Look-ahead buffer: data currently being encoded

LZ77 encodes data using triplets (tokens) composed of:
- **Offset**: distance from the current position to the start of a match in the search buffer
- **Length**: number of characters that match
- **Indicator**: the next character to be encoded

# Examples

```
use the_algorithms_rust::compression::LZ77Compressor;

let compressor = LZ77Compressor::new(13, 6);
let text = "ababcbababaa";
let compressed = compressor.compress(text);
let decompressed = compressor.decompress(&compressed);
assert_eq!(text, decompressed);
```

# References

- [Wikipedia: LZ77 and LZ78](https://en.wikipedia.org/wiki/LZ77_and_LZ78)

### Original Rust Signatures
```rust
pub fn new(offset: usize, length: usize, indicator: char) -> Self;
pub fn new(window_size: usize, lookahead_buffer_size: usize) -> Self;
pub fn compress(&self, text: &str) -> Vec<Token>;
pub fn decompress(&self, tokens: &[Token]) -> String;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`lz77.rs`](../../../src/compression/lz77.rs).

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
cargo test --lib compression::lz77
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`lz77.rs`](../../../src/compression/lz77.rs).
