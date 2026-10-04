//! Vigenère Cipher
//!
//! # Algorithm
//!
//! Rotate each ascii character by the offset of the corresponding key character.
//! When we reach the last key character, we start over from the first one.
//! This implementation does not rotate unicode characters.

/// Vigenère cipher to rotate plain_text text by key and return an owned String.
pub fn vigenere(plain_text: &str, key: &str) -> String {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (vigenere)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement vigenere");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(vigenere("", "test"), "");
    }

    #[test]
    fn vigenere_base() {
        assert_eq!(
            vigenere("LoremIpsumDolorSitAmet", "base"),
            "MojinIhwvmVsmojWjtSqft"
        );
    }

    #[test]
    fn vigenere_with_spaces() {
        assert_eq!(
            vigenere(
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
                "spaces"
            ),
            "Ddrgq ahhuo hgddr uml sbev, ggfheexwljr chahxsemfy tlkx."
        );
    }

    #[test]
    fn vigenere_unicode_and_numbers() {
        assert_eq!(
            vigenere("1 Lorem ⏳ ipsum dolor sit amet Ѡ", "unicode"),
            "1 Fbzga ⏳ ltmhu fcosl fqv opin Ѡ"
        );
    }

    #[test]
    fn vigenere_unicode_key() {
        assert_eq!(
            vigenere("Lorem ipsum dolor sit amet", "😉 key!"),
            "Vspoq gzwsw hmvsp cmr kqcd"
        );
    }

    #[test]
    fn vigenere_empty_key() {
        assert_eq!(vigenere("Lorem ipsum", ""), "Lorem ipsum");
    }
}
