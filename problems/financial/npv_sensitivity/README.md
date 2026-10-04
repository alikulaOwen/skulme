# Npv Sensitivity

**Category:** `financial` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

/ Computes the Net Present Value (NPV) of a cash flow series
/ at multiple discount rates to show sensitivity.
/
/ # Inputs:
/ - `cash_flows`: A slice of cash flows, where each entry is a period value
/   e.g., year 0 is initial investment, year 1+ are returns or costs
/ - `discount_rates`: A slice of discount rates, e.g. `[0.05, 0.10, 0.20]`,
/   where each rate is evaluated independently.
/
/ # Output:
/ - Returns a vector of NPV values, each corresponding to a rate in `discount_rates`.
/   For example, output is `[npv_rate1, npv_rate2, ...]`.

### Original Rust Signatures
```rust
pub fn npv_sensitivity(cash_flows: &[f64], discount_rates: &[f64]) -> Vec<f64>;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`npv_sensitivity.rs`](../../../src/financial/npv_sensitivity.rs).

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
cargo test --lib financial::npv_sensitivity
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`npv_sensitivity.rs`](../../../src/financial/npv_sensitivity.rs).
