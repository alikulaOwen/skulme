/// Encode an ASCII string into its location in a Polybius square.
/// Only alphabetical characters are encoded.
pub fn encode_ascii(string: &str) -> String {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (encode_ascii)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement encode_ascii");
}

/// Decode a string of ints into their corresponding
/// letters in a Polybius square.
///
/// Any invalid characters, or whitespace will be ignored.
pub fn decode_ascii(string: &str) -> String {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (decode_ascii)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement decode_ascii");
}


#[cfg(test)]
mod tests {
    use super::{decode_ascii, encode_ascii};

    #[test]
    fn encode_empty() {
        assert_eq!(encode_ascii(""), "");
    }

    #[test]
    fn encode_valid_string() {
        assert_eq!(encode_ascii("This is a test"), "4423244324431144154344");
    }

    #[test]
    fn encode_emoji() {
        assert_eq!(encode_ascii("🙂"), "");
    }

    #[test]
    fn decode_empty() {
        assert_eq!(decode_ascii(""), "");
    }

    #[test]
    fn decode_valid_string() {
        assert_eq!(
            decode_ascii("44 23 24 43 24 43 11 44 15 43 44 "),
            "THISISATEST"
        );
    }

    #[test]
    fn decode_emoji() {
        assert_eq!(decode_ascii("🙂"), "");
    }

    #[test]
    fn decode_string_with_whitespace() {
        assert_eq!(
            decode_ascii("44\n23\t\r24\r\n43   2443\n 11 \t 44\r \r15 \n43 44"),
            "THISISATEST"
        );
    }

    #[test]
    fn decode_unknown_string() {
        assert_eq!(decode_ascii("94 63 64 83 64 48 77 00 05 47 48 "), "");
    }

    #[test]
    fn decode_odd_length() {
        assert_eq!(decode_ascii("11 22 33 4"), "AGN");
    }

    #[test]
    fn encode_and_decode() {
        let string = "Do you ever wonder why we're here?";
        let encode = encode_ascii(string);
        assert_eq!(
            "1434543445155115425234331415425223545215421523154215",
            encode,
        );
        assert_eq!("DOYOUEVERWONDERWHYWEREHERE", decode_ascii(&encode));
    }
}
