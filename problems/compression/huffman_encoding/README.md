# Huffman Encoding

**Category:** `compression` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

Huffman Encoding implementation

Huffman coding is a lossless data compression algorithm that assigns variable-length codes
to characters based on their frequency of occurrence. Characters that occur more frequently
are assigned shorter codes, while less frequent characters get longer codes.

# Algorithm Overview

1. Count the frequency of each character in the input
2. Build a min-heap (priority queue) of nodes based on frequency
3. Build the Huffman tree by repeatedly:
- Remove two nodes with minimum frequency
- Create a parent node with combined frequency
- Insert the parent back into the heap
4. Traverse the tree to assign binary codes to each character
5. Encode the input using the generated codes

# Time Complexity

- Building frequency map: O(n) where n is input length
- Building Huffman tree: O(m log m) where m is number of unique characters
- Encoding: O(n)

# Usage

As a library:
```no_run
use the_algorithms_rust::compression::huffman_encode;

let text = "hello world";
let (encoded, codes) = huffman_encode(text);
println!("Original: {}", text);
println!("Encoded: {}", encoded);
```

As a command-line tool:
```bash
rustc huffman_encoding.rs -o huffman
./huffman input.txt
```

### Original Rust Signatures
```rust
pub fn huffman_encode(text: &str) -> (String, HashMap<char, String>);
pub fn huffman_decode(encoded: &str, codes: &HashMap<char, String>) -> String;
pub fn demonstrate_huffman_from_file(file_path: &str) -> std::io::Result<()>;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`huffman_encoding.rs`](../../../src/compression/huffman_encoding.rs).

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
cargo test --lib compression::huffman_encoding
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`huffman_encoding.rs`](../../../src/compression/huffman_encoding.rs).
