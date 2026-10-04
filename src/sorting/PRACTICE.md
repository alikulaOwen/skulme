# Sorting - Algorithm Practice & Interview Guide

Sorting algorithms arrange items of a list into a specific order (ascending or descending).

## Key Java Interview Takeaways
- Java's `Arrays.sort(primitive[])` uses Dual-Pivot Quicksort (O(N log N) average, O(N^2) worst case).
- `Arrays.sort(Object[])` and `Collections.sort(List)` use Timsort (guaranteed stable, O(N log N) worst case).
- For custom ordering, use `Comparator.comparingInt(...)` or `(a, b) -> Integer.compare(a, b)`. NEVER use `a - b` due to integer underflow/overflow risk.
- Remember stability: stable sorts preserve the relative order of duplicate elements.

## Comparison: Rust vs Java
- Rust's `slice::sort()` is stable Timsort/pdqsort; `slice::sort_unstable()` is in-place pattern-defeating quicksort.
- Java distinguishes primitive arrays (`int[]`) from object arrays (`Integer[]`), whereas Rust treats all types uniformly under generics `T: Ord`.

---

## Problems & Practice Workspaces (36 Problems)

Each problem has a dedicated workspace with problem statements, runnable Java apps (`java Solution.java`), Python (`python3 solution.py`), TypeScript (`bun solution.ts`), and comparison to the original Rust implementation.

| Problem | Rust Source | Java Executable | Rust Reference Test |
| :--- | :--- | :--- | :--- |
| [Bead Sort](../../problems/sorting/bead_sort/README.md) | [`bead_sort.rs`](./bead_sort.rs) | `java Solution.java` | `cargo test --lib sorting::bead_sort` |
| [Binary Insertion Sort](../../problems/sorting/binary_insertion_sort/README.md) | [`binary_insertion_sort.rs`](./binary_insertion_sort.rs) | `java Solution.java` | `cargo test --lib sorting::binary_insertion_sort` |
| [Bingo Sort](../../problems/sorting/bingo_sort/README.md) | [`bingo_sort.rs`](./bingo_sort.rs) | `java Solution.java` | `cargo test --lib sorting::bingo_sort` |
| [Bitonic Sort](../../problems/sorting/bitonic_sort/README.md) | [`bitonic_sort.rs`](./bitonic_sort.rs) | `java Solution.java` | `cargo test --lib sorting::bitonic_sort` |
| [Bogo Sort](../../problems/sorting/bogo_sort/README.md) | [`bogo_sort.rs`](./bogo_sort.rs) | `java Solution.java` | `cargo test --lib sorting::bogo_sort` |
| [Bubble Sort](../../problems/sorting/bubble_sort/README.md) | [`bubble_sort.rs`](./bubble_sort.rs) | `java Solution.java` | `cargo test --lib sorting::bubble_sort` |
| [Bucket Sort](../../problems/sorting/bucket_sort/README.md) | [`bucket_sort.rs`](./bucket_sort.rs) | `java Solution.java` | `cargo test --lib sorting::bucket_sort` |
| [Cocktail Shaker Sort](../../problems/sorting/cocktail_shaker_sort/README.md) | [`cocktail_shaker_sort.rs`](./cocktail_shaker_sort.rs) | `java Solution.java` | `cargo test --lib sorting::cocktail_shaker_sort` |
| [Comb Sort](../../problems/sorting/comb_sort/README.md) | [`comb_sort.rs`](./comb_sort.rs) | `java Solution.java` | `cargo test --lib sorting::comb_sort` |
| [Counting Sort](../../problems/sorting/counting_sort/README.md) | [`counting_sort.rs`](./counting_sort.rs) | `java Solution.java` | `cargo test --lib sorting::counting_sort` |
| [Cycle Sort](../../problems/sorting/cycle_sort/README.md) | [`cycle_sort.rs`](./cycle_sort.rs) | `java Solution.java` | `cargo test --lib sorting::cycle_sort` |
| [Dutch National Flag Sort](../../problems/sorting/dutch_national_flag_sort/README.md) | [`dutch_national_flag_sort.rs`](./dutch_national_flag_sort.rs) | `java Solution.java` | `cargo test --lib sorting::dutch_national_flag_sort` |
| [Exchange Sort](../../problems/sorting/exchange_sort/README.md) | [`exchange_sort.rs`](./exchange_sort.rs) | `java Solution.java` | `cargo test --lib sorting::exchange_sort` |
| [Gnome Sort](../../problems/sorting/gnome_sort/README.md) | [`gnome_sort.rs`](./gnome_sort.rs) | `java Solution.java` | `cargo test --lib sorting::gnome_sort` |
| [Heap Sort](../../problems/sorting/heap_sort/README.md) | [`heap_sort.rs`](./heap_sort.rs) | `java Solution.java` | `cargo test --lib sorting::heap_sort` |
| [Insertion Sort](../../problems/sorting/insertion_sort/README.md) | [`insertion_sort.rs`](./insertion_sort.rs) | `java Solution.java` | `cargo test --lib sorting::insertion_sort` |
| [Intro Sort](../../problems/sorting/intro_sort/README.md) | [`intro_sort.rs`](./intro_sort.rs) | `java Solution.java` | `cargo test --lib sorting::intro_sort` |
| [Merge Sort](../../problems/sorting/merge_sort/README.md) | [`merge_sort.rs`](./merge_sort.rs) | `java Solution.java` | `cargo test --lib sorting::merge_sort` |
| [Odd Even Sort](../../problems/sorting/odd_even_sort/README.md) | [`odd_even_sort.rs`](./odd_even_sort.rs) | `java Solution.java` | `cargo test --lib sorting::odd_even_sort` |
| [Pancake Sort](../../problems/sorting/pancake_sort/README.md) | [`pancake_sort.rs`](./pancake_sort.rs) | `java Solution.java` | `cargo test --lib sorting::pancake_sort` |
| [Patience Sort](../../problems/sorting/patience_sort/README.md) | [`patience_sort.rs`](./patience_sort.rs) | `java Solution.java` | `cargo test --lib sorting::patience_sort` |
| [Pigeonhole Sort](../../problems/sorting/pigeonhole_sort/README.md) | [`pigeonhole_sort.rs`](./pigeonhole_sort.rs) | `java Solution.java` | `cargo test --lib sorting::pigeonhole_sort` |
| [Quick Sort](../../problems/sorting/quick_sort/README.md) | [`quick_sort.rs`](./quick_sort.rs) | `java Solution.java` | `cargo test --lib sorting::quick_sort` |
| [Quick Sort 3 Ways](../../problems/sorting/quick_sort_3_ways/README.md) | [`quick_sort_3_ways.rs`](./quick_sort_3_ways.rs) | `java Solution.java` | `cargo test --lib sorting::quick_sort_3_ways` |
| [Radix Sort](../../problems/sorting/radix_sort/README.md) | [`radix_sort.rs`](./radix_sort.rs) | `java Solution.java` | `cargo test --lib sorting::radix_sort` |
| [Selection Sort](../../problems/sorting/selection_sort/README.md) | [`selection_sort.rs`](./selection_sort.rs) | `java Solution.java` | `cargo test --lib sorting::selection_sort` |
| [Shell Sort](../../problems/sorting/shell_sort/README.md) | [`shell_sort.rs`](./shell_sort.rs) | `java Solution.java` | `cargo test --lib sorting::shell_sort` |
| [Sleep Sort](../../problems/sorting/sleep_sort/README.md) | [`sleep_sort.rs`](./sleep_sort.rs) | `java Solution.java` | `cargo test --lib sorting::sleep_sort` |
| [Sort Utils](../../problems/sorting/sort_utils/README.md) | [`sort_utils.rs`](./sort_utils.rs) | `java Solution.java` | `cargo test --lib sorting::sort_utils` |
| [Stooge Sort](../../problems/sorting/stooge_sort/README.md) | [`stooge_sort.rs`](./stooge_sort.rs) | `java Solution.java` | `cargo test --lib sorting::stooge_sort` |
| [Strand Sort](../../problems/sorting/strand_sort/README.md) | [`strand_sort.rs`](./strand_sort.rs) | `java Solution.java` | `cargo test --lib sorting::strand_sort` |
| [Tim Sort](../../problems/sorting/tim_sort/README.md) | [`tim_sort.rs`](./tim_sort.rs) | `java Solution.java` | `cargo test --lib sorting::tim_sort` |
| [Tournament Sort](../../problems/sorting/tournament_sort/README.md) | [`tournament_sort.rs`](./tournament_sort.rs) | `java Solution.java` | `cargo test --lib sorting::tournament_sort` |
| [Tree Sort](../../problems/sorting/tree_sort/README.md) | [`tree_sort.rs`](./tree_sort.rs) | `java Solution.java` | `cargo test --lib sorting::tree_sort` |
| [Wave Sort](../../problems/sorting/wave_sort/README.md) | [`wave_sort.rs`](./wave_sort.rs) | `java Solution.java` | `cargo test --lib sorting::wave_sort` |
| [Wiggle Sort](../../problems/sorting/wiggle_sort/README.md) | [`wiggle_sort.rs`](./wiggle_sort.rs) | `java Solution.java` | `cargo test --lib sorting::wiggle_sort` |

---
*Generated for Java Software Engineering Interview Preparation.*
