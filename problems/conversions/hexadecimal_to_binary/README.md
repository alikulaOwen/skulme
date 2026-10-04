# Hexadecimal To Binary

**Category:** `conversions` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

Author : cyrixninja
Hexadecimal to Binary Converter : Converts Hexadecimal to Binary
Wikipedia References  : 1. https://en.wikipedia.org/wiki/Hexadecimal
2. https://en.wikipedia.org/wiki/Binary_number
Other References for Testing : https://www.rapidtables.com/convert/number/hex-to-binary.html

### Original Rust Signatures
```rust
pub fn hexadecimal_to_binary(hex_str: &str) -> Result<String, String>;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`hexadecimal_to_binary.rs`](../../../src/conversions/hexadecimal_to_binary.rs).

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
cargo test --lib conversions::hexadecimal_to_binary
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`hexadecimal_to_binary.rs`](../../../src/conversions/hexadecimal_to_binary.rs).
