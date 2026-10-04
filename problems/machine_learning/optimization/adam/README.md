# Adam

**Category:** `machine_learning` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

# Adam (Adaptive Moment Estimation) optimizer

The `Adam (Adaptive Moment Estimation)` optimizer is an adaptive learning rate algorithm used
in gradient descent and machine learning, such as for training neural networks to solve deep
learning problems. Boasting memory-efficient fast convergence rates, it sets and iteratively
updates learning rates individually for each model parameter based on the gradient history.

Setting `weight_decay > 0.0` enables the AdamW variant (Loshchilov & Hutter, 2019), which
applies weight decay directly to the parameters rather than folding it into the gradients.
This keeps the decay rate constant and independent of the gradient history — the key flaw
that AdamW corrects over naive L2 regularization inside Adam. With `weight_decay = 0.0`
(the default), the two algorithms are identical.

## Algorithm:

Given:
- α is the learning rate
- (β_1, β_2) are the exponential decay rates for moment estimates
- ϵ is any small value to prevent division by zero
- λ is the weight decay coefficient (0.0 for standard Adam, > 0.0 for AdamW)
- g_t are the gradients at time step t
- m_t are the biased first moment estimates of the gradient at time step t
- v_t are the biased second raw moment estimates of the gradient at time step t
- θ_t are the model parameters at time step t
- t is the time step

Required:
θ_0

Initialize:
m_0 <- 0
v_0 <- 0
t <- 0

while θ_t not converged do
m_t = β_1 * m_{t−1} + (1 − β_1) * g_t
v_t = β_2 * v_{t−1} + (1 − β_2) * g_t^2
m_hat_t = m_t / (1 - β_1^t)
v_hat_t = v_t / (1 - β_2^t)
θ_t = θ_{t-1} − α * (m_hat_t / (sqrt(v_hat_t) + ϵ) + λ * θ_{t-1})

## Resources:
- Adam: A Method for Stochastic Optimization (by Diederik P. Kingma and Jimmy Ba):
- [https://arxiv.org/abs/1412.6980]
- Decoupled Weight Decay Regularization (by Ilya Loshchilov and Frank Hutter):
- [https://arxiv.org/abs/1711.05101]
- PyTorch Adam optimizer:
- [https://pytorch.org/docs/stable/generated/torch.optim.Adam.html]
- PyTorch AdamW optimizer:
- [https://pytorch.org/docs/stable/generated/torch.optim.AdamW.html]

### Original Rust Signatures
```rust
pub fn new(learning_rate: Option<f64>,
        betas: Option<(f64, f64);
pub fn step(&mut self, gradients: &[f64], params: &[f64]) -> Vec<f64>;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`adam.rs`](../../../../src/machine_learning/optimization/adam.rs).

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
cargo test --lib machine_learning::optimization::adam
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`adam.rs`](../../../../src/machine_learning/optimization/adam.rs).
