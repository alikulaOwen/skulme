# Compression - Algorithm Practice & Interview Guide

Algorithmic problem implementation and analysis.

## Key Java Interview Takeaways
- Analyze time and space complexity before coding.
- Consider edge cases: empty input, single element, negative numbers, extreme values.
- Write clean, idiomatic Java with proper class naming and methods.

## Comparison: Rust vs Java
- Compare memory management: Rust ownership/borrowing vs Java garbage-collected references.
- Compare error handling: Rust `Result`/`Option` vs Java exceptions/`null`.

---

## Problems & Practice Workspaces (6 Problems)

Each problem has a dedicated workspace with problem statements, runnable Java apps (`java Solution.java`), Python (`python3 solution.py`), TypeScript (`bun solution.ts`), and comparison to the original Rust implementation.

| Problem | Rust Source | Java Executable | Rust Reference Test |
| :--- | :--- | :--- | :--- |
| [Burrows Wheeler Transform](../../problems/compression/burrows_wheeler_transform/README.md) | [`burrows_wheeler_transform.rs`](./burrows_wheeler_transform.rs) | `java Solution.java` | `cargo test --lib compression::burrows_wheeler_transform` |
| [Huffman Encoding](../../problems/compression/huffman_encoding/README.md) | [`huffman_encoding.rs`](./huffman_encoding.rs) | `java Solution.java` | `cargo test --lib compression::huffman_encoding` |
| [LZ77](../../problems/compression/lz77/README.md) | [`lz77.rs`](./lz77.rs) | `java Solution.java` | `cargo test --lib compression::lz77` |
| [Move To Front](../../problems/compression/move_to_front/README.md) | [`move_to_front.rs`](./move_to_front.rs) | `java Solution.java` | `cargo test --lib compression::move_to_front` |
| [Peak Signal To Noise Ratio](../../problems/compression/peak_signal_to_noise_ratio/README.md) | [`peak_signal_to_noise_ratio.rs`](./peak_signal_to_noise_ratio.rs) | `java Solution.java` | `cargo test --lib compression::peak_signal_to_noise_ratio` |
| [Run Length Encoding](../../problems/compression/run_length_encoding/README.md) | [`run_length_encoding.rs`](./run_length_encoding.rs) | `java Solution.java` | `cargo test --lib compression::run_length_encoding` |

---
*Generated for Java Software Engineering Interview Preparation.*
