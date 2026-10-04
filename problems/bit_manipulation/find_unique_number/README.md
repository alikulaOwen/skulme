# Find Unique Number

**Category:** `bit_manipulation` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

Finds the unique number in a slice where every other element appears twice.

This function uses the XOR bitwise operation. Since XOR has the property that
`a ^ a = 0` and `a ^ 0 = a`, all paired numbers cancel out, leaving only the
unique number.

# Arguments

* `arr` - A slice of integers where all elements except one appear exactly twice

# Returns

* `Ok(i32)` - The unique number that appears only once
* `Err(String)` - An error message if the input is empty

# Examples

```
# use the_algorithms_rust::bit_manipulation::find_unique_number;
assert_eq!(find_unique_number(&[1, 1, 2, 2, 3]).unwrap(), 3);
assert_eq!(find_unique_number(&[4, 5, 4, 6, 6]).unwrap(), 5);
assert_eq!(find_unique_number(&[7]).unwrap(), 7);
assert_eq!(find_unique_number(&[10, 20, 10]).unwrap(), 20);
assert!(find_unique_number(&[]).is_err());
```

### Original Rust Signatures
```rust
pub fn find_unique_number(arr: &[i32]) -> Result<i32, String>;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`find_unique_number.rs`](../../../src/bit_manipulation/find_unique_number.rs).

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
cargo test --lib bit_manipulation::find_unique_number
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`find_unique_number.rs`](../../../src/bit_manipulation/find_unique_number.rs).
