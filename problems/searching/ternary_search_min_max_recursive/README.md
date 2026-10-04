# Ternary Search Min Max Recursive

**Category:** `searching` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

Recursive ternary search algorithm for finding maximum of unimodal function

### Original Rust Signatures
```rust
pub fn ternary_search_max_rec(f: fn(f32) -> f32,
    start: f32,
    end: f32,
    absolute_precision: f32,
) -> f32;
pub fn ternary_search_min_rec(f: fn(f32) -> f32,
    start: f32,
    end: f32,
    absolute_precision: f32,
) -> f32;
```

### Complexity
- **Time Complexity:** `O(N)`
- **Space Complexity:** `O(1)`

---

## Java Interview Strategy & Tips

- In Binary Search, ALWAYS calculate midpoint using `int mid = left + (right - left) / 2;` to avoid 32-bit integer overflow.
- Check loop invariants: `while (left <= right)` when `right = n - 1` vs `while (left < right)` when `right = n`.
- Java provides `Arrays.binarySearch()`, which returns `-(insertion_point + 1)` if the key is not present.

### Rust vs. Java Perspective
- Rust's `slice::binary_search()` returns `Result<usize, usize>` (`Ok(index)` or `Err(insert_index)`).
- Java returns a primitive `int`, encoding not-found as a negative integer.
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`ternary_search_min_max_recursive.rs`](../../../src/searching/ternary_search_min_max_recursive.rs).

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
cargo test --lib searching::ternary_search_min_max_recursive
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`ternary_search_min_max_recursive.rs`](../../../src/searching/ternary_search_min_max_recursive.rs).
