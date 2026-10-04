# Stable Matching

**Category:** `greedy` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

Update man's next proposal index

### Original Rust Signatures
```rust
pub fn stable_matching(men_preferences: &HashMap<String, Vec<String>>,
    women_preferences: &HashMap<String, Vec<String>>,) -> HashMap<String, String>;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`stable_matching.rs`](../../../src/greedy/stable_matching.rs).

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
cargo test --lib greedy::stable_matching
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`stable_matching.rs`](../../../src/greedy/stable_matching.rs).
