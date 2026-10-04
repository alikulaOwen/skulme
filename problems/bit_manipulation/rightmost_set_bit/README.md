# Rightmost Set Bit

**Category:** `bit_manipulation` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

Finds the index (position) of the rightmost set bit in a number.

The index is 1-based, where position 1 is the least significant bit (rightmost).
This function uses the bitwise trick `n & -n` to isolate the rightmost set bit,
then calculates its position using logarithm base 2.

# Algorithm

1. Use `n & -n` to isolate the rightmost set bit
2. Calculate log2 of the result to get the 0-based position
3. Add 1 to convert to 1-based indexing

# Arguments

* `num` - A positive integer

# Returns

* `Ok(u32)` - The 1-based position of the rightmost set bit
* `Err(String)` - An error message if the input is invalid

# Examples

```
# use the_algorithms_rust::bit_manipulation::index_of_rightmost_set_bit;
// 18 in binary: 10010, rightmost set bit is at position 2
assert_eq!(index_of_rightmost_set_bit(18).unwrap(), 2);

// 12 in binary: 1100, rightmost set bit is at position 3
assert_eq!(index_of_rightmost_set_bit(12).unwrap(), 3);

// 5 in binary: 101, rightmost set bit is at position 1
assert_eq!(index_of_rightmost_set_bit(5).unwrap(), 1);

// 16 in binary: 10000, rightmost set bit is at position 5
assert_eq!(index_of_rightmost_set_bit(16).unwrap(), 5);

// 0 has no set bits
assert!(index_of_rightmost_set_bit(0).is_err());
```

### Original Rust Signatures
```rust
pub fn index_of_rightmost_set_bit(num: i32) -> Result<u32, String>;
pub fn index_of_rightmost_set_bit_log(num: i32) -> Result<u32, String>;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`rightmost_set_bit.rs`](../../../src/bit_manipulation/rightmost_set_bit.rs).

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
cargo test --lib bit_manipulation::rightmost_set_bit
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`rightmost_set_bit.rs`](../../../src/bit_manipulation/rightmost_set_bit.rs).
