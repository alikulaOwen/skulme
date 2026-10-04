# Data Structures - Algorithm Practice & Interview Guide

Fundamental and advanced data structures for organizing, storing, and accessing data efficiently.

## Key Java Interview Takeaways
- Memorize Java's Collections Hierarchy:
  - `List`: `ArrayList` (dynamic array), `LinkedList` (doubly linked).
  - `Queue` / `Deque`: `ArrayDeque` (fast array-backed ring buffer), `PriorityQueue` (min-heap).
  - `Set`: `HashSet` (O(1)), `TreeSet` (Red-Black Tree, O(log N)), `LinkedHashSet` (insertion-ordered).
  - `Map`: `HashMap` (O(1)), `TreeMap` (Red-Black Tree, O(log N)), `LinkedHashMap` (LRU cache base).
- Interview classic: implementing LRU Cache with `LinkedHashMap(capacity, 0.75f, true)` or custom Doubly Linked List + HashMap.

## Comparison: Rust vs Java
- Rust's standard library provides `VecDeque`, `BinaryHeap` (Max-heap by default, unlike Java's Min-heap!), `BTreeMap`, and `HashMap`.
- Building self-referential structures like linked lists or trees from scratch is tricky in Rust due to ownership (`Box`, `Rc<RefCell<T>>`), whereas Java handles node references naturally via garbage collection.

---

## Problems & Practice Workspaces (23 Problems)

Each problem has a dedicated workspace with problem statements, runnable Java apps (`java Solution.java`), Python (`python3 solution.py`), TypeScript (`bun solution.ts`), and comparison to the original Rust implementation.

| Problem | Rust Source | Java Executable | Rust Reference Test |
| :--- | :--- | :--- | :--- |
| [AVL Tree](../../problems/data_structures/avl_tree/README.md) | [`avl_tree.rs`](./avl_tree.rs) | `java Solution.java` | `cargo test --lib data_structures::avl_tree` |
| [B Tree](../../problems/data_structures/b_tree/README.md) | [`b_tree.rs`](./b_tree.rs) | `java Solution.java` | `cargo test --lib data_structures::b_tree` |
| [Binary Search Tree](../../problems/data_structures/binary_search_tree/README.md) | [`binary_search_tree.rs`](./binary_search_tree.rs) | `java Solution.java` | `cargo test --lib data_structures::binary_search_tree` |
| [Fenwick Tree](../../problems/data_structures/fenwick_tree/README.md) | [`fenwick_tree.rs`](./fenwick_tree.rs) | `java Solution.java` | `cargo test --lib data_structures::fenwick_tree` |
| [Floyds Algorithm](../../problems/data_structures/floyds_algorithm/README.md) | [`floyds_algorithm.rs`](./floyds_algorithm.rs) | `java Solution.java` | `cargo test --lib data_structures::floyds_algorithm` |
| [Graph](../../problems/data_structures/graph/README.md) | [`graph.rs`](./graph.rs) | `java Solution.java` | `cargo test --lib data_structures::graph` |
| [Hash Table](../../problems/data_structures/hash_table/README.md) | [`hash_table.rs`](./hash_table.rs) | `java Solution.java` | `cargo test --lib data_structures::hash_table` |
| [Heap](../../problems/data_structures/heap/README.md) | [`heap.rs`](./heap.rs) | `java Solution.java` | `cargo test --lib data_structures::heap` |
| [Lazy Segment Tree](../../problems/data_structures/lazy_segment_tree/README.md) | [`lazy_segment_tree.rs`](./lazy_segment_tree.rs) | `java Solution.java` | `cargo test --lib data_structures::lazy_segment_tree` |
| [Linked List](../../problems/data_structures/linked_list/README.md) | [`linked_list.rs`](./linked_list.rs) | `java Solution.java` | `cargo test --lib data_structures::linked_list` |
| [Bloom Filter](../../problems/data_structures/probabilistic/bloom_filter/README.md) | [`bloom_filter.rs`](./bloom_filter.rs) | `java Solution.java` | `cargo test --lib data_structures::probabilistic::bloom_filter` |
| [Count Min Sketch](../../problems/data_structures/probabilistic/count_min_sketch/README.md) | [`count_min_sketch.rs`](./count_min_sketch.rs) | `java Solution.java` | `cargo test --lib data_structures::probabilistic::count_min_sketch` |
| [Queue](../../problems/data_structures/queue/README.md) | [`queue.rs`](./queue.rs) | `java Solution.java` | `cargo test --lib data_structures::queue` |
| [Range Minimum Query](../../problems/data_structures/range_minimum_query/README.md) | [`range_minimum_query.rs`](./range_minimum_query.rs) | `java Solution.java` | `cargo test --lib data_structures::range_minimum_query` |
| [Rb Tree](../../problems/data_structures/rb_tree/README.md) | [`rb_tree.rs`](./rb_tree.rs) | `java Solution.java` | `cargo test --lib data_structures::rb_tree` |
| [Segment Tree](../../problems/data_structures/segment_tree/README.md) | [`segment_tree.rs`](./segment_tree.rs) | `java Solution.java` | `cargo test --lib data_structures::segment_tree` |
| [Segment Tree Recursive](../../problems/data_structures/segment_tree_recursive/README.md) | [`segment_tree_recursive.rs`](./segment_tree_recursive.rs) | `java Solution.java` | `cargo test --lib data_structures::segment_tree_recursive` |
| [Skip List](../../problems/data_structures/skip_list/README.md) | [`skip_list.rs`](./skip_list.rs) | `java Solution.java` | `cargo test --lib data_structures::skip_list` |
| [Stack Using Singly Linked List](../../problems/data_structures/stack_using_singly_linked_list/README.md) | [`stack_using_singly_linked_list.rs`](./stack_using_singly_linked_list.rs) | `java Solution.java` | `cargo test --lib data_structures::stack_using_singly_linked_list` |
| [Treap](../../problems/data_structures/treap/README.md) | [`treap.rs`](./treap.rs) | `java Solution.java` | `cargo test --lib data_structures::treap` |
| [Trie](../../problems/data_structures/trie/README.md) | [`trie.rs`](./trie.rs) | `java Solution.java` | `cargo test --lib data_structures::trie` |
| [Union Find](../../problems/data_structures/union_find/README.md) | [`union_find.rs`](./union_find.rs) | `java Solution.java` | `cargo test --lib data_structures::union_find` |
| [Veb Tree](../../problems/data_structures/veb_tree/README.md) | [`veb_tree.rs`](./veb_tree.rs) | `java Solution.java` | `cargo test --lib data_structures::veb_tree` |

---
*Generated for Java Software Engineering Interview Preparation.*
