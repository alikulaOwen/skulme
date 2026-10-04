# Subarray Sum Equals K

**Category:** `general` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

/ Counts the number of contiguous subarrays that sum to exactly k.
/
/ # Parameters
/
/ - `nums`: A slice of integers
/ - `k`: The target sum
/
/ # Returns
/
/ The number of contiguous subarrays with sum equal to k.
/
/ # Complexity
/
/ - Time: O(n)
/ - Space: O(n)

### Original Rust Signatures
```rust
pub fn subarray_sum_equals_k(nums: &[i32], k: i32) -> i32;
```

### Complexity
- **Time Complexity:** `O(n)`
- **Space Complexity:** `O(n)`

---

## Java Interview Strategy & Tips

- Analyze time and space complexity before coding.
- Consider edge cases: empty input, single element, negative numbers, extreme values.
- Write clean, idiomatic Java with proper class naming and methods.

### Rust vs. Java Perspective
- Compare memory management: Rust ownership/borrowing vs Java garbage-collected references.
- Compare error handling: Rust `Result`/`Option` vs Java exceptions/`null`.
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`subarray_sum_equals_k.rs`](../../../src/general/subarray_sum_equals_k.rs).

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
cargo test --lib general::subarray_sum_equals_k
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`subarray_sum_equals_k.rs`](../../../src/general/subarray_sum_equals_k.rs).
