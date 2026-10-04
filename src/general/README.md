# General - Algorithm Practice & Interview Guide

Algorithmic problem implementation and analysis.

## Key Java Interview Takeaways
- Analyze time and space complexity before coding.
- Consider edge cases: empty input, single element, negative numbers, extreme values.
- Write clean, idiomatic Java with proper class naming and methods.

## Comparison: Rust vs Java
- Compare memory management: Rust ownership/borrowing vs Java garbage-collected references.
- Compare error handling: Rust `Result`/`Option` vs Java exceptions/`null`.

---

## Problems & Practice Workspaces (13 Problems)

Each problem has a dedicated workspace with problem statements, runnable Java apps (`java Solution.java`), Python (`python3 solution.py`), TypeScript (`bun solution.ts`), and comparison to the original Rust implementation.

| Problem | Rust Source | Java Executable | Rust Reference Test |
| :--- | :--- | :--- | :--- |
| [Convex Hull](../../problems/general/convex_hull/README.md) | [`convex_hull.rs`](./convex_hull.rs) | `java Solution.java` | `cargo test --lib general::convex_hull` |
| [Fisher Yates Shuffle](../../problems/general/fisher_yates_shuffle/README.md) | [`fisher_yates_shuffle.rs`](./fisher_yates_shuffle.rs) | `java Solution.java` | `cargo test --lib general::fisher_yates_shuffle` |
| [Genetic](../../problems/general/genetic/README.md) | [`genetic.rs`](./genetic.rs) | `java Solution.java` | `cargo test --lib general::genetic` |
| [Hanoi](../../problems/general/hanoi/README.md) | [`hanoi.rs`](./hanoi.rs) | `java Solution.java` | `cargo test --lib general::hanoi` |
| [Huffman Encoding](../../problems/general/huffman_encoding/README.md) | [`huffman_encoding.rs`](./huffman_encoding.rs) | `java Solution.java` | `cargo test --lib general::huffman_encoding` |
| [Kadane Algorithm](../../problems/general/kadane_algorithm/README.md) | [`kadane_algorithm.rs`](./kadane_algorithm.rs) | `java Solution.java` | `cargo test --lib general::kadane_algorithm` |
| [Kmeans](../../problems/general/kmeans/README.md) | [`kmeans.rs`](./kmeans.rs) | `java Solution.java` | `cargo test --lib general::kmeans` |
| [Mex](../../problems/general/mex/README.md) | [`mex.rs`](./mex.rs) | `java Solution.java` | `cargo test --lib general::mex` |
| [Heap](../../problems/general/permutations/heap/README.md) | [`heap.rs`](./heap.rs) | `java Solution.java` | `cargo test --lib general::permutations::heap` |
| [Naive](../../problems/general/permutations/naive/README.md) | [`naive.rs`](./naive.rs) | `java Solution.java` | `cargo test --lib general::permutations::naive` |
| [Steinhaus Johnson Trotter](../../problems/general/permutations/steinhaus_johnson_trotter/README.md) | [`steinhaus_johnson_trotter.rs`](./steinhaus_johnson_trotter.rs) | `java Solution.java` | `cargo test --lib general::permutations::steinhaus_johnson_trotter` |
| [Subarray Sum Equals K](../../problems/general/subarray_sum_equals_k/README.md) | [`subarray_sum_equals_k.rs`](./subarray_sum_equals_k.rs) | `java Solution.java` | `cargo test --lib general::subarray_sum_equals_k` |
| [Two Sum](../../problems/general/two_sum/README.md) | [`two_sum.rs`](./two_sum.rs) | `java Solution.java` | `cargo test --lib general::two_sum` |

---
*Generated for Java Software Engineering Interview Preparation.*
