# Count Min Sketch

**Category:** `data_structures` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

The common implementation of a CountMinSketch
Holding a DEPTH x WIDTH matrix of counts

The idea behind the implementation is the following:
Let's start from our problem statement above. We have a frequency map of counts, and want to go reduce its space complexity
The immediate way to do this would be to use a Vector with a fixed size, let this size be `WIDTH`
We will be holding the count of each item `item` in the Vector, at index `i = hash(item) % WIDTH` where `hash` is a hash function: `item -> usize`
We now have constant space.

The problem though is that we'll potentially run into a lot of collisions.
Taking an extreme example, if `WIDTH = 1`, all items will have the same count, which is the sum of counts of every items
We could reduce the amount of collisions by using a bigger `WIDTH` but this wouldn't be way more efficient than the "big" frequency map
How do we improve the solution, but still keeping constant space?

The idea is to use not just one vector, but multiple (`DEPTH`) ones and attach different `hash` functions to each vector
This would lead to the following data structure:
<- WIDTH = 5 ->
D   hash1: [0, 0, 0, 0, 0]
E   hash2: [0, 0, 0, 0, 0]
P   hash3: [0, 0, 0, 0, 0]
T   hash4: [0, 0, 0, 0, 0]
H   hash5: [0, 0, 0, 0, 0]
=   hash6: [0, 0, 0, 0, 0]
7   hash7: [0, 0, 0, 0, 0]
Every hash function must return a different value for the same item.
Let's say we hash "TEST" and:
hash1("TEST") = 42 => idx = 2
hash2("TEST") = 26 => idx = 1
hash3("TEST") = 10 => idx = 0
hash4("TEST") = 33 => idx = 3
hash5("TEST") = 54 => idx = 4
hash6("TEST") = 11 => idx = 1
hash7("TEST") = 50 => idx = 0
This would lead our structure to become:
<- WIDTH = 5 ->
D   hash1: [0, 0, 1, 0, 0]
E   hash2: [0, 1, 0, 0, 0]
P   hash3: [1, 0, 0, 0, 0]
T   hash4: [0, 0, 0, 1, 0]
H   hash5: [0, 0, 0, 0, 1]
=   hash6: [0, 1, 0, 0, 0]
7   hash7: [1, 0, 0, 0, 0]

Now say we hash "OTHER" and:
hash1("OTHER") = 23 => idx = 3
hash2("OTHER") = 11 => idx = 1
hash3("OTHER") = 52 => idx = 2
hash4("OTHER") = 25 => idx = 0
hash5("OTHER") = 31 => idx = 1
hash6("OTHER") = 24 => idx = 4
hash7("OTHER") = 30 => idx = 0
Leading our data structure to become:
<- WIDTH = 5 ->
D   hash1: [0, 0, 1, 1, 0]
E   hash2: [0, 2, 0, 0, 0]
P   hash3: [1, 0, 1, 0, 0]
T   hash4: [1, 0, 0, 1, 0]
H   hash5: [0, 1, 0, 0, 1]
=   hash6: [0, 1, 0, 0, 1]
7   hash7: [2, 0, 0, 0, 0]

We actually can witness some collisions (invalid counts of `2` above in some rows).
This means that if we have to return the count for "TEST", we'd actually fetch counts from every row and return the minimum value

This could potentially be overestimated if we have a huge number of entries and a lot of collisions.
But an interesting property is that the count we return for "TEST" cannot be underestimated

### Original Rust Structs
`HashCountMinSketch`

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
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`count_min_sketch.rs`](../../../../src/data_structures/probabilistic/count_min_sketch.rs).

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
cargo test --lib data_structures::probabilistic::count_min_sketch
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`count_min_sketch.rs`](../../../../src/data_structures/probabilistic/count_min_sketch.rs).
