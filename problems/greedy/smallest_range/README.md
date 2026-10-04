# Smallest Range

**Category:** `greedy` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

# Smallest Range Covering Elements from K Lists

Given `k` sorted integer lists, finds the smallest range `[lo, hi]` such
that at least one element from every list lies within that range.

## Algorithm

A min-heap is seeded with the first element of each list.  On every
iteration the heap yields the current global minimum; the global maximum is
maintained separately.  If `[min, max]` is tighter than the best range seen
so far, it is recorded.  The minimum is then replaced by the next element
from the same list.  The loop stops as soon as any list is exhausted,
because no further range can cover all lists.

## References

- <https://en.wikipedia.org/wiki/Priority_queue>

### Original Rust Signatures
```rust
pub fn smallest_range(nums: &[&[i64]]) -> Option<[i64; 2]>;
```

### Complexity
- **Time Complexity:** ``O(n log k)` where `n` is the total number of elements`
- **Space Complexity:** ``O(k)` for the heap.`

---

## Java Interview Strategy & Tips

- Greedy algorithms usually require sorting input first (e.g. by end-time in interval scheduling) or using a `PriorityQueue`.
- In interviews, you must be able to justify why the greedy choice property holds and does not get trapped in local optima.

### Rust vs. Java Perspective
- Greedy logic translates directly between languages; differences lie only in sorting collections and priority queue APIs.
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`smallest_range.rs`](../../../src/greedy/smallest_range.rs).

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
cargo test --lib greedy::smallest_range
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`smallest_range.rs`](../../../src/greedy/smallest_range.rs).
