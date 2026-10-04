# Roman Numerals

**Category:** `conversions` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

Roman Numeral Conversion

This module provides conversion between Roman numerals and integers.

Roman numerals use combinations of letters from the Latin alphabet:
I, V, X, L, C, D, and M to represent numbers.

# Rules

- I = 1, V = 5, X = 10, L = 50, C = 100, D = 500, M = 1000
- When a smaller value appears before a larger value, subtract the smaller
(e.g., IV = 4, IX = 9)
- When a smaller value appears after a larger value, add the smaller
(e.g., VI = 6, XI = 11)

# References

- [Roman Numerals - Wikipedia](https://en.wikipedia.org/wiki/Roman_numerals)
- [LeetCode #13 - Roman to Integer](https://leetcode.com/problems/roman-to-integer/)

### Original Rust Signatures
```rust
pub fn roman_to_int(roman: &str) -> Result<u32, String>;
pub fn int_to_roman(mut number: u32) -> Result<String, String>;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`roman_numerals.rs`](../../../src/conversions/roman_numerals.rs).

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
cargo test --lib conversions::roman_numerals
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`roman_numerals.rs`](../../../src/conversions/roman_numerals.rs).
