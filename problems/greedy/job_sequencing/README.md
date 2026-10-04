# Job Sequencing

**Category:** `greedy` | **Language Track:** Java (Interview Focus), Rust, Python, TypeScript

---

## Problem Statement

Job Sequencing

Given a set of jobs, each with a deadline and profit, schedule jobs to
maximise total profit. Each job takes exactly one unit of time and must
be completed on or before its deadline. Only one job can run at a time.

# Algorithm (greedy)
1. Sort jobs by profit in descending order.
2. For each job (highest profit first), find the latest free time-slot
that is ≤ the job's deadline and assign the job there.
3. Return the sequence of scheduled jobs and the total profit earned.

# Complexity
- Time:  O(n·D) — for each of the n jobs we may scan backwards through up to D slots,
where D is the maximum deadline.
- Space: O(min(n, D)) — slot array is capped at the number of jobs, since at most
n jobs can ever be scheduled regardless of how large D is.

# References
- Cormen et al., *Introduction to Algorithms*, 4th ed., §16.5
- <https://en.wikipedia.org/wiki/Optimal_job_scheduling>

### Original Rust Signatures
```rust
pub fn new(name: impl Into<String>, deadline: usize, profit: u64) -> Self;
pub fn schedule_jobs(mut jobs: Vec<Job>) -> ScheduleResult;
```

### Complexity
- **Time Complexity:** `slot`
- **Space Complexity:** `O(min(n, D)) — slot array is capped at the number of jobs, since at most`

---

## Java Interview Strategy & Tips

- Greedy algorithms usually require sorting input first (e.g. by end-time in interval scheduling) or using a `PriorityQueue`.
- In interviews, you must be able to justify why the greedy choice property holds and does not get trapped in local optima.

### Rust vs. Java Perspective
- Greedy logic translates directly between languages; differences lie only in sorting collections and priority queue APIs.
- **Reference Implementation:** Compare your Java solution with the original Rust code in [`job_sequencing.rs`](../../../src/greedy/job_sequencing.rs).

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
cargo test --lib greedy::job_sequencing
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
- [ ] Contrast time/space complexity and idiomatic differences with the Rust source in [`job_sequencing.rs`](../../../src/greedy/job_sequencing.rs).
