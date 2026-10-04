# Burrows Wheeler Transform

**Category:** `compression` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

Burrows-Wheeler Transform

The Burrows-Wheeler transform (BWT, also called block-sorting compression)
rearranges a character string into runs of similar characters. This is useful
for compression, since it tends to be easy to compress a string that has runs
of repeated characters by techniques such as move-to-front transform and
run-length encoding. More importantly, the transformation is reversible,
without needing to store any additional data except the position of the first
original character. The BWT is thus a "free" method of improving the efficiency
of text compression algorithms, costing only some extra computation.

More info: <https://en.wikipedia.org/wiki/Burrows%E2%80%93Wheeler_transform>

### Original Rust Signatures
```rust
pub fn all_rotations(s: &str) -> Vec<String>;
pub fn bwt_transform(s: &str) -> BwtResult;
pub fn reverse_bwt(bwt_string: &str, idx_original_string: usize) -> String;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`burrows_wheeler_transform.rs`](../../../src/compression/burrows_wheeler_transform.rs).

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
cargo test --lib compression::burrows_wheeler_transform
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`burrows_wheeler_transform.rs`](../../../src/compression/burrows_wheeler_transform.rs).
