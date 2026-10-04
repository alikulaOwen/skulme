# Greedy - Algorithm Practice & Interview Guide

Greedy algorithms make the locally optimal choice at each step with the goal of finding a global optimum.

## Key Java Interview Takeaways
- Greedy algorithms usually require sorting input first (e.g. by end-time in interval scheduling) or using a `PriorityQueue`.
- In interviews, you must be able to justify why the greedy choice property holds and does not get trapped in local optima.

## Comparison: Rust vs Java
- Greedy logic translates directly between languages; differences lie only in sorting collections and priority queue APIs.

---

## Problems & Practice Workspaces (4 Problems)

Each problem has a dedicated workspace with problem statements, runnable Java apps (`java Solution.java`), Python (`python3 solution.py`), TypeScript (`bun solution.ts`), and comparison to the original Rust implementation.

| Problem | Rust Source | Java Executable | Rust Reference Test |
| :--- | :--- | :--- | :--- |
| [Job Sequencing](../../problems/greedy/job_sequencing/README.md) | [`job_sequencing.rs`](./job_sequencing.rs) | `java Solution.java` | `cargo test --lib greedy::job_sequencing` |
| [Minimum Coin Change](../../problems/greedy/minimum_coin_change/README.md) | [`minimum_coin_change.rs`](./minimum_coin_change.rs) | `java Solution.java` | `cargo test --lib greedy::minimum_coin_change` |
| [Smallest Range](../../problems/greedy/smallest_range/README.md) | [`smallest_range.rs`](./smallest_range.rs) | `java Solution.java` | `cargo test --lib greedy::smallest_range` |
| [Stable Matching](../../problems/greedy/stable_matching/README.md) | [`stable_matching.rs`](./stable_matching.rs) | `java Solution.java` | `cargo test --lib greedy::stable_matching` |

---
*Generated for Java Software Engineering Interview Preparation.*
