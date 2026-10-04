# Random Forest

**Category:** `machine_learning` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

/ Train a single decision tree on a bootstrap sample with random feature subset

### Original Rust Signatures
```rust
pub fn fit(training_data: Vec<(Vec<f64>, f64);
pub fn predict(&self, test_point: &[f64]) -> Option<f64>;
pub fn predict_batch(&self, test_points: &[Vec<f64>]) -> Vec<Option<f64>>;
pub fn random_forest(training_data: Vec<(Vec<f64>, f64);
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`random_forest.rs`](../../../src/machine_learning/random_forest.rs).

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
cargo test --lib machine_learning::random_forest
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`random_forest.rs`](../../../src/machine_learning/random_forest.rs).
