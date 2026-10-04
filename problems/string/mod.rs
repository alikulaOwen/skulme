// Automatically generated category module

#[path = "aho_corasick/solution.rs"]
pub mod aho_corasick;

#[path = "anagram/solution.rs"]
pub mod anagram;

#[path = "autocomplete_using_trie/solution.rs"]
pub mod autocomplete_using_trie;

#[path = "boyer_moore_search/solution.rs"]
pub mod boyer_moore_search;

#[path = "burrows_wheeler_transform/solution.rs"]
pub mod burrows_wheeler_transform;

#[path = "duval_algorithm/solution.rs"]
pub mod duval_algorithm;

#[path = "hamming_distance/solution.rs"]
pub mod hamming_distance;

#[path = "isogram/solution.rs"]
pub mod isogram;

#[path = "isomorphism/solution.rs"]
pub mod isomorphism;

#[path = "jaro_winkler_distance/solution.rs"]
pub mod jaro_winkler_distance;

#[path = "knuth_morris_pratt/solution.rs"]
pub mod knuth_morris_pratt;

#[path = "levenshtein_distance/solution.rs"]
pub mod levenshtein_distance;

#[path = "lipogram/solution.rs"]
pub mod lipogram;

#[path = "manacher/solution.rs"]
pub mod manacher;

#[path = "palindrome/solution.rs"]
pub mod palindrome;

#[path = "pangram/solution.rs"]
pub mod pangram;

#[path = "rabin_karp/solution.rs"]
pub mod rabin_karp;

#[path = "reverse/solution.rs"]
pub mod reverse;

#[path = "run_length_encoding/solution.rs"]
pub mod run_length_encoding;

#[path = "shortest_palindrome/solution.rs"]
pub mod shortest_palindrome;

#[path = "suffix_array/solution.rs"]
pub mod suffix_array;

#[path = "suffix_array_manber_myers/solution.rs"]
pub mod suffix_array_manber_myers;

#[path = "suffix_tree/solution.rs"]
pub mod suffix_tree;

#[path = "z_algorithm/solution.rs"]
pub mod z_algorithm;


pub use self::aho_corasick::AhoCorasick;
pub use self::anagram::check_anagram;
pub use self::autocomplete_using_trie::Autocomplete;
pub use self::boyer_moore_search::boyer_moore_search;
pub use self::burrows_wheeler_transform::{
    burrows_wheeler_transform, inv_burrows_wheeler_transform,
};
pub use self::duval_algorithm::duval_algorithm;
pub use self::hamming_distance::hamming_distance;
pub use self::isogram::is_isogram;
pub use self::isomorphism::is_isomorphic;
pub use self::jaro_winkler_distance::jaro_winkler_distance;
pub use self::knuth_morris_pratt::knuth_morris_pratt;
pub use self::levenshtein_distance::{naive_levenshtein_distance, optimized_levenshtein_distance};
pub use self::lipogram::is_lipogram;
pub use self::manacher::manacher;
pub use self::palindrome::is_palindrome;
pub use self::pangram::is_pangram;
pub use self::pangram::PangramStatus;
pub use self::rabin_karp::rabin_karp;
pub use self::reverse::reverse;
pub use self::run_length_encoding::{run_length_decoding, run_length_encoding};
pub use self::shortest_palindrome::shortest_palindrome;
pub use self::suffix_array::generate_suffix_array;
pub use self::suffix_array_manber_myers::generate_suffix_array_manber_myers;
pub use self::suffix_tree::{Node, SuffixTree};
pub use self::z_algorithm::match_pattern;
pub use self::z_algorithm::z_array;
