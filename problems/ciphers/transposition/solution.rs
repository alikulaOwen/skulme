//! Transposition Cipher
//!
//! The Transposition Cipher is a method of encryption by which a message is shifted
//! according to a regular system, so that the ciphertext is a rearrangement of the
//! original message. The most commonly referred to Transposition Cipher is the
//! COLUMNAR TRANSPOSITION cipher, which is demonstrated below.

use std::ops::RangeInclusive;

/// Encrypts or decrypts a message, using multiple keys. The
/// encryption is based on the columnar transposition method.
pub fn transposition(decrypt_mode: bool, msg: &str, key: &str) -> String {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (transposition)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement transposition");
}

/// Performs the columnar transposition encryption
fn encrypt(mut msg: String, key_order: Vec<usize>) -> String {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (encrypt)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement encrypt");
}

/// Performs the columnar transposition decryption
fn decrypt(mut msg: String, key_order: Vec<usize>) -> String {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (decrypt)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement decrypt");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encryption() {
        assert_eq!(
            transposition(
                false,
                "The quick brown fox jumps over the lazy dog",
                "Archive",
            ),
            "TKOOL ERJEZ CFSEG QOURY UWMTD HBXVA INPHO"
        );

        assert_eq!(
            transposition(
                false,
                "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ.,/;'[]{}:|_+=-`~() ",
                "Tenacious"
            ),
            "DMVENW ENWFOX BKTCLU FOXGPY CLUDMV GPYHQZ IRAJSA JSBKTH QZIR"
        );

        assert_eq!(
            transposition(false, "WE ARE DISCOVERED. FLEE AT ONCE.", "ZEBRAS"),
            "EVLNA CDTES EAROF ODEEC WIREE"
        );
    }

    #[test]
    fn decryption() {
        assert_eq!(
            transposition(true, "TKOOL ERJEZ CFSEG QOURY UWMTD HBXVA INPHO", "Archive"),
            "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG"
        );

        assert_eq!(
            transposition(
                true,
                "DMVENW ENWFOX BKTCLU FOXGPY CLUDMV GPYHQZ IRAJSA JSBKTH QZIR",
                "Tenacious"
            ),
            "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
        );

        assert_eq!(
            transposition(true, "EVLNA CDTES EAROF ODEEC WIREE", "ZEBRAS"),
            "WEAREDISCOVEREDFLEEATONCE"
        );
    }

    #[test]
    fn double_encryption() {
        assert_eq!(
            transposition(
                false,
                "The quick brown fox jumps over the lazy dog",
                "Archive Snow"
            ),
            "KEZEUWHAH ORCGRMBIO TLESOUDVP OJFQYTXN"
        );

        assert_eq!(
            transposition(
                false,
                "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ.,/;'[]{}:|_+=-`~() ",
                "Tenacious Drink"
            ),
            "DWOCXLGZSKI VNBUPDYRJHN FTOCVQJBZEW KFYMHASQMEX LGUPIATR"
        );

        assert_eq!(
            transposition(false, "WE ARE DISCOVERED. FLEE AT ONCE.", "ZEBRAS STRIPE"),
            "CAEEN SOIAE DRLEF WEDRE EVTOC"
        );
    }

    #[test]
    fn double_decryption() {
        assert_eq!(
            transposition(
                true,
                "KEZEUWHAH ORCGRMBIO TLESOUDVP OJFQYTXN",
                "Archive Snow"
            ),
            "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG"
        );

        assert_eq!(
            transposition(
                true,
                "DWOCXLGZSKI VNBUPDYRJHN FTOCVQJBZEW KFYMHASQMEX LGUPIATR",
                "Tenacious Drink",
            ),
            "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ"
        );

        assert_eq!(
            transposition(true, "CAEEN SOIAE DRLEF WEDRE EVTOC", "ZEBRAS STRIPE"),
            "WEAREDISCOVEREDFLEEATONCE"
        );
    }
}
