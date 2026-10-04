pub fn run_length_encoding(target: &str) -> String {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (run_length_encoding)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement run_length_encoding");
}

pub fn run_length_decoding(target: &str) -> String {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (run_length_decoding)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement run_length_decoding");
}


#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_run_length {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (raw_str, encoded) = $test_case;
                    assert_eq!(run_length_encoding(raw_str), encoded);
                    assert_eq!(run_length_decoding(encoded), raw_str);
                }
            )*
        };
    }

    test_run_length! {
        empty_input: ("", ""),
        repeated_char: ("aaaaaaaaaa", "10a"),
        no_repeated: ("abcdefghijk", "1a1b1c1d1e1f1g1h1i1j1k"),
        regular_input: ("aaaaabbbcccccdddddddddd", "5a3b5c10d"),
        two_blocks_with_same_char: ("aaabbaaaa", "3a2b4a"),
        long_input: ("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabbbcccccdddddddddd", "200a3b5c10d"),
    }
}
