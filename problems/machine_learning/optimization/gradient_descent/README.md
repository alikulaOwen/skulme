# Gradient Descent

**Category:** `machine_learning` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

/ Gradient Descent Optimization
/
/ Gradient descent is an iterative optimization algorithm used to find the minimum of a function.
/ It works by updating the parameters (in this case, elements of the vector `x`) in the direction of
/ the steepest decrease in the function's value. This is achieved by subtracting the gradient of
/ the function at the current point from the current point. The learning rate controls the step size.
/
/ The equation for a single parameter (univariate) is:
/ x_{k+1} = x_k - learning_rate * derivative_of_function(x_k)
/
/ For multivariate functions, it extends to each parameter:
/ x_{k+1} = x_k - learning_rate * gradient_of_function(x_k)
/
/ # Arguments
/
/ * `derivative_fn` - The function that calculates the gradient of the objective function at a given point.
/ * `x` - The initial parameter vector to be optimized.
/ * `learning_rate` - Step size for each iteration.
/ * `num_iterations` - The number of iterations to run the optimization.
/
/ # Returns
/
/ A reference to the optimized parameter vector `x`.

### Original Rust Signatures
```rust
pub fn gradient_descent(derivative_fn: impl Fn(&[f64]) -> Vec<f64>,
    x: &mut Vec<f64>,
    learning_rate: f64,
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`gradient_descent.rs`](../../../../src/machine_learning/optimization/gradient_descent.rs).

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
cargo test --lib machine_learning::optimization::gradient_descent
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`gradient_descent.rs`](../../../../src/machine_learning/optimization/gradient_descent.rs).
