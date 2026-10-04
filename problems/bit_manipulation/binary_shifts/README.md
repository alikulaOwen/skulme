# Binary Shifts

**Category:** `bit_manipulation` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

Binary Shift Operations

This module provides implementations of various binary shift operations with
binary string output for visualization.

# Shift Types

- **Logical Left Shift**: Shifts bits left, filling with zeros on the right
- **Logical Right Shift**: Shifts bits right, filling with zeros on the left
- **Arithmetic Left Shift**: Same as logical left shift (included for completeness)
- **Arithmetic Right Shift**: Shifts bits right, preserving the sign bit

# Note on Arithmetic vs Logical Left Shifts

In most systems, arithmetic left shift and logical left shift are identical operations.
Both shift bits to the left and fill with zeros on the right. The distinction between
arithmetic and logical shifts only matters for right shifts, where arithmetic shifts
preserve the sign bit.

# References

- [Bitwise Operations - Python Docs](https://docs.python.org/3/library/stdtypes.html#bitwise-operations-on-integer-types)
- [Bit Shift - Interview Cake](https://www.interviewcake.com/concept/java/bit-shift)

### Original Rust Signatures
```rust
pub fn logical_left_shift(number: i32, shift_amount: i32) -> Result<String, String>;
pub fn logical_right_shift(number: i32, shift_amount: i32) -> Result<String, String>;
pub fn arithmetic_right_shift(number: i32, shift_amount: i32) -> Result<String, String>;
pub fn arithmetic_left_shift(number: i32, shift_amount: i32) -> Result<String, String>;
```

### Complexity
- **Time Complexity:** `O(N)`
- **Space Complexity:** `O(1)`

---

## Java Interview Strategy & Tips

- Note the bit shift difference:
  - `>>` is Arithmetic Right Shift (preserves sign bit).
  - `>>>` is Logical (Unsigned) Right Shift (fills left with zeros).
- Useful bit hacks:
  - `n & (n - 1)` removes the lowest set bit.
  - `n & (-n)` isolates the lowest set bit.
  - `n ^ n = 0` (used for single number / unique element finding).
- Built-ins: `Integer.bitCount(n)`, `Integer.highestOneBit(n)`, `Integer.numberOfLeadingZeros(n)`.

### Rust vs. Java Perspective
- Rust has explicit unsigned integer types (`u32`, `u64`), so `>>` is automatically unsigned for unsigned types.
- Java only has signed integers (`int`, `long`), which is why the `>>>` operator exists.
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`binary_shifts.rs`](../../../src/bit_manipulation/binary_shifts.rs).

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
cargo test --lib bit_manipulation::binary_shifts
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`binary_shifts.rs`](../../../src/bit_manipulation/binary_shifts.rs).
