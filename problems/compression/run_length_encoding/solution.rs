// https://en.wikipedia.org/wiki/Run-length_encoding

pub fn run_length_encode(text: &str) -> Vec<(char, i32)> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (run_length_encode)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement run_length_encode");
}

pub fn run_length_decode(encoded: &[(char, i32)]) -> String {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (run_length_decode)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement run_length_decode");
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run_length_decode() {
        let res = run_length_decode(&[('A', 0)]);
        assert_eq!(res, "");
        let res = run_length_decode(&[('B', 1)]);
        assert_eq!(res, "B");
        let res = run_length_decode(&[('A', 5), ('z', 3), ('B', 1)]);
        assert_eq!(res, "AAAAAzzzB");
    }

    #[test]
    fn test_run_length_encode() {
        let res = run_length_encode("");
        assert_eq!(res, []);

        let res = run_length_encode("A");
        assert_eq!(res, [('A', 1)]);

        let res = run_length_encode("AA");
        assert_eq!(res, [('A', 2)]);

        let res = run_length_encode("AAAABBBCCDAA");
        assert_eq!(res, [('A', 4), ('B', 3), ('C', 2), ('D', 1), ('A', 2)]);

        let res = run_length_encode("Rust-Trends");
        assert_eq!(
            res,
            [
                ('R', 1),
                ('u', 1),
                ('s', 1),
                ('t', 1),
                ('-', 1),
                ('T', 1),
                ('r', 1),
                ('e', 1),
                ('n', 1),
                ('d', 1),
                ('s', 1)
            ]
        );
    }
}
