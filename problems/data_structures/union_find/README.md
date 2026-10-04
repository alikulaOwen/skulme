# Union Find

**Category:** `data_structures` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

A Union-Find (Disjoint Set) data structure implementation in Rust.

The Union-Find data structure keeps track of elements partitioned into
disjoint (non-overlapping) sets.
It provides near-constant-time operations to add new sets, to find the
representative of a set, and to merge sets.

### Original Rust Signatures
```rust
pub fn with_capacity(capacity: usize) -> Self;
pub fn insert(&mut self, item: T);
pub fn find(&mut self, value: &T) -> Option<usize>;
pub fn union(&mut self, first_item: &T, sec_item: &T) -> Option<bool>;
pub fn is_same_set(&mut self, first_item: &T, sec_item: &T) -> bool;
pub fn count(&self) -> usize;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`union_find.rs`](../../../src/data_structures/union_find.rs).

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
cargo test --lib data_structures::union_find
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`union_find.rs`](../../../src/data_structures/union_find.rs).
