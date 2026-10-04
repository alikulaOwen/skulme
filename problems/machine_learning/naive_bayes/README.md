# Naive Bayes

**Category:** `machine_learning` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

/ Naive Bayes classifier for classification tasks.
/ This implementation uses Gaussian Naive Bayes, which assumes that
/ features follow a normal (Gaussian) distribution.
/ The algorithm calculates class priors and feature statistics (mean and variance)
/ for each class, then uses Bayes' theorem to predict class probabilities.

### Original Rust Signatures
```rust
pub fn train_naive_bayes(training_data: Vec<(Vec<f64>, f64);
pub fn predict_naive_bayes(model: &[ClassStatistics], test_point: &[f64]) -> Option<f64>;
pub fn naive_bayes(training_data: Vec<(Vec<f64>, f64);
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`naive_bayes.rs`](../../../src/machine_learning/naive_bayes.rs).

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
cargo test --lib machine_learning::naive_bayes
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`naive_bayes.rs`](../../../src/machine_learning/naive_bayes.rs).
