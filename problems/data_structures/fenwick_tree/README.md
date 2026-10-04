# Fenwick Tree

**Category:** `data_structures` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

A Fenwick Tree (also known as a Binary Indexed Tree) that supports efficient
prefix sum, range sum and point queries, as well as point updates.

The Fenwick Tree uses **1-based** indexing internally but presents a **0-based** interface to the user.
This design improves efficiency and simplifies both internal operations and external usage.

### Original Rust Signatures
```rust
pub fn with_capacity(capacity: usize) -> Self;
pub fn update(&mut self, index: usize, value: T) -> Result<(), FenwickTreeError>;
pub fn prefix_query(&self, index: usize) -> Result<T, FenwickTreeError>;
pub fn range_query(&self, left: usize, right: usize) -> Result<T, FenwickTreeError>;
pub fn point_query(&self, index: usize) -> Result<T, FenwickTreeError>;
pub fn set(&mut self, index: usize, value: T) -> Result<(), FenwickTreeError>;
```

### Complexity
- **Time Complexity:** `O(N)`
- **Space Complexity:** `O(1)`

---

## Java Interview Strategy & Tips

- Memorize Java's Collections Hierarchy:
  - `List`: `ArrayList` (dynamic array), `LinkedList` (doubly linked).
  - `Queue` / `Deque`: `ArrayDeque` (fast array-backed ring buffer), `PriorityQueue` (min-heap).
  - `Set`: `HashSet` (O(1)), `TreeSet` (Red-Black Tree, O(log N)), `LinkedHashSet` (insertion-ordered).
  - `Map`: `HashMap` (O(1)), `TreeMap` (Red-Black Tree, O(log N)), `LinkedHashMap` (LRU cache base).
- Interview classic: implementing LRU Cache with `LinkedHashMap(capacity, 0.75f, true)` or custom Doubly Linked List + HashMap.

### Rust vs. Java Perspective
- Rust's standard library provides `VecDeque`, `BinaryHeap` (Max-heap by default, unlike Java's Min-heap!), `BTreeMap`, and `HashMap`.
- Building self-referential structures like linked lists or trees from scratch is tricky in Rust due to ownership (`Box`, `Rc<RefCell<T>>`), whereas Java handles node references naturally via garbage collection.
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`fenwick_tree.rs`](../../../src/data_structures/fenwick_tree.rs).

---

## How to Spin & Run

### 1. Java (Target Interview Language)
Run directly as a single-file application with built-in tests:
```bash
java Solution.java
```

### 2. Rust (Original Ground Truth Answer)
Run the crate unit tests for this module from the repository root:
```bash
cargo test --lib data_structures::fenwick_tree
```

### 3. Python (Rapid Prototyping)
```bash
python3 solution.py
```

### 4. TypeScript (Industry Standard)
```bash
bun solution.ts
# or using Deno:
deno run solution.ts
```

---

## Self-Evaluation Checklist
- [ ] Understand the problem constraints and edge cases (e.g. empty input, bounds, duplicates).
- [ ] Implement the optimal solution in `Solution.java`.
- [ ] Verify correctness using `java Solution.java`.
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`fenwick_tree.rs`](../../../src/data_structures/fenwick_tree.rs).
