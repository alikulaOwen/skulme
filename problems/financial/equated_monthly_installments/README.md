# Equated Monthly Installments

**Category:** `financial` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

Calculates the Equated Monthly Installment (EMI) for a loan.

Formula: A = p * r * (1 + r)^n / ((1 + r)^n - 1)
where:
- `p` is the principal
- `r` is the monthly interest rate (annual rate / 12)
- `n` is the total number of monthly payments (years * 12)

Wikipedia Reference: https://en.wikipedia.org/wiki/Equated_monthly_installment

### Original Rust Signatures
```rust
pub fn equated_monthly_installments(principal: f64,
    rate_per_annum: f64,
    years_to_repay: u32,) -> Result<f64, &'static str>;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`equated_monthly_installments.rs`](../../../src/financial/equated_monthly_installments.rs).

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
cargo test --lib financial::equated_monthly_installments
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`equated_monthly_installments.rs`](../../../src/financial/equated_monthly_installments.rs).
