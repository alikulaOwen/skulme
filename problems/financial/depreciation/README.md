# Depreciation

**Category:** `financial` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

# Depreciation

In accounting, depreciation refers to the decreases in the value of a fixed
asset during the asset's useful life. When an organization purchases a fixed
asset, the purchase expenditure is not recognized as an expense immediately.
Instead, the decreases in the asset's value are recognized as expenses over
the years during which the asset is used.

The following methods are implemented here:
- **Straight-line method** — cost spread evenly over the asset's useful life.
- **Diminishing balance method** — a fixed percentage is applied each year to
the asset's remaining book value.
- **Units-of-production method** — depreciation is tied to actual usage
(units produced / hours used) rather than time.
- **Sum-of-years' digits (SYD)** — an accelerated method that applies a
declining fraction to the depreciable cost each year.
- **Double-declining balance (DDB)** — the most common accelerated method;
uses `rate = 2 / useful_years` and automatically switches to straight-line
in the year that method yields a higher charge.

Further information: <https://en.wikipedia.org/wiki/Depreciation>

### Original Rust Signatures
```rust
pub fn straight_line_depreciation(useful_years: u32,
    purchase_value: f64,
    residual_value: f64,) -> Result<Vec<f64>, DepreciationError>;
pub fn diminishing_balance_depreciation(useful_years: u32,
    purchase_value: f64,
    residual_value: f64,
    rate: f64,) -> Result<Vec<f64>, DepreciationError>;
pub fn units_of_production_depreciation(purchase_value: f64,
    residual_value: f64,
    total_units: f64,
    units_per_period: &[f64],) -> Result<Vec<f64>, DepreciationError>;
pub fn sum_of_years_digits_depreciation(useful_years: u32,
    purchase_value: f64,
    residual_value: f64,) -> Result<Vec<f64>, DepreciationError>;
pub fn double_declining_balance_depreciation(useful_years: u32,
    purchase_value: f64,
    residual_value: f64,) -> Result<Vec<f64>, DepreciationError>;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`depreciation.rs`](../../../src/financial/depreciation.rs).

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
cargo test --lib financial::depreciation
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`depreciation.rs`](../../../src/financial/depreciation.rs).
