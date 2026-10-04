# Peak Signal To Noise Ratio

**Category:** `compression` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

# Peak Signal-to-Noise Ratio (PSNR)

Measures the quality of a reconstructed or compressed image relative to the original.
A higher PSNR generally indicates better quality.

Reference: <https://en.wikipedia.org/wiki/Peak_signal-to-noise_ratio>

### Original Rust Signatures
```rust
pub fn peak_signal_to_noise_ratio(original: &[u8], compressed: &[u8]) -> f64;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`peak_signal_to_noise_ratio.rs`](../../../src/compression/peak_signal_to_noise_ratio.rs).

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
cargo test --lib compression::peak_signal_to_noise_ratio
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`peak_signal_to_noise_ratio.rs`](../../../src/compression/peak_signal_to_noise_ratio.rs).
