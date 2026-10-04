# Exponential Moving Average

**Category:** `financial` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

Calculate the exponential moving average (EMA) on the series of stock prices.

Wikipedia Reference: <https://en.wikipedia.org/wiki/Exponential_smoothing>
Investopedia Reference: <https://www.investopedia.com/terms/e/ema.asp>

Exponential moving average is used in finance to analyze changes in stock prices.
EMA is used in conjunction with Simple Moving Average (SMA). EMA reacts to
changes in value quicker than SMA, which is one of its key advantages.

# Formula
```text
st = alpha * xt + (1 - alpha) * st_prev
```
Where:
- `st`      : Exponential moving average at timestamp t
- `xt`      : Stock price at timestamp t
- `st_prev` : Exponential moving average at timestamp t-1
- `alpha`   : 2 / (1 + window_size) — the smoothing factor

### Original Rust Signatures
```rust
pub fn exponential_moving_average(stock_prices: impl Iterator<Item = f64>,
    window_size: usize,) -> Result<impl Iterator<Item = f64>, &'static str>;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`exponential_moving_average.rs`](../../../src/financial/exponential_moving_average.rs).

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
cargo test --lib financial::exponential_moving_average
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`exponential_moving_average.rs`](../../../src/financial/exponential_moving_average.rs).
