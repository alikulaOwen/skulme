# Geometry - Algorithm Practice & Interview Guide

Algorithmic problem implementation and analysis.

## Key Java Interview Takeaways
- Analyze time and space complexity before coding.
- Consider edge cases: empty input, single element, negative numbers, extreme values.
- Write clean, idiomatic Java with proper class naming and methods.

## Comparison: Rust vs Java
- Compare memory management: Rust ownership/borrowing vs Java garbage-collected references.
- Compare error handling: Rust `Result`/`Option` vs Java exceptions/`null`.

---

## Problems & Practice Workspaces (7 Problems)

Each problem has a dedicated workspace with problem statements, runnable Java apps (`java Solution.java`), Python (`python3 solution.py`), TypeScript (`bun solution.ts`), and comparison to the original Rust implementation.

| Problem | Rust Source | Java Executable | Rust Reference Test |
| :--- | :--- | :--- | :--- |
| [Closest Points](../../problems/geometry/closest_points/README.md) | [`closest_points.rs`](./closest_points.rs) | `java Solution.java` | `cargo test --lib geometry::closest_points` |
| [Graham Scan](../../problems/geometry/graham_scan/README.md) | [`graham_scan.rs`](./graham_scan.rs) | `java Solution.java` | `cargo test --lib geometry::graham_scan` |
| [Jarvis Scan](../../problems/geometry/jarvis_scan/README.md) | [`jarvis_scan.rs`](./jarvis_scan.rs) | `java Solution.java` | `cargo test --lib geometry::jarvis_scan` |
| [Point](../../problems/geometry/point/README.md) | [`point.rs`](./point.rs) | `java Solution.java` | `cargo test --lib geometry::point` |
| [Polygon Points](../../problems/geometry/polygon_points/README.md) | [`polygon_points.rs`](./polygon_points.rs) | `java Solution.java` | `cargo test --lib geometry::polygon_points` |
| [Ramer Douglas Peucker](../../problems/geometry/ramer_douglas_peucker/README.md) | [`ramer_douglas_peucker.rs`](./ramer_douglas_peucker.rs) | `java Solution.java` | `cargo test --lib geometry::ramer_douglas_peucker` |
| [Segment](../../problems/geometry/segment/README.md) | [`segment.rs`](./segment.rs) | `java Solution.java` | `cargo test --lib geometry::segment` |

---
*Generated for Java Software Engineering Interview Preparation.*
