# Support Vector Classifier

**Category:** `machine_learning` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

Support Vector Classifier (SVC)

This module implements a Support Vector Machine classifier with support for
linear and RBF (Radial Basis Function) kernels. It uses the dual formulation
of the SVM optimization problem.

# Example
```
use ndarray::array;
use the_algorithms_rust::machine_learning::{SVC, Kernel};

let observations = vec![
array![0.0, 1.0],
array![0.0, 2.0],
array![1.0, 1.0],
array![1.0, 2.0],
];
let classes = array![1.0, 1.0, -1.0, -1.0];

let mut svc = SVC::new(Kernel::Linear, f64::INFINITY).unwrap();
svc.fit(&observations, &classes).unwrap();
assert_eq!(svc.predict(&array![0.0, 1.0]), 1.0);
assert_eq!(svc.predict(&array![1.0, 1.0]), -1.0);
```

### Original Rust Signatures
```rust
pub fn new(kernel: Kernel, regularization: f64) -> Result<Self, SVCError>;
pub fn fit(&mut self,
        observations: &[Array1<f64>],
        classes: &Array1<f64>,) -> Result<(), SVCError>;
pub fn predict(&self, observation: &Array1<f64>) -> f64;
pub fn n_support_vectors(&self) -> usize;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`support_vector_classifier.rs`](../../../src/machine_learning/support_vector_classifier.rs).

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
cargo test --lib machine_learning::support_vector_classifier
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`support_vector_classifier.rs`](../../../src/machine_learning/support_vector_classifier.rs).
