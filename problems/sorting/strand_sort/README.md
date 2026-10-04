# Strand Sort

**Category:** `sorting` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

# Strand Sort

Strand Sort is a comparison-based sorting algorithm that works by repeatedly
extracting increasing subsequences ("strands") from the input and merging
them into a growing result list.

## Algorithm
1. Remove the first element of the remaining input and start a new *strand*.
2. Scan the rest of the input left-to-right; whenever an element is ≥ the
last element of the strand, pull it out of the input and append it to the
strand.  One full pass yields one sorted strand.
3. Merge the strand into the accumulated result via a standard two-way merge.
4. Repeat until the input is empty.

## Complexity

| Case    | Time   | Space |
|---------|--------|-------|
| Best    | O(n)   | O(n)  |
| Average | O(n²)  | O(n)  |
| Worst   | O(n²)  | O(n)  |

The best case occurs when the input is already sorted (one strand, one merge).
The worst case occurs when the input is reverse-sorted (n strands of length 1).

## Reference
- [Wikipedia: Strand sort](https://en.wikipedia.org/wiki/Strand_sort)

### Original Rust Signatures
```rust
pub fn strand_sort(arr: &mut Vec<T>);
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`strand_sort.rs`](../../../src/sorting/strand_sort.rs).

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
cargo test --lib sorting::strand_sort
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`strand_sort.rs`](../../../src/sorting/strand_sort.rs).
