# Finance Ratios

**Category:** `financial` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

Calculating simple ratios like Return on Investment (ROI), Debt to Equity, Gross Profit Margin
and Earnings per Sale (EPS)

### Original Rust Signatures
```rust
pub fn return_on_investment(gain: f64, cost: f64) -> f64;
pub fn debt_to_equity(debt: f64, equity: f64) -> f64;
pub fn gross_profit_margin(revenue: f64, cost: f64) -> f64;
pub fn earnings_per_sale(net_income: f64, pref_dividend: f64, share_avg: f64) -> f64;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`finance_ratios.rs`](../../../src/financial/finance_ratios.rs).

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
cargo test --lib financial::finance_ratios
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`finance_ratios.rs`](../../../src/financial/finance_ratios.rs).
