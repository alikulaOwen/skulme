# String - Algorithm Practice & Interview Guide

String algorithms for pattern matching, parsing, hashing, and substring manipulations.

## Key Java Interview Takeaways
- Strings in Java are immutable! `str += "a"` creates a brand new String object each time. Inside loops, ALWAYS use `StringBuilder`.
- Access characters via `str.charAt(i)` and length via `str.length()`.
- Compare strings with `str1.equals(str2)`, NEVER with `str1 == str2` (which checks reference equality!).
- To convert to char array for fast swaps: `char[] chars = str.toCharArray()`.

## Comparison: Rust vs Java
- Rust strings are UTF-8 bytes (`String`, `&str`), so direct byte indexing `s[i]` is prohibited if characters span multiple bytes.
- Java strings use UTF-16 code units (`char`), and provide O(1) indexed `charAt(i)`.

---

## Problems & Practice Workspaces (24 Problems)

Each problem has a dedicated workspace with problem statements, runnable Java apps (`java Solution.java`), Python (`python3 solution.py`), TypeScript (`bun solution.ts`), and comparison to the original Rust implementation.

| Problem | Rust Source | Java Executable | Rust Reference Test |
| :--- | :--- | :--- | :--- |
| [Aho Corasick](../../problems/string/aho_corasick/README.md) | [`aho_corasick.rs`](./aho_corasick.rs) | `java Solution.java` | `cargo test --lib string::aho_corasick` |
| [Anagram](../../problems/string/anagram/README.md) | [`anagram.rs`](./anagram.rs) | `java Solution.java` | `cargo test --lib string::anagram` |
| [Autocomplete Using Trie](../../problems/string/autocomplete_using_trie/README.md) | [`autocomplete_using_trie.rs`](./autocomplete_using_trie.rs) | `java Solution.java` | `cargo test --lib string::autocomplete_using_trie` |
| [Boyer Moore Search](../../problems/string/boyer_moore_search/README.md) | [`boyer_moore_search.rs`](./boyer_moore_search.rs) | `java Solution.java` | `cargo test --lib string::boyer_moore_search` |
| [Burrows Wheeler Transform](../../problems/string/burrows_wheeler_transform/README.md) | [`burrows_wheeler_transform.rs`](./burrows_wheeler_transform.rs) | `java Solution.java` | `cargo test --lib string::burrows_wheeler_transform` |
| [Duval Algorithm](../../problems/string/duval_algorithm/README.md) | [`duval_algorithm.rs`](./duval_algorithm.rs) | `java Solution.java` | `cargo test --lib string::duval_algorithm` |
| [Hamming Distance](../../problems/string/hamming_distance/README.md) | [`hamming_distance.rs`](./hamming_distance.rs) | `java Solution.java` | `cargo test --lib string::hamming_distance` |
| [Isogram](../../problems/string/isogram/README.md) | [`isogram.rs`](./isogram.rs) | `java Solution.java` | `cargo test --lib string::isogram` |
| [Isomorphism](../../problems/string/isomorphism/README.md) | [`isomorphism.rs`](./isomorphism.rs) | `java Solution.java` | `cargo test --lib string::isomorphism` |
| [Jaro Winkler Distance](../../problems/string/jaro_winkler_distance/README.md) | [`jaro_winkler_distance.rs`](./jaro_winkler_distance.rs) | `java Solution.java` | `cargo test --lib string::jaro_winkler_distance` |
| [Knuth Morris Pratt](../../problems/string/knuth_morris_pratt/README.md) | [`knuth_morris_pratt.rs`](./knuth_morris_pratt.rs) | `java Solution.java` | `cargo test --lib string::knuth_morris_pratt` |
| [Levenshtein Distance](../../problems/string/levenshtein_distance/README.md) | [`levenshtein_distance.rs`](./levenshtein_distance.rs) | `java Solution.java` | `cargo test --lib string::levenshtein_distance` |
| [Lipogram](../../problems/string/lipogram/README.md) | [`lipogram.rs`](./lipogram.rs) | `java Solution.java` | `cargo test --lib string::lipogram` |
| [Manacher](../../problems/string/manacher/README.md) | [`manacher.rs`](./manacher.rs) | `java Solution.java` | `cargo test --lib string::manacher` |
| [Palindrome](../../problems/string/palindrome/README.md) | [`palindrome.rs`](./palindrome.rs) | `java Solution.java` | `cargo test --lib string::palindrome` |
| [Pangram](../../problems/string/pangram/README.md) | [`pangram.rs`](./pangram.rs) | `java Solution.java` | `cargo test --lib string::pangram` |
| [Rabin Karp](../../problems/string/rabin_karp/README.md) | [`rabin_karp.rs`](./rabin_karp.rs) | `java Solution.java` | `cargo test --lib string::rabin_karp` |
| [Reverse](../../problems/string/reverse/README.md) | [`reverse.rs`](./reverse.rs) | `java Solution.java` | `cargo test --lib string::reverse` |
| [Run Length Encoding](../../problems/string/run_length_encoding/README.md) | [`run_length_encoding.rs`](./run_length_encoding.rs) | `java Solution.java` | `cargo test --lib string::run_length_encoding` |
| [Shortest Palindrome](../../problems/string/shortest_palindrome/README.md) | [`shortest_palindrome.rs`](./shortest_palindrome.rs) | `java Solution.java` | `cargo test --lib string::shortest_palindrome` |
| [Suffix Array](../../problems/string/suffix_array/README.md) | [`suffix_array.rs`](./suffix_array.rs) | `java Solution.java` | `cargo test --lib string::suffix_array` |
| [Suffix Array Manber Myers](../../problems/string/suffix_array_manber_myers/README.md) | [`suffix_array_manber_myers.rs`](./suffix_array_manber_myers.rs) | `java Solution.java` | `cargo test --lib string::suffix_array_manber_myers` |
| [Suffix Tree](../../problems/string/suffix_tree/README.md) | [`suffix_tree.rs`](./suffix_tree.rs) | `java Solution.java` | `cargo test --lib string::suffix_tree` |
| [Z Algorithm](../../problems/string/z_algorithm/README.md) | [`z_algorithm.rs`](./z_algorithm.rs) | `java Solution.java` | `cargo test --lib string::z_algorithm` |

---
*Generated for Java Software Engineering Interview Preparation.*
