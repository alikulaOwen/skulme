# Manacher

**Category:** `string` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

MEMO: We need to detect odd palindrome as well,
therefore, inserting dummy string so that
we can find a pair with dummy center character.

### Original Rust Signatures
```rust
pub fn manacher(s: String) -> String;
```

### Complexity
- **Time Complexity:** `O(N)`
- **Space Complexity:** `O(1)`

---

## Java Interview Strategy & Tips

- Strings in Java are immutable! `str += "a"` creates a brand new String object each time. Inside loops, ALWAYS use `StringBuilder`.
- Access characters via `str.charAt(i)` and length via `str.length()`.
- Compare strings with `str1.equals(str2)`, NEVER with `str1 == str2` (which checks reference equality!).
- To convert to char array for fast swaps: `char[] chars = str.toCharArray()`.

### Rust vs. Java Perspective
- Rust strings are UTF-8 bytes (`String`, `&str`), so direct byte indexing `s[i]` is prohibited if characters span multiple bytes.
- Java strings use UTF-16 code units (`char`), and provide O(1) indexed `charAt(i)`.
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`manacher.rs`](../../../src/string/manacher.rs).

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
cargo test --lib string::manacher
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`manacher.rs`](../../../src/string/manacher.rs).
