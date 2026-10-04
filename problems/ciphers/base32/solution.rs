//! Base32 encoding and decoding implementation.
//!
//! Base32 is a binary-to-text encoding scheme that represents binary data using 32 ASCII characters
//! (A-Z and 2-7). It's commonly used when case-insensitive encoding is needed or when avoiding
//! characters that might be confused (like 0/O or 1/l).
//!
//! This implementation follows the standard Base32 alphabet as defined in RFC 4648.

const B32_CHARSET: &[u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

/// Encodes the given bytes into base32.
///
/// The function converts binary data into base32 format using the standard alphabet.
/// Output is padded with '=' characters to make the length a multiple of 8.
///
/// # Arguments
///
/// * `data` - A byte slice to encode
///
/// # Returns
///
/// A `Vec<u8>` containing the base32-encoded data with padding.
///
/// # Examples
///
/// ```
/// use the_algorithms_rust::ciphers::base32_encode;
/// assert_eq!(base32_encode(b"Hello World!"), b"JBSWY3DPEBLW64TMMQQQ====");
/// assert_eq!(base32_encode(b"123456"), b"GEZDGNBVGY======");
/// assert_eq!(base32_encode(b"some long complex string"), b"ONXW2ZJANRXW4ZZAMNXW24DMMV4CA43UOJUW4ZY=");
/// ```
pub fn base32_encode(data: &[u8]) -> Vec<u8> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (base32_encode)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement base32_encode");
}

/// Decodes base32-encoded data into bytes.
///
/// The function decodes base32 format back to binary data, removing padding characters.
///
/// # Arguments
///
/// * `data` - A byte slice containing base32-encoded data
///
/// # Returns
///
/// * `Ok(Vec<u8>)` - Successfully decoded bytes
/// * `Err(String)` - Error message if the input is invalid
///
/// # Errors
///
/// Returns an error if:
/// - The input contains invalid base32 characters
/// - The input cannot be properly decoded
///
/// # Examples
///
/// ```
/// use the_algorithms_rust::ciphers::base32_decode;
/// assert_eq!(base32_decode(b"JBSWY3DPEBLW64TMMQQQ====").unwrap(), b"Hello World!");
/// assert_eq!(base32_decode(b"GEZDGNBVGY======").unwrap(), b"123456");
/// assert_eq!(base32_decode(b"ONXW2ZJANRXW4ZZAMNXW24DMMV4CA43UOJUW4ZY=").unwrap(), b"some long complex string");
/// ```
pub fn base32_decode(data: &[u8]) -> Result<Vec<u8>, String> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (base32_decode)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement base32_decode");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_hello_world() {
        assert_eq!(base32_encode(b"Hello World!"), b"JBSWY3DPEBLW64TMMQQQ====");
    }

    #[test]
    fn test_encode_numbers() {
        assert_eq!(base32_encode(b"123456"), b"GEZDGNBVGY======");
    }

    #[test]
    fn test_encode_long_string() {
        assert_eq!(
            base32_encode(b"some long complex string"),
            b"ONXW2ZJANRXW4ZZAMNXW24DMMV4CA43UOJUW4ZY="
        );
    }

    #[test]
    fn test_encode_empty() {
        assert_eq!(base32_encode(b""), b"");
    }

    #[test]
    fn test_encode_single_char() {
        assert_eq!(base32_encode(b"A"), b"IE======");
    }

    #[test]
    fn test_decode_hello_world() {
        assert_eq!(
            base32_decode(b"JBSWY3DPEBLW64TMMQQQ====").unwrap(),
            b"Hello World!"
        );
    }

    #[test]
    fn test_decode_numbers() {
        assert_eq!(base32_decode(b"GEZDGNBVGY======").unwrap(), b"123456");
    }

    #[test]
    fn test_decode_long_string() {
        assert_eq!(
            base32_decode(b"ONXW2ZJANRXW4ZZAMNXW24DMMV4CA43UOJUW4ZY=").unwrap(),
            b"some long complex string"
        );
    }

    #[test]
    fn test_decode_empty() {
        assert_eq!(base32_decode(b"").unwrap(), b"");
    }

    #[test]
    fn test_decode_single_char() {
        assert_eq!(base32_decode(b"IE======").unwrap(), b"A");
    }

    #[test]
    fn test_decode_without_padding() {
        assert_eq!(
            base32_decode(b"JBSWY3DPEBLW64TMMQQQ").unwrap(),
            b"Hello World!"
        );
    }

    #[test]
    fn test_decode_invalid_character() {
        let result = base32_decode(b"INVALID!@#$");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid base32 character"));
    }

    #[test]
    fn test_roundtrip_hello() {
        let original = b"Hello";
        let encoded = base32_encode(original);
        let decoded = base32_decode(&encoded).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn test_roundtrip_various_strings() {
        let test_cases = vec![
            b"a" as &[u8],
            b"ab",
            b"abc",
            b"abcd",
            b"abcde",
            b"The quick brown fox jumps over the lazy dog",
            b"1234567890",
            b"!@#$%^&*()",
        ];

        for original in test_cases {
            let encoded = base32_encode(original);
            let decoded = base32_decode(&encoded).unwrap();
            assert_eq!(decoded, original, "Failed for: {original:?}");
        }
    }

    #[test]
    fn test_all_charset_characters() {
        // Test that all characters in the charset can be encoded/decoded
        for i in 0..32 {
            let data = vec![i * 8]; // Arbitrary byte values
            let encoded = base32_encode(&data);
            let decoded = base32_decode(&encoded).unwrap();
            assert_eq!(decoded, data);
        }
    }

    #[test]
    fn test_binary_data() {
        let binary_data = vec![0x00, 0x01, 0x02, 0xFF, 0xFE, 0xFD];
        let encoded = base32_encode(&binary_data);
        let decoded = base32_decode(&encoded).unwrap();
        assert_eq!(decoded, binary_data);
    }

    #[test]
    fn test_padding_variations() {
        // Test different amounts of padding
        let test_cases: Vec<(&[u8], &[u8])> = vec![
            (b"f", b"MY======"),
            (b"fo", b"MZXQ===="),
            (b"foo", b"MZXW6==="),
            (b"foob", b"MZXW6YQ="),
            (b"fooba", b"MZXW6YTB"),
            (b"foobar", b"MZXW6YTBOI======"),
        ];

        for (input, expected) in test_cases {
            let encoded = base32_encode(input);
            assert_eq!(encoded, expected, "Encoding failed for: {input:?}");
            let decoded = base32_decode(&encoded).unwrap();
            assert_eq!(decoded, input, "Roundtrip failed for: {input:?}");
        }
    }
}
