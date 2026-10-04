//! Vernam Cipher
//!
//! The Vernam cipher is a symmetric stream cipher where plaintext is combined
//! with a random or pseudorandom stream of data (the key) of the same length.
//! This implementation uses the alphabet A-Z with modular arithmetic.
//!
//! # Algorithm
//!
//! For encryption: C = (P + K) mod 26
//! For decryption: P = (C - K) mod 26
//!
//! Where P is plaintext, K is key, and C is ciphertext (all converted to 0-25 range)

/// Encrypts a plaintext string using the Vernam cipher.
///
/// The function converts all input to uppercase and works only with letters A-Z.
/// The key is repeated cyclically if it's shorter than the plaintext.
///
/// # Arguments
///
/// * `plaintext` - The text to encrypt (will be converted to uppercase)
/// * `key` - The encryption key (will be converted to uppercase, must not be empty)
///
/// # Returns
///
/// The encrypted ciphertext as an uppercase string
///
/// # Panics
///
/// Panics if the key is empty
///
/// # Example
///
/// ```
/// use the_algorithms_rust::ciphers::vernam_encrypt;
///
/// let ciphertext = vernam_encrypt("HELLO", "KEY");
/// assert_eq!(ciphertext, "RIJVS");
/// ```
pub fn vernam_encrypt(plaintext: &str, key: &str) -> String {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (vernam_encrypt)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement vernam_encrypt");
}

/// Decrypts a ciphertext string using the Vernam cipher.
///
/// The function converts all input to uppercase and works only with letters A-Z.
/// The key is repeated cyclically if it's shorter than the ciphertext.
///
/// # Arguments
///
/// * `ciphertext` - The text to decrypt (will be converted to uppercase)
/// * `key` - The decryption key (will be converted to uppercase, must not be empty)
///
/// # Returns
///
/// The decrypted plaintext as an uppercase string
///
/// # Panics
///
/// Panics if the key is empty
///
/// # Example
///
/// ```
/// use the_algorithms_rust::ciphers::vernam_decrypt;
///
/// let plaintext = vernam_decrypt("RIJVS", "KEY");
/// assert_eq!(plaintext, "HELLO");
/// ```
pub fn vernam_decrypt(ciphertext: &str, key: &str) -> String {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (vernam_decrypt)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement vernam_decrypt");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_basic() {
        assert_eq!(vernam_encrypt("HELLO", "KEY"), "RIJVS");
    }

    #[test]
    fn test_decrypt_basic() {
        assert_eq!(vernam_decrypt("RIJVS", "KEY"), "HELLO");
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let plaintext = "HELLO";
        let key = "KEY";
        let encrypted = vernam_encrypt(plaintext, key);
        let decrypted = vernam_decrypt(&encrypted, key);
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_encrypt_decrypt_long_text() {
        let plaintext = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
        let key = "SECRET";
        let encrypted = vernam_encrypt(plaintext, key);
        let decrypted = vernam_decrypt(&encrypted, key);
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_lowercase_input() {
        // Should convert to uppercase
        assert_eq!(vernam_encrypt("hello", "key"), "RIJVS");
        assert_eq!(vernam_decrypt("rijvs", "key"), "HELLO");
    }

    #[test]
    fn test_mixed_case_input() {
        assert_eq!(vernam_encrypt("HeLLo", "KeY"), "RIJVS");
        assert_eq!(vernam_decrypt("RiJvS", "kEy"), "HELLO");
    }

    #[test]
    fn test_single_character() {
        assert_eq!(vernam_encrypt("A", "B"), "B");
        assert_eq!(vernam_decrypt("B", "B"), "A");
    }

    #[test]
    fn test_key_wrapping() {
        // Key shorter than plaintext, should wrap around
        let encrypted = vernam_encrypt("AAAA", "BC");
        assert_eq!(encrypted, "BCBC");
        let decrypted = vernam_decrypt(&encrypted, "BC");
        assert_eq!(decrypted, "AAAA");
    }

    #[test]
    fn test_alphabet_boundary() {
        // Test wrapping at alphabet boundaries
        assert_eq!(vernam_encrypt("Z", "B"), "A"); // 25 + 1 = 26 -> 0
        assert_eq!(vernam_decrypt("A", "B"), "Z"); // 0 - 1 = -1 -> 25
    }

    #[test]
    fn test_same_key_as_plaintext() {
        let text = "HELLO";
        let encrypted = vernam_encrypt(text, text);
        assert_eq!(encrypted, "OIWWC");
    }

    #[test]
    fn test_with_spaces_and_numbers() {
        // Non-alphabetic characters should be filtered out
        let encrypted = vernam_encrypt("HELLO 123 WORLD", "KEY");
        let expected = vernam_encrypt("HELLOWORLD", "KEY");
        assert_eq!(encrypted, expected);
    }

    #[test]
    #[should_panic(expected = "Key cannot be empty")]
    fn test_empty_key_encrypt() {
        vernam_encrypt("HELLO", "");
    }

    #[test]
    #[should_panic(expected = "Key cannot be empty")]
    fn test_empty_key_decrypt() {
        vernam_decrypt("HELLO", "");
    }

    #[test]
    #[should_panic(expected = "Key must contain at least one letter")]
    fn test_key_with_only_numbers() {
        vernam_encrypt("HELLO", "12345");
    }

    #[test]
    fn test_empty_plaintext() {
        assert_eq!(vernam_encrypt("", "KEY"), "");
    }

    #[test]
    fn test_plaintext_with_only_numbers() {
        assert_eq!(vernam_encrypt("12345", "KEY"), "");
    }
}
