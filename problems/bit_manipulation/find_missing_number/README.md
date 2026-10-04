# Find Missing Number

**Category:** `bit_manipulation` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

Finds the missing number in a slice of consecutive integers.

This function uses XOR bitwise operation to find the missing number.
It XORs all expected numbers in the range [min, max] with the actual
numbers present in the array. Since XOR has the property that `a ^ a = 0`,
all present numbers cancel out, leaving only the missing number.

# Arguments

* `nums` - A slice of integers forming a sequence with one missing number

# Returns

* `Ok(i32)` - The missing number in the sequence
* `Err(String)` - An error message if the input is invalid

# Examples

```
# use the_algorithms_rust::bit_manipulation::find_missing_number;
assert_eq!(find_missing_number(&[0, 1, 3, 4]).unwrap(), 2);
assert_eq!(find_missing_number(&[4, 3, 1, 0]).unwrap(), 2);
assert_eq!(find_missing_number(&[-4, -3, -1, 0]).unwrap(), -2);
assert_eq!(find_missing_number(&[-2, 2, 1, 3, 0]).unwrap(), -1);
assert_eq!(find_missing_number(&[1, 3, 4, 5, 6]).unwrap(), 2);
```

### Original Rust Signatures
```rust
pub fn find_missing_number(nums: &[i32]) -> Result<i32, String>;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`find_missing_number.rs`](../../../src/bit_manipulation/find_missing_number.rs).

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
cargo test --lib bit_manipulation::find_missing_number
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`find_missing_number.rs`](../../../src/bit_manipulation/find_missing_number.rs).
