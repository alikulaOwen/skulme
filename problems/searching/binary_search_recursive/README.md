# Binary Search Recursive

**Category:** `searching` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

Recursively performs a binary search for a specified item within a sorted array.

This function can handle both ascending and descending ordered arrays. It
takes a reference to the item to search for and a slice of the array. If
the item is found, it returns the index of the item within the array. If
the item is not found, it returns `None`.

# Parameters

- `item`: A reference to the item to search for.
- `arr`: A slice of the sorted array in which to search.
- `left`: The left bound of the current search range.
- `right`: The right bound of the current search range.
- `is_asc`: A boolean indicating whether the array is sorted in ascending order.

# Returns

An `Option<usize>` which is:
- `Some(index)` if the item is found at the given index.
- `None` if the item is not found in the array.

### Original Rust Signatures
```rust
pub fn binary_search_rec(item: &T, arr: &[T], left: usize, right: usize) -> Option<usize>;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`binary_search_recursive.rs`](../../../src/searching/binary_search_recursive.rs).

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
cargo test --lib searching::binary_search_recursive
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`binary_search_recursive.rs`](../../../src/searching/binary_search_recursive.rs).
