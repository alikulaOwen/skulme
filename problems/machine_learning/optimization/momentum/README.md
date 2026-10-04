# Momentum

**Category:** `machine_learning` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

/ Momentum Optimization
/
/ Momentum is an extension of gradient descent that accelerates convergence by accumulating
/ a velocity vector in directions of persistent reduction in the objective function.
/ This helps the optimizer navigate ravines and avoid getting stuck in local minima.
/
/ The algorithm maintains a velocity vector that accumulates exponentially decaying moving
/ averages of past gradients. This allows the optimizer to build up speed in consistent
/ directions while dampening oscillations.
/
/ The update equations are:
/ velocity_{k+1} = beta * velocity_k + gradient_of_function(x_k)
/ x_{k+1} = x_k - learning_rate * velocity_{k+1}
/
/ where beta (typically 0.9) controls how much past gradients influence the current update.
/
/ # Arguments
/
/ * `derivative_fn` - The function that calculates the gradient of the objective function at a given point.
/ * `x` - The initial parameter vector to be optimized.
/ * `learning_rate` - Step size for each iteration.
/ * `beta` - Momentum coefficient (typically 0.9). Higher values give more weight to past gradients.
/ * `num_iterations` - The number of iterations to run the optimization.
/
/ # Returns
/
/ A reference to the optimized parameter vector `x`.

### Original Rust Signatures
```rust
pub fn momentum(derivative: impl Fn(&[f64]) -> Vec<f64>,
    x: &mut Vec<f64>,
    learning_rate: f64,
    beta: f64,
    num_iterations: i32,
) -> &mut Vec<f64>;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`momentum.rs`](../../../../src/machine_learning/optimization/momentum.rs).

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
cargo test --lib machine_learning::optimization::momentum
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`momentum.rs`](../../../../src/machine_learning/optimization/momentum.rs).
