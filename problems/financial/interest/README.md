# Interest

**Category:** `financial` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

Calculates simple, compound, and APR interest on a principal amount.

Formulas:
Simple Interest:   I = p * r * t
Compound Interest: I = p * ((1 + r)^n - 1)
APR Interest:      Compound interest with r = annual_rate / 365
and n = years * 365
where:
- `p` is the principal
- `r` is the interest rate per period
- `t` is the number of periods (days)
- `n` is the total number of compounding periods

Reference: https://www.investopedia.com/terms/i/interest.asp

### Original Rust Signatures
```rust
pub fn simple_interest(principal: f64,
    daily_interest_rate: f64,
    days_between_payments: f64,) -> Result<f64, &'static str>;
pub fn compound_interest(principal: f64,
    nominal_annual_interest_rate: f64,
    number_of_compounding_periods: f64,) -> Result<f64, &'static str>;
pub fn apr_interest(principal: f64,
    nominal_annual_percentage_rate: f64,
    number_of_years: f64,) -> Result<f64, &'static str>;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`interest.rs`](../../../src/financial/interest.rs).

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
cargo test --lib financial::interest
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`interest.rs`](../../../src/financial/interest.rs).
