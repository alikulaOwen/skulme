# Binary Count Trailing Zeros

**Category:** `bit_manipulation` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

Counts the number of trailing zeros in the binary representation of a number

# Arguments

* `num` - The input number

# Returns

The number of trailing zeros in the binary representation

# Examples

```
use the_algorithms_rust::bit_manipulation::binary_count_trailing_zeros;

assert_eq!(binary_count_trailing_zeros(25), 0);
assert_eq!(binary_count_trailing_zeros(36), 2);
assert_eq!(binary_count_trailing_zeros(16), 4);
assert_eq!(binary_count_trailing_zeros(58), 1);
```

### Original Rust Signatures
```rust
pub fn binary_count_trailing_zeros(num: u64) -> u32;
pub fn binary_count_trailing_zeros_bitwise(num: u64) -> u32;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`binary_count_trailing_zeros.rs`](../../../src/bit_manipulation/binary_count_trailing_zeros.rs).

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
cargo test --lib bit_manipulation::binary_count_trailing_zeros
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`binary_count_trailing_zeros.rs`](../../../src/bit_manipulation/binary_count_trailing_zeros.rs).
