# Dynamic Programming - Algorithm Practice & Interview Guide

Dynamic Programming breaks down complex problems into overlapping subproblems with optimal substructure.

## Key Java Interview Takeaways
- Multi-dimensional arrays `int[][] dp = new int[m][n]` in Java are arrays of heap references; consider flat arrays `int[m * n]` or rolling 1D arrays for cache locality.
- Watch for integer overflow when initializing memoization tables with `Integer.MAX_VALUE` (adding 1 wraps around to negative). Use `1_000_000_000` or check for sentinel before adding.
- Identify: State definition, Base cases, Transition relation, and Evaluation order.

## Comparison: Rust vs Java
- Rust guarantees memory safety and bounds checks, but idiomatically uses flat vectors `Vec<T>` with 1D indexing.
- Java relies on JVM GC for allocated DP tables, so minimize object allocations inside DP loops.

---

## Problems & Practice Workspaces (26 Problems)

Each problem has a dedicated workspace with problem statements, runnable Java apps (`java Solution.java`), Python (`python3 solution.py`), TypeScript (`bun solution.ts`), and comparison to the original Rust implementation.

| Problem | Rust Source | Java Executable | Rust Reference Test |
| :--- | :--- | :--- | :--- |
| [Catalan Numbers](../../problems/dynamic_programming/catalan_numbers/README.md) | [`catalan_numbers.rs`](./catalan_numbers.rs) | `java Solution.java` | `cargo test --lib dynamic_programming::catalan_numbers` |
| [Coin Change](../../problems/dynamic_programming/coin_change/README.md) | [`coin_change.rs`](./coin_change.rs) | `java Solution.java` | `cargo test --lib dynamic_programming::coin_change` |
| [Egg Dropping](../../problems/dynamic_programming/egg_dropping/README.md) | [`egg_dropping.rs`](./egg_dropping.rs) | `java Solution.java` | `cargo test --lib dynamic_programming::egg_dropping` |
| [Fibonacci](../../problems/dynamic_programming/fibonacci/README.md) | [`fibonacci.rs`](./fibonacci.rs) | `java Solution.java` | `cargo test --lib dynamic_programming::fibonacci` |
| [Fractional Knapsack](../../problems/dynamic_programming/fractional_knapsack/README.md) | [`fractional_knapsack.rs`](./fractional_knapsack.rs) | `java Solution.java` | `cargo test --lib dynamic_programming::fractional_knapsack` |
| [Integer Partition](../../problems/dynamic_programming/integer_partition/README.md) | [`integer_partition.rs`](./integer_partition.rs) | `java Solution.java` | `cargo test --lib dynamic_programming::integer_partition` |
| [Is Subsequence](../../problems/dynamic_programming/is_subsequence/README.md) | [`is_subsequence.rs`](./is_subsequence.rs) | `java Solution.java` | `cargo test --lib dynamic_programming::is_subsequence` |
| [Knapsack](../../problems/dynamic_programming/knapsack/README.md) | [`knapsack.rs`](./knapsack.rs) | `java Solution.java` | `cargo test --lib dynamic_programming::knapsack` |
| [Longest Common Subsequence](../../problems/dynamic_programming/longest_common_subsequence/README.md) | [`longest_common_subsequence.rs`](./longest_common_subsequence.rs) | `java Solution.java` | `cargo test --lib dynamic_programming::longest_common_subsequence` |
| [Longest Common Substring](../../problems/dynamic_programming/longest_common_substring/README.md) | [`longest_common_substring.rs`](./longest_common_substring.rs) | `java Solution.java` | `cargo test --lib dynamic_programming::longest_common_substring` |
| [Longest Continuous Increasing Subsequence](../../problems/dynamic_programming/longest_continuous_increasing_subsequence/README.md) | [`longest_continuous_increasing_subsequence.rs`](./longest_continuous_increasing_subsequence.rs) | `java Solution.java` | `cargo test --lib dynamic_programming::longest_continuous_increasing_subsequence` |
| [Longest Increasing Subsequence](../../problems/dynamic_programming/longest_increasing_subsequence/README.md) | [`longest_increasing_subsequence.rs`](./longest_increasing_subsequence.rs) | `java Solution.java` | `cargo test --lib dynamic_programming::longest_increasing_subsequence` |
| [Matrix Chain Multiply](../../problems/dynamic_programming/matrix_chain_multiply/README.md) | [`matrix_chain_multiply.rs`](./matrix_chain_multiply.rs) | `java Solution.java` | `cargo test --lib dynamic_programming::matrix_chain_multiply` |
| [Maximal Square](../../problems/dynamic_programming/maximal_square/README.md) | [`maximal_square.rs`](./maximal_square.rs) | `java Solution.java` | `cargo test --lib dynamic_programming::maximal_square` |
| [Maximum Subarray](../../problems/dynamic_programming/maximum_subarray/README.md) | [`maximum_subarray.rs`](./maximum_subarray.rs) | `java Solution.java` | `cargo test --lib dynamic_programming::maximum_subarray` |
| [Minimum Cost Path](../../problems/dynamic_programming/minimum_cost_path/README.md) | [`minimum_cost_path.rs`](./minimum_cost_path.rs) | `java Solution.java` | `cargo test --lib dynamic_programming::minimum_cost_path` |
| [Optimal BST](../../problems/dynamic_programming/optimal_bst/README.md) | [`optimal_bst.rs`](./optimal_bst.rs) | `java Solution.java` | `cargo test --lib dynamic_programming::optimal_bst` |
| [Palindrome Partitioning](../../problems/dynamic_programming/palindrome_partitioning/README.md) | [`palindrome_partitioning.rs`](./palindrome_partitioning.rs) | `java Solution.java` | `cargo test --lib dynamic_programming::palindrome_partitioning` |
| [Rod Cutting](../../problems/dynamic_programming/rod_cutting/README.md) | [`rod_cutting.rs`](./rod_cutting.rs) | `java Solution.java` | `cargo test --lib dynamic_programming::rod_cutting` |
| [Smith Waterman](../../problems/dynamic_programming/smith_waterman/README.md) | [`smith_waterman.rs`](./smith_waterman.rs) | `java Solution.java` | `cargo test --lib dynamic_programming::smith_waterman` |
| [Snail](../../problems/dynamic_programming/snail/README.md) | [`snail.rs`](./snail.rs) | `java Solution.java` | `cargo test --lib dynamic_programming::snail` |
| [Subset Generation](../../problems/dynamic_programming/subset_generation/README.md) | [`subset_generation.rs`](./subset_generation.rs) | `java Solution.java` | `cargo test --lib dynamic_programming::subset_generation` |
| [Subset Sum](../../problems/dynamic_programming/subset_sum/README.md) | [`subset_sum.rs`](./subset_sum.rs) | `java Solution.java` | `cargo test --lib dynamic_programming::subset_sum` |
| [Task Assignment](../../problems/dynamic_programming/task_assignment/README.md) | [`task_assignment.rs`](./task_assignment.rs) | `java Solution.java` | `cargo test --lib dynamic_programming::task_assignment` |
| [Trapped Rainwater](../../problems/dynamic_programming/trapped_rainwater/README.md) | [`trapped_rainwater.rs`](./trapped_rainwater.rs) | `java Solution.java` | `cargo test --lib dynamic_programming::trapped_rainwater` |
| [Word Break](../../problems/dynamic_programming/word_break/README.md) | [`word_break.rs`](./word_break.rs) | `java Solution.java` | `cargo test --lib dynamic_programming::word_break` |

---
*Generated for Java Software Engineering Interview Preparation.*
