# Mean Squared Error Loss

**Category:** `machine_learning` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

# Mean Square Loss Function

The `mse_loss` function calculates the Mean Square Error loss, which is a
robust loss function used in machine learning.

## Formula

For a pair of actual and predicted values, represented as vectors `actual`
and `predicted`, the Mean Square  loss is calculated as:

- loss = `(actual - predicted)^2 / n_elements`.

It returns the average loss by dividing the `total_loss` by total no. of
elements.

### Original Rust Signatures
```rust
pub fn mse_loss(predicted: &[f64], actual: &[f64]) -> f64;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`mean_squared_error_loss.rs`](../../../../src/machine_learning/loss_function/mean_squared_error_loss.rs).

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
cargo test --lib machine_learning::loss_function::mean_squared_error_loss
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`mean_squared_error_loss.rs`](../../../../src/machine_learning/loss_function/mean_squared_error_loss.rs).
