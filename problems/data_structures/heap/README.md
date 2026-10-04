# Heap

**Category:** `data_structures` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

A generic heap data structure.

This module provides a `Heap` implementation that can function as either a
min-heap or a max-heap. It supports common heap operations such as adding,
removing, and iterating over elements. The heap can also be created from
an unsorted vector and supports custom comparators for flexible sorting
behavior.

### Original Rust Signatures
```rust
pub fn new(comparator: fn(&T, &T) -> bool) -> Self;
pub fn from_vec(items: Vec<T>, comparator: fn(&T, &T) -> bool) -> Self;
pub fn len(&self) -> usize;
pub fn is_empty(&self) -> bool;
pub fn add(&mut self, value: T);
pub fn pop(&mut self) -> Option<T>;
pub fn iter(&self) -> Iter<'_, T>;
pub fn new_min() -> Heap<T>;
pub fn new_max() -> Heap<T>;
pub fn from_vec_min(items: Vec<T>) -> Heap<T>;
pub fn from_vec_max(items: Vec<T>) -> Heap<T>;
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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`heap.rs`](../../../src/data_structures/heap.rs).

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
cargo test --lib data_structures::heap
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`heap.rs`](../../../src/data_structures/heap.rs).
