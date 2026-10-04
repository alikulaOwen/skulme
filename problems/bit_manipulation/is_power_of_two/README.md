# Is Power Of Two

**Category:** `bit_manipulation` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

Power of Two Check

This module provides a function to determine if a given positive integer is a power of two
using efficient bit manipulation.

# Algorithm

The algorithm uses the property that powers of two have exactly one bit set in their
binary representation. When we subtract 1 from a power of two, all bits after the single
set bit become 1, and the set bit becomes 0:

```text
n     = 0..100..00  (power of 2)
n - 1 = 0..011..11
n & (n - 1) = 0     (no intersections)
```

For example:
- 8 in binary:  1000
- 7 in binary:  0111
- 8 & 7 = 0000 = 0 ✓

Author: Alexander Pantyukhin
Date: November 1, 2022

### Original Rust Signatures
```rust
pub fn is_power_of_two(number: i32) -> Result<bool, String>;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`is_power_of_two.rs`](../../../src/bit_manipulation/is_power_of_two.rs).

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
cargo test --lib bit_manipulation::is_power_of_two
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`is_power_of_two.rs`](../../../src/bit_manipulation/is_power_of_two.rs).
