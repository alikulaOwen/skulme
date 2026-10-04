# Number Theory - Algorithm Practice & Interview Guide

Algorithmic problem implementation and analysis.

## Key Java Interview Takeaways
- Analyze time and space complexity before coding.
- Consider edge cases: empty input, single element, negative numbers, extreme values.
- Write clean, idiomatic Java with proper class naming and methods.

## Comparison: Rust vs Java
- Compare memory management: Rust ownership/borrowing vs Java garbage-collected references.
- Compare error handling: Rust `Result`/`Option` vs Java exceptions/`null`.

---

## Problems & Practice Workspaces (3 Problems)

Each problem has a dedicated workspace with problem statements, runnable Java apps (`java Solution.java`), Python (`python3 solution.py`), TypeScript (`bun solution.ts`), and comparison to the original Rust implementation.

| Problem | Rust Source | Java Executable | Rust Reference Test |
| :--- | :--- | :--- | :--- |
| [Compute Totient](../../problems/number_theory/compute_totient/README.md) | [`compute_totient.rs`](./compute_totient.rs) | `java Solution.java` | `cargo test --lib number_theory::compute_totient` |
| [Euler Totient](../../problems/number_theory/euler_totient/README.md) | [`euler_totient.rs`](./euler_totient.rs) | `java Solution.java` | `cargo test --lib number_theory::euler_totient` |
| [Kth Factor](../../problems/number_theory/kth_factor/README.md) | [`kth_factor.rs`](./kth_factor.rs) | `java Solution.java` | `cargo test --lib number_theory::kth_factor` |

---
*Generated for Java Software Engineering Interview Preparation.*
