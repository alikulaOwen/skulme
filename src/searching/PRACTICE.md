# Searching - Algorithm Practice & Interview Guide

Searching algorithms locate a target element or determine its absence within a collection.

## Key Java Interview Takeaways
- In Binary Search, ALWAYS calculate midpoint using `int mid = left + (right - left) / 2;` to avoid 32-bit integer overflow.
- Check loop invariants: `while (left <= right)` when `right = n - 1` vs `while (left < right)` when `right = n`.
- Java provides `Arrays.binarySearch()`, which returns `-(insertion_point + 1)` if the key is not present.

## Comparison: Rust vs Java
- Rust's `slice::binary_search()` returns `Result<usize, usize>` (`Ok(index)` or `Err(insert_index)`).
- Java returns a primitive `int`, encoding not-found as a negative integer.

---

## Problems & Practice Workspaces (16 Problems)

Each problem has a dedicated workspace with problem statements, runnable Java apps (`java Solution.java`), Python (`python3 solution.py`), TypeScript (`bun solution.ts`), and comparison to the original Rust implementation.

| Problem | Rust Source | Java Executable | Rust Reference Test |
| :--- | :--- | :--- | :--- |
| [Binary Search](../../problems/searching/binary_search/README.md) | [`binary_search.rs`](./binary_search.rs) | `java Solution.java` | `cargo test --lib searching::binary_search` |
| [Binary Search Recursive](../../problems/searching/binary_search_recursive/README.md) | [`binary_search_recursive.rs`](./binary_search_recursive.rs) | `java Solution.java` | `cargo test --lib searching::binary_search_recursive` |
| [Exponential Search](../../problems/searching/exponential_search/README.md) | [`exponential_search.rs`](./exponential_search.rs) | `java Solution.java` | `cargo test --lib searching::exponential_search` |
| [Fibonacci Search](../../problems/searching/fibonacci_search/README.md) | [`fibonacci_search.rs`](./fibonacci_search.rs) | `java Solution.java` | `cargo test --lib searching::fibonacci_search` |
| [Interpolation Search](../../problems/searching/interpolation_search/README.md) | [`interpolation_search.rs`](./interpolation_search.rs) | `java Solution.java` | `cargo test --lib searching::interpolation_search` |
| [Jump Search](../../problems/searching/jump_search/README.md) | [`jump_search.rs`](./jump_search.rs) | `java Solution.java` | `cargo test --lib searching::jump_search` |
| [Kth Smallest](../../problems/searching/kth_smallest/README.md) | [`kth_smallest.rs`](./kth_smallest.rs) | `java Solution.java` | `cargo test --lib searching::kth_smallest` |
| [Kth Smallest Heap](../../problems/searching/kth_smallest_heap/README.md) | [`kth_smallest_heap.rs`](./kth_smallest_heap.rs) | `java Solution.java` | `cargo test --lib searching::kth_smallest_heap` |
| [Linear Search](../../problems/searching/linear_search/README.md) | [`linear_search.rs`](./linear_search.rs) | `java Solution.java` | `cargo test --lib searching::linear_search` |
| [Moore Voting](../../problems/searching/moore_voting/README.md) | [`moore_voting.rs`](./moore_voting.rs) | `java Solution.java` | `cargo test --lib searching::moore_voting` |
| [Quick Select](../../problems/searching/quick_select/README.md) | [`quick_select.rs`](./quick_select.rs) | `java Solution.java` | `cargo test --lib searching::quick_select` |
| [Saddleback Search](../../problems/searching/saddleback_search/README.md) | [`saddleback_search.rs`](./saddleback_search.rs) | `java Solution.java` | `cargo test --lib searching::saddleback_search` |
| [Ternary Search](../../problems/searching/ternary_search/README.md) | [`ternary_search.rs`](./ternary_search.rs) | `java Solution.java` | `cargo test --lib searching::ternary_search` |
| [Ternary Search Min Max](../../problems/searching/ternary_search_min_max/README.md) | [`ternary_search_min_max.rs`](./ternary_search_min_max.rs) | `java Solution.java` | `cargo test --lib searching::ternary_search_min_max` |
| [Ternary Search Min Max Recursive](../../problems/searching/ternary_search_min_max_recursive/README.md) | [`ternary_search_min_max_recursive.rs`](./ternary_search_min_max_recursive.rs) | `java Solution.java` | `cargo test --lib searching::ternary_search_min_max_recursive` |
| [Ternary Search Recursive](../../problems/searching/ternary_search_recursive/README.md) | [`ternary_search_recursive.rs`](./ternary_search_recursive.rs) | `java Solution.java` | `cargo test --lib searching::ternary_search_recursive` |

---
*Generated for Java Software Engineering Interview Preparation.*
