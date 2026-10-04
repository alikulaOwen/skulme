# Navigation - Algorithm Practice & Interview Guide

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
| [Bearing](../../problems/navigation/bearing/README.md) | [`bearing.rs`](./bearing.rs) | `java Solution.java` | `cargo test --lib navigation::bearing` |
| [Haversine](../../problems/navigation/haversine/README.md) | [`haversine.rs`](./haversine.rs) | `java Solution.java` | `cargo test --lib navigation::haversine` |
| [Rhumbline](../../problems/navigation/rhumbline/README.md) | [`rhumbline.rs`](./rhumbline.rs) | `java Solution.java` | `cargo test --lib navigation::rhumbline` |

---
*Generated for Java Software Engineering Interview Preparation.*
