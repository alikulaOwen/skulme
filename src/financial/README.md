# Financial - Algorithm Practice & Interview Guide

Algorithmic problem implementation and analysis.

## Key Java Interview Takeaways
- Analyze time and space complexity before coding.
- Consider edge cases: empty input, single element, negative numbers, extreme values.
- Write clean, idiomatic Java with proper class naming and methods.

## Comparison: Rust vs Java
- Compare memory management: Rust ownership/borrowing vs Java garbage-collected references.
- Compare error handling: Rust `Result`/`Option` vs Java exceptions/`null`.

---

## Problems & Practice Workspaces (10 Problems)

Each problem has a dedicated workspace with problem statements, runnable Java apps (`java Solution.java`), Python (`python3 solution.py`), TypeScript (`bun solution.ts`), and comparison to the original Rust implementation.

| Problem | Rust Source | Java Executable | Rust Reference Test |
| :--- | :--- | :--- | :--- |
| [Depreciation](../../problems/financial/depreciation/README.md) | [`depreciation.rs`](./depreciation.rs) | `java Solution.java` | `cargo test --lib financial::depreciation` |
| [Equated Monthly Installments](../../problems/financial/equated_monthly_installments/README.md) | [`equated_monthly_installments.rs`](./equated_monthly_installments.rs) | `java Solution.java` | `cargo test --lib financial::equated_monthly_installments` |
| [Exponential Moving Average](../../problems/financial/exponential_moving_average/README.md) | [`exponential_moving_average.rs`](./exponential_moving_average.rs) | `java Solution.java` | `cargo test --lib financial::exponential_moving_average` |
| [Finance Ratios](../../problems/financial/finance_ratios/README.md) | [`finance_ratios.rs`](./finance_ratios.rs) | `java Solution.java` | `cargo test --lib financial::finance_ratios` |
| [Interest](../../problems/financial/interest/README.md) | [`interest.rs`](./interest.rs) | `java Solution.java` | `cargo test --lib financial::interest` |
| [Npv](../../problems/financial/npv/README.md) | [`npv.rs`](./npv.rs) | `java Solution.java` | `cargo test --lib financial::npv` |
| [Npv Sensitivity](../../problems/financial/npv_sensitivity/README.md) | [`npv_sensitivity.rs`](./npv_sensitivity.rs) | `java Solution.java` | `cargo test --lib financial::npv_sensitivity` |
| [Payback](../../problems/financial/payback/README.md) | [`payback.rs`](./payback.rs) | `java Solution.java` | `cargo test --lib financial::payback` |
| [Present Value](../../problems/financial/present_value/README.md) | [`present_value.rs`](./present_value.rs) | `java Solution.java` | `cargo test --lib financial::present_value` |
| [Treynor Ratio](../../problems/financial/treynor_ratio/README.md) | [`treynor_ratio.rs`](./treynor_ratio.rs) | `java Solution.java` | `cargo test --lib financial::treynor_ratio` |

---
*Generated for Java Software Engineering Interview Preparation.*
