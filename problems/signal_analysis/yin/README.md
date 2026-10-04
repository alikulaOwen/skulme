# Yin

**Category:** `signal_analysis` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

Can't find frequency 440 between 500 and 700

### Original Rust Signatures
```rust
pub fn get_frequency(&self) -> f64;
pub fn get_frequency_with_interpolation(&self) -> f64;
pub fn init(threshold: f64,
        min_expected_frequency: f64,
        max_expected_frequency: f64,
        sample_rate: f64,) -> Yin;
pub fn yin(&self, frequencies: &[f64]) -> Result<YinResult, String>;
```

### Complexity
- **Time Complexity:** `O(N)`
- **Space Complexity:** `O(1)`

---

## Java Interview Strategy & Tips

- Analyze time and space complexity before coding.
- Consider edge cases: empty input, single element, negative numbers, extreme values.
- Write clean, idiomatic Java with proper class naming and methods.

### Rust vs. Java Perspective
- Compare memory management: Rust ownership/borrowing vs Java garbage-collected references.
- Compare error handling: Rust `Result`/`Option` vs Java exceptions/`null`.
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`yin.rs`](../../../src/signal_analysis/yin.rs).

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
cargo test --lib signal_analysis::yin
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`yin.rs`](../../../src/signal_analysis/yin.rs).
