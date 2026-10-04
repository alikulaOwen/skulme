# Hashing - Algorithm Practice & Interview Guide

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
| [Blake2b](../../problems/hashing/blake2b/README.md) | [`blake2b.rs`](./blake2b.rs) | `java Solution.java` | `cargo test --lib hashing::blake2b` |
| [Fletcher](../../problems/hashing/fletcher/README.md) | [`fletcher.rs`](./fletcher.rs) | `java Solution.java` | `cargo test --lib hashing::fletcher` |
| [Hashing Traits](../../problems/hashing/hashing_traits/README.md) | [`hashing_traits.rs`](./hashing_traits.rs) | `java Solution.java` | `cargo test --lib hashing::hashing_traits` |
| [MD5](../../problems/hashing/md5/README.md) | [`md5.rs`](./md5.rs) | `java Solution.java` | `cargo test --lib hashing::md5` |
| [Sha1](../../problems/hashing/sha1/README.md) | [`sha1.rs`](./sha1.rs) | `java Solution.java` | `cargo test --lib hashing::sha1` |
| [Sha2](../../problems/hashing/sha2/README.md) | [`sha2.rs`](./sha2.rs) | `java Solution.java` | `cargo test --lib hashing::sha2` |
| [Sha3](../../problems/hashing/sha3/README.md) | [`sha3.rs`](./sha3.rs) | `java Solution.java` | `cargo test --lib hashing::sha3` |

---
*Generated for Java Software Engineering Interview Preparation.*
