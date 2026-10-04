# Tim Sort

**Category:** `sorting` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

Implements Tim sort algorithm.

Tim sort is a hybrid sorting algorithm derived from merge sort and insertion sort.
It is designed to perform well on many kinds of real-world data.

### Original Rust Signatures
```rust
pub fn tim_sort(arr: &mut [T]);
```

### Complexity
- **Time Complexity:** `O(N)`
- **Space Complexity:** `O(1)`

---

## Java Interview Strategy & Tips

- Java's `Arrays.sort(primitive[])` uses Dual-Pivot Quicksort (O(N log N) average, O(N^2) worst case).
- `Arrays.sort(Object[])` and `Collections.sort(List)` use Timsort (guaranteed stable, O(N log N) worst case).
- For custom ordering, use `Comparator.comparingInt(...)` or `(a, b) -> Integer.compare(a, b)`. NEVER use `a - b` due to integer underflow/overflow risk.
- Remember stability: stable sorts preserve the relative order of duplicate elements.

### Rust vs. Java Perspective
- Rust's `slice::sort()` is stable Timsort/pdqsort; `slice::sort_unstable()` is in-place pattern-defeating quicksort.
- Java distinguishes primitive arrays (`int[]`) from object arrays (`Integer[]`), whereas Rust treats all types uniformly under generics `T: Ord`.
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`tim_sort.rs`](../../../src/sorting/tim_sort.rs).

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
cargo test --lib sorting::tim_sort
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`tim_sort.rs`](../../../src/sorting/tim_sort.rs).
