# Swap Odd Even Bits

**Category:** `bit_manipulation` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

Swaps odd and even bits in an integer.

This function separates the even bits (0, 2, 4, 6, etc.) and odd bits (1, 3, 5, 7, etc.)
using bitwise AND operations, then swaps them by shifting and combining with OR.

# Arguments

* `num` - A 32-bit unsigned integer

# Returns

A new integer with odd and even bits swapped

# Examples

```
use the_algorithms_rust::bit_manipulation::swap_odd_even_bits;

assert_eq!(swap_odd_even_bits(0), 0);
assert_eq!(swap_odd_even_bits(1), 2);
assert_eq!(swap_odd_even_bits(2), 1);
assert_eq!(swap_odd_even_bits(3), 3);
assert_eq!(swap_odd_even_bits(4), 8);
assert_eq!(swap_odd_even_bits(5), 10);
assert_eq!(swap_odd_even_bits(6), 9);
assert_eq!(swap_odd_even_bits(23), 43);
```

### Original Rust Signatures
```rust
pub fn swap_odd_even_bits(num: u32) -> u32;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`swap_odd_even_bits.rs`](../../../src/bit_manipulation/swap_odd_even_bits.rs).

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
cargo test --lib bit_manipulation::swap_odd_even_bits
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`swap_odd_even_bits.rs`](../../../src/bit_manipulation/swap_odd_even_bits.rs).
