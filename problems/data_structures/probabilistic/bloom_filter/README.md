# Bloom Filter

**Category:** `data_structures` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

We may have made some progress in term of CPU efficiency, using binary operators.
But we might still run into a lot of collisions with a single 128-bits number.
Can we use greater numbers then? Currently, our implementation is limited to 128 bits.

Should we go back to using an array, then?
We could! But instead of using `Vec<bool>` we could use `Vec<u8>`.
Each `u8` can act as a mask as we've done before, and is actually 1 byte in memory (same as a boolean!)
That'd allow us to go over 128 bits, but would divide by 8 the memory footprint.
That's one thing, and will involve dividing / shifting by 8 in different places.

But still, can we reduce the collisions furthermore?

As we did with count-min-sketch, we could use multiple hash function.
When inserting a value, we compute its hash with every hash function (`hash_i`) and perform the same operation as above (the OR with `fingerprint`)
Then when looking for a value, if **ANY** of the tests (`hash` then `AND`) returns 0 then this means the value is missing from the set, otherwise it would have returned 1
If it returns `1`, it **may** be that the item is present, but could also be a collision
This is what a Bloom Filter is about: returning `false` means the value is necessarily absent, and returning true means it may be present

### Original Rust Signatures
```rust
pub fn with_dimensions(filter_size: usize, hash_count: usize) -> Self;
pub fn from_estimate(estimated_count_of_items: usize,
        max_false_positive_probability: f64,) -> Self;
```

### Complexity
- **Time Complexity:** `O(N)`
- **Space Complexity:** `/// Instead of storing every element and grow the set infinitely, let's use a vector with constant capacity `CAPACITY``

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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`bloom_filter.rs`](../../../../src/data_structures/probabilistic/bloom_filter.rs).

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
cargo test --lib data_structures::probabilistic::bloom_filter
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`bloom_filter.rs`](../../../../src/data_structures/probabilistic/bloom_filter.rs).
