# Find Previous Power Of Two

**Category:** `bit_manipulation` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

Previous Power of Two

This module provides a function to find the largest power of two that is less than
or equal to a given non-negative integer.

# Algorithm

The algorithm works by repeatedly left-shifting (doubling) a power value starting
from 1 until it exceeds the input number, then returning the previous power (by
right-shifting once).

For more information: <https://stackoverflow.com/questions/1322510>

### Original Rust Signatures
```rust
pub fn find_previous_power_of_two(number: i32) -> Result<u32, String>;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`find_previous_power_of_two.rs`](../../../src/bit_manipulation/find_previous_power_of_two.rs).

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
cargo test --lib bit_manipulation::find_previous_power_of_two
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`find_previous_power_of_two.rs`](../../../src/bit_manipulation/find_previous_power_of_two.rs).
