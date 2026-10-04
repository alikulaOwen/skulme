# Word Break

**Category:** `dynamic_programming` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

Checks if a string can be segmented into a space-separated sequence
of one or more words from the given dictionary.

# Arguments
* `s` - The input string to be segmented.
* `word_dict` - A slice of words forming the dictionary.

# Returns
* `bool` - `true` if the string can be segmented, `false` otherwise.

### Original Rust Signatures
```rust
pub fn word_break(s: &str, word_dict: &[&str]) -> bool;
```

### Complexity
- **Time Complexity:** `O(N)`
- **Space Complexity:** `separated sequence`

---

## Java Interview Strategy & Tips

- Multi-dimensional arrays `int[][] dp = new int[m][n]` in Java are arrays of heap references; consider flat arrays `int[m * n]` or rolling 1D arrays for cache locality.
- Watch for integer overflow when initializing memoization tables with `Integer.MAX_VALUE` (adding 1 wraps around to negative). Use `1_000_000_000` or check for sentinel before adding.
- Identify: State definition, Base cases, Transition relation, and Evaluation order.

### Rust vs. Java Perspective
- Rust guarantees memory safety and bounds checks, but idiomatically uses flat vectors `Vec<T>` with 1D indexing.
- Java relies on JVM GC for allocated DP tables, so minimize object allocations inside DP loops.
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`word_break.rs`](../../../src/dynamic_programming/word_break.rs).

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
cargo test --lib dynamic_programming::word_break
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`word_break.rs`](../../../src/dynamic_programming/word_break.rs).
