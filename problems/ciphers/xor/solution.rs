pub fn xor_bytes(text: &[u8], key: u8) -> Vec<u8> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (xor_bytes)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement xor_bytes");
}

pub fn xor(text: &str, key: u8) -> Vec<u8> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (xor)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement xor");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let test_string = "test string";
        let ciphered_text = xor(test_string, 32);
        assert_eq!(test_string.as_bytes(), xor_bytes(&ciphered_text, 32));
    }

    #[test]
    fn test_every_alphabet_with_space() {
        let test_string = "The quick brown fox jumps over the lazy dog";
        let ciphered_text = xor(test_string, 64);
        assert_eq!(test_string.as_bytes(), xor_bytes(&ciphered_text, 64));
    }

    #[test]
    fn test_multi_byte() {
        let test_string = "日本語";
        let key = 42;
        let ciphered_text = xor(test_string, key);
        assert_eq!(test_string.as_bytes(), xor_bytes(&ciphered_text, key));
    }

    #[test]
    fn test_zero_byte() {
        let test_string = "The quick brown fox jumps over the lazy dog";
        let key = b' ';
        let ciphered_text = xor(test_string, key);
        assert_eq!(test_string.as_bytes(), xor_bytes(&ciphered_text, key));
    }

    #[test]
    fn test_invalid_byte() {
        let test_string = "The quick brown fox jumps over the lazy dog";
        let key = !0;
        let ciphered_text = xor(test_string, key);
        assert_eq!(test_string.as_bytes(), xor_bytes(&ciphered_text, key));
    }
}
