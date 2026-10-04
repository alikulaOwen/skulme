# Huffman Encoding

**Category:** `general` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

For the `value` to overflow, the sum of frequencies should be bigger
than u64. So we should be safe here
/ The encoded value

### Original Rust Signatures
```rust
pub fn get_alphabet(height: u32,
        path: u64,
        node: &HuffmanNode<T>,
        map: &mut BTreeMap<T, HuffmanValue>,);
pub fn new(alphabet: &[(T, u64);
pub fn encode(&self, data: &[T]) -> HuffmanEncoding;
pub fn new() -> Self;
pub fn add_data(&mut self, data: HuffmanValue);
pub fn decode(&self, dict: &HuffmanDictionary<T>) -> Option<Vec<T>>;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`huffman_encoding.rs`](../../../src/general/huffman_encoding.rs).

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
cargo test --lib general::huffman_encoding
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`huffman_encoding.rs`](../../../src/general/huffman_encoding.rs).
