//! Hill Cipher
//!
//! The Hill Cipher is a polygraphic substitution cipher based on linear algebra.
//!
//! # Algorithm
//!
//! Let the order of the encryption key be N (as it is a square matrix).
//! The text is divided into batches of length N and converted to numerical vectors
//! by a simple mapping starting with A=0 and so on.
//!
//! The key matrix is multiplied with the batch vector to obtain the encoded vector.
//! After multiplication, modular 36 calculations map results to alphanumerics.
//!
//! For decryption, the modular inverse of the encryption key is computed and used
//! with the same process to recover the original message.
//!
//! # Constraints
//!
//! The determinant of the encryption key matrix must be coprime with 36.
//!
//! # Note
//!
//! - Only alphanumeric characters are considered
//! - Text is padded to a multiple of the key size using the last character
//! - Decrypted text may have padding characters at the end
//!
//! # References
//!
//! - <https://apprendre-en-ligne.net/crypto/hill/Hillciph.pdf>
//! - <https://www.youtube.com/watch?v=kfmNeskzs2o>

const KEY_STRING: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
const MODULUS: i32 = 36;

/// Hill Cipher implementation
pub struct HillCipher {
    encrypt_key: Vec<Vec<i32>>,
    break_key: usize,
}

impl HillCipher {
    /// Creates a new Hill Cipher with the given encryption key matrix.
    ///
    /// # Arguments
    ///
    /// * `encrypt_key` - An NxN square matrix
    ///
    /// # Returns
    ///
    /// `Err` if the matrix is invalid or determinant is not coprime with 36
    pub fn new(mut encrypt_key: Vec<Vec<i32>>) -> Result<Self, String> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}

    fn replace_letter(&self, letter: char) -> Option<usize> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (replace_letter)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement replace_letter");
}

    fn replace_digit(&self, num: i32) -> char {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (replace_digit)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement replace_digit");
}

    fn determinant(matrix: &[Vec<i32>]) -> i32 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (determinant)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement determinant");
}

    fn get_minor(matrix: &[Vec<i32>], row: usize, col: usize) -> Vec<Vec<i32>> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (get_minor)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement get_minor");
}

    fn cofactor_matrix(matrix: &[Vec<i32>]) -> Vec<Vec<i32>> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (cofactor_matrix)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement cofactor_matrix");
}

    fn transpose(matrix: &[Vec<i32>]) -> Vec<Vec<i32>> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (transpose)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement transpose");
}

    fn mod_inverse(a: i32, m: i32) -> Option<i32> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (mod_inverse)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement mod_inverse");
}

    fn gcd(mut a: i32, mut b: i32) -> i32 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (gcd)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement gcd");
}

    fn check_determinant(&self) -> Result<(), String> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (check_determinant)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement check_determinant");
}

    fn process_text(&self, text: &str) -> String {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (process_text)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement process_text");
}

    /// Encrypts the given text using the Hill cipher.
    pub fn encrypt(&self, text: &str) -> String {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (encrypt)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement encrypt");
}

    fn make_decrypt_key(&self) -> Vec<Vec<i32>> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (make_decrypt_key)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement make_decrypt_key");
}

    /// Decrypts the given text using the Hill cipher.
    pub fn decrypt(&self, text: &str) -> String {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (decrypt)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement decrypt");
}
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt() {
        let key = vec![vec![2, 5], vec![1, 6]];
        let cipher = HillCipher::new(key).unwrap();
        assert_eq!(cipher.encrypt("testing hill cipher"), "WHXYJOLM9C6XT085LL");
        assert_eq!(cipher.encrypt("hello"), "85FF00");
    }

    #[test]
    fn test_decrypt() {
        let key = vec![vec![2, 5], vec![1, 6]];
        let cipher = HillCipher::new(key).unwrap();
        assert_eq!(cipher.decrypt("WHXYJOLM9C6XT085LL"), "TESTINGHILLCIPHERR");
        assert_eq!(cipher.decrypt("85FF00"), "HELLOO");
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let key = vec![vec![2, 5], vec![1, 6]];
        let cipher = HillCipher::new(key).unwrap();
        let original = "HELLO WORLD";
        let encrypted = cipher.encrypt(original);
        let decrypted = cipher.decrypt(&encrypted);

        // Note: decrypted might have padding
        assert!(decrypted.starts_with("HELLOWORLD"));
    }

    #[test]
    fn test_process_text() {
        let key = vec![vec![2, 5], vec![1, 6]];
        let cipher = HillCipher::new(key).unwrap();
        assert_eq!(
            cipher.process_text("Testing Hill Cipher"),
            "TESTINGHILLCIPHERR"
        );
        assert_eq!(cipher.process_text("hello"), "HELLOO");
    }

    #[test]
    fn test_replace_letter() {
        let key = vec![vec![2, 5], vec![1, 6]];
        let cipher = HillCipher::new(key).unwrap();
        assert_eq!(cipher.replace_letter('T'), Some(19));
        assert_eq!(cipher.replace_letter('0'), Some(26));
        assert_eq!(cipher.replace_letter('A'), Some(0));
    }

    #[test]
    fn test_replace_digit() {
        let key = vec![vec![2, 5], vec![1, 6]];
        let cipher = HillCipher::new(key).unwrap();
        assert_eq!(cipher.replace_digit(19), 'T');
        assert_eq!(cipher.replace_digit(26), '0');
        assert_eq!(cipher.replace_digit(0), 'A');
    }

    #[test]
    fn test_invalid_determinant() {
        // Matrix with determinant not coprime with 36
        let key = vec![vec![2, 4], vec![1, 2]]; // det = 0
        assert!(HillCipher::new(key).is_err());
    }

    #[test]
    fn test_3x3_matrix() {
        // Matrix with determinant = 1 (coprime with 36)
        let key = vec![vec![1, 2, 3], vec![0, 1, 4], vec![5, 6, 0]];
        let cipher = HillCipher::new(key).unwrap();
        let encrypted = cipher.encrypt("ACT");
        let decrypted = cipher.decrypt(&encrypted);
        assert_eq!(decrypted, "ACT");
    }

    #[test]
    fn test_gcd() {
        assert_eq!(HillCipher::gcd(48, 18), 6);
        assert_eq!(HillCipher::gcd(7, 36), 1);
        assert_eq!(HillCipher::gcd(12, 36), 12);
    }

    #[test]
    fn test_mod_inverse() {
        assert_eq!(HillCipher::mod_inverse(7, 36), Some(31));
        assert_eq!(HillCipher::mod_inverse(1, 36), Some(1));
        assert_eq!(HillCipher::mod_inverse(2, 36), None); // 2 and 36 are not coprime
    }
}
