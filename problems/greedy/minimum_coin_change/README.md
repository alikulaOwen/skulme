# Minimum Coin Change

**Category:** `greedy` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

# Minimum Coin Change (Greedy Algorithm)

This module implements a greedy algorithm to find the minimum number of coins
needed to make change for a given amount using specified denominations.

## Algorithm

The greedy approach works by always selecting the largest denomination possible
at each step. While this approach doesn't guarantee an optimal solution for all
denomination systems, it works correctly for canonical coin systems (like most
real-world currencies including USD, EUR, INR, etc.).

## Time Complexity

O(n) where n is the number of denominations

## Space Complexity

O(m) where m is the number of coins in the result

## Example

```
# fn find_minimum_change(denominations: &[i32], value: i32) -> Vec<i32> {
#     if value <= 0 || denominations.is_empty() {
#         return Vec::new();
#     }
#     let mut remaining_value = value;
#     let mut result = Vec::new();
#     let mut sorted_denominations = denominations.to_vec();
#     sorted_denominations.sort_unstable_by(|a, b| b.cmp(a));
#     for &denomination in &sorted_denominations {
#         while remaining_value >= denomination {
#             remaining_value -= denomination;
#             result.push(denomination);
#         }
#     }
#     result
# }
let denominations = vec![1, 2, 5, 10, 20, 50, 100, 500, 2000];
let result = find_minimum_change(&denominations, 987);
assert_eq!(result, vec![500, 100, 100, 100, 100, 50, 20, 10, 5, 2]);
```

### Original Rust Signatures
```rust
pub fn find_minimum_change(denominations: &[i32], value: i32) -> Vec<i32>;
```

### Complexity
- **Time Complexity:** `O(N)`
- **Space Complexity:** `O(1)`

---

## Java Interview Strategy & Tips

- Greedy algorithms usually require sorting input first (e.g. by end-time in interval scheduling) or using a `PriorityQueue`.
- In interviews, you must be able to justify why the greedy choice property holds and does not get trapped in local optima.

### Rust vs. Java Perspective
- Greedy logic translates directly between languages; differences lie only in sorting collections and priority queue APIs.
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`minimum_coin_change.rs`](../../../src/greedy/minimum_coin_change.rs).

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
cargo test --lib greedy::minimum_coin_change
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`minimum_coin_change.rs`](../../../src/greedy/minimum_coin_change.rs).
