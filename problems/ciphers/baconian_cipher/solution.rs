// Author : cyrixninja
//Program to encode and decode Baconian or Bacon's Cipher
//Wikipedia reference : https://en.wikipedia.org/wiki/Bacon%27s_cipher
// Bacon's cipher or the Baconian cipher is a method of steganographic message encoding devised by Francis Bacon in 1605.
// A message is concealed in the presentation of text, rather than its content. Bacon cipher is categorized as both a substitution cipher (in plain code) and a concealment cipher (using the two typefaces).

// Encode Baconian Cipher
pub fn baconian_encode(message: &str) -> String {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (baconian_encode)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement baconian_encode");
}

// Decode Baconian Cipher
pub fn baconian_decode(encoded: &str) -> String {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (baconian_decode)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement baconian_decode");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_baconian_encoding() {
        let message = "HELLO";
        let encoded = baconian_encode(message);
        assert_eq!(encoded, "AABBBAABAAABABBABABBABBBA");
    }

    #[test]
    fn test_baconian_decoding() {
        let message = "AABBBAABAAABABBABABBABBBA";
        let decoded = baconian_decode(message);
        assert_eq!(decoded, "HELLO");
    }
}
