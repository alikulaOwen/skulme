# Big Integer - Algorithm Practice & Interview Guide

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
| [Fast Factorial](../../problems/big_integer/fast_factorial/README.md) | [`fast_factorial.rs`](./fast_factorial.rs) | `java Solution.java` | `cargo test --lib big_integer::fast_factorial` |
| [Multiply](../../problems/big_integer/multiply/README.md) | [`multiply.rs`](./multiply.rs) | `java Solution.java` | `cargo test --lib big_integer::multiply` |
| [Poly1305](../../problems/big_integer/poly1305/README.md) | [`poly1305.rs`](./poly1305.rs) | `java Solution.java` | `cargo test --lib big_integer::poly1305` |

---
*Generated for Java Software Engineering Interview Preparation.*
