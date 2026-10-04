# Reverse Bits

**Category:** `bit_manipulation` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

This module provides a function to reverse the bits of a 32-bit unsigned integer.

The algorithm works by iterating through each of the 32 bits from least
significant to most significant, extracting each bit and placing it in the
reverse position.

# Algorithm

For each of the 32 bits:
1. Shift the result left by 1 to make room for the next bit
2. Extract the least significant bit of the input using bitwise AND with 1
3. OR that bit into the result
4. Shift the input right by 1 to process the next bit

# Time Complexity

O(1) - Always processes exactly 32 bits

# Space Complexity

O(1) - Uses a constant amount of extra space

# Example

```
use the_algorithms_rust::bit_manipulation::reverse_bits;

let n = 43261596;  // Binary: 00000010100101000001111010011100
let reversed = reverse_bits(n);
assert_eq!(reversed, 964176192);  // Binary: 00111001011110000010100101000000
```

### Original Rust Signatures
```rust
pub fn reverse_bits(n: u32) -> u32;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`reverse_bits.rs`](../../../src/bit_manipulation/reverse_bits.rs).

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
cargo test --lib bit_manipulation::reverse_bits
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`reverse_bits.rs`](../../../src/bit_manipulation/reverse_bits.rs).
